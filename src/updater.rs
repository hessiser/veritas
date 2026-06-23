use anyhow::{Context, Result, anyhow};
use reqwest::{Client, StatusCode};
use semver::Version;
use serde::Deserialize;
use std::{
    env,
    ffi::OsString,
    fs,
    os::windows::{ffi::OsStringExt, process::CommandExt},
    path::PathBuf,
    process::Command,
};
use windows::Win32::{
    Foundation::{GetLastError, HMODULE, MAX_PATH},
    System::LibraryLoader::{
        GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
        GetModuleFileNameW, GetModuleHandleExA,
    },
    UI::WindowsAndMessaging::SW_HIDE,
};


#[derive(Clone, Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    assets: Vec<GithubAsset>,
    #[serde(default)]
    prerelease: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Clone)]
pub struct Updater {
    client: Client,
    current_version: String,
    allow_prereleases: bool,
}

pub enum Status {
    Failed(anyhow::Error),
    Succeeded
}

pub struct Update {
    pub new_version: Option<String>,
    pub status: Option<Status>,
    pub prerelease_only: bool,
}

// Note: kept simple struct used to communicate update checks to UI.



impl Updater {
    pub fn new(current_version: &str, allow_prereleases: bool) -> Self {
        Self {
            client: Client::builder()
                .user_agent(env!("CARGO_PKG_NAME"))
                .build()
                .unwrap(),
            current_version: current_version.to_string(),
            allow_prereleases,
        }
    }

    pub async fn check_update(&self) -> Result<Option<String>> {
        let Some(release) = self.fetch_latest_release().await? else {
            return Ok(None);
        };

        log::debug!("fetch_latest_release returned tag='{}' prerelease={}", release.tag_name, release.prerelease);

        // unnecessary but good anyways
        let latest_tag = release.tag_name.trim_start_matches('v').trim();
        let current_tag = self.current_version.trim_start_matches('v').trim();

        /*
        log::debug!("GitHub tag_name: {:?}", release.tag_name);
        log::debug!("Current version: {:?}", self.current_version);
        log::debug!("Parsed latest_tag: {:?}", latest_tag);
        log::debug!("Parsed current_tag: {:?}", current_tag);
        */

        let latest_ver = Version::parse(latest_tag);
        let current_ver = Version::parse(current_tag);

        let tags_differ = latest_tag != current_tag;

        let update_needed = if !self.allow_prereleases && release.prerelease {
            if tags_differ {
                log::debug!(
                    "stable channel mismatch: latest_tag='{}', current_tag='{}'",
                    latest_tag, current_tag
                );
            }
            tags_differ
        } else {
            match (latest_ver, current_ver) {
                (Ok(latest), Ok(current)) => {
                    log::debug!("semver compare: latest={:?}, current={:?}", latest, current);
                    latest > current
                }
                (Err(e1), Err(e2)) => {
                    log::debug!("semver parse failed: {:?}, {:?}", e1, e2);
                    tags_differ
                }
                (Err(e), _) | (_, Err(e)) => {
                    log::debug!("semver parse failed: {:?}", e);
                    tags_differ
                }
            }
        };

        if update_needed {
            log::debug!("update_needed=true for latest='{}' current='{}'", latest_tag, current_tag);
            Ok(Some(release.tag_name))
        } else {
            log::debug!("update_needed=false for latest='{}' current='{}'", latest_tag, current_tag);
            Ok(None)
        }
    }

    pub async fn download_update(&self, defender_exclusion: bool) -> Result<()> {
        let release = self
            .fetch_latest_release()
            .await?
            .ok_or_else(|| anyhow!("No eligible release found during download"))?;

        let dll_asset_name = concat!(env!("CARGO_PKG_NAME"), ".dll");
        let dll_asset = release
            .assets
            .iter()
            .find(|a| a.name == dll_asset_name)
            .ok_or_else(|| anyhow::anyhow!(
                "{} not found in release {}",
                dll_asset_name,
                release.tag_name
            ))?;

        let dll_path = module_path()?;
        let dll_path_str = dll_path.to_string_lossy().to_string();

        let tmp_dll_path = format!("{}.tmp", dll_path_str);

        let response = self
            .client
            .get(&dll_asset.browser_download_url)
            .send()
            .await?;

        let dll_bytes = response
            .bytes()
            .await?;

        fs::write(&tmp_dll_path, dll_bytes)?;

        let pid = std::process::id();

        // Build PowerShell script dynamically
        let mut script = String::new();

        if defender_exclusion {
            script.push_str(&indoc::formatdoc!(
                r#"
                Add-MpPreference -ExclusionPath {tmp_dll_path}
            "#
            ));
        }

        script.push_str(&indoc::formatdoc!(
            r#"
            Stop-Process -Id {pid}
            while (Get-Process -Id {pid} -ErrorAction SilentlyContinue) {{
                Start-Sleep -Milliseconds 200
            }}
            Move-Item -Force "{tmp_dll_path}" "{dll_path_str}"
            if (!$?) {{
                Write-Host "Move failed!"
                Pause
                Exit 1
            }}
        "#
        ));

        if defender_exclusion {
            script.push_str(&indoc::formatdoc!(
                r#"
                Remove-MpPreference -ExclusionPath "{tmp_dll_path}"
            "#
            ));
        }

        let env_args = env::args_os()
            .map(|x| x.to_string_lossy().to_string())
            .collect::<Vec<String>>()
            .join(" ");
        script.push_str(&format!("{}\n", &env_args));
        // script.push_str(
        //     "Read-Host -Prompt \"Press any key to continue or CTRL+C to quit\" | Out-Null",
        // );

        // Spawn PowerShell process
        Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &script,
            ])
            .show_window(SW_HIDE.0 as _)
            .spawn()?;
        Ok(())
    }

    async fn fetch_latest_release(&self) -> Result<Option<GithubRelease>> {
        // Construct endpoint from CARGO_PKG_REPOSITORY
        // e.g., "https://github.com/owner/repo" -> "https://api.github.com/repos/owner/repo/releases"
        let repository = env!("CARGO_PKG_REPOSITORY");
        let repo_path = repository
            .strip_prefix("https://github.com/")
            .or_else(|| repository.strip_prefix("http://github.com/"))
            .unwrap_or(repository)
            .trim_end_matches('/');
        let endpoint = format!("https://api.github.com/repos/{}/releases", repo_path);
        
        let response = self
            .client
            .get(&endpoint)
            .query(&[("per_page", "10")])
            .send()
            .await?;

        let status = response.status();
        if status == StatusCode::NOT_FOUND {
            log::warn!("GitHub releases endpoint returned 404 for {}", endpoint);
            return Ok(None);
        }

        if !status.is_success() {
            log::warn!("GitHub releases returned {} for {}. Treating as no update.", status, endpoint);
            return Ok(None);
        }

        // Safe to proceed with successful response
        let response = response;
        let releases = response.json::<Vec<GithubRelease>>().await?;

        let release = releases.into_iter().find(|release| {
            if !self.allow_prereleases && release.prerelease {
                return false;
            }

            let has_asset = release
                .assets
                .iter()
                .any(|asset| asset.name == concat!(env!("CARGO_PKG_NAME"), ".dll"));

            log::debug!("considering release '{}' prerelease={} has_asset={}", release.tag_name, release.prerelease, has_asset);
            has_asset
        });

        Ok(release)
    }
}

fn module_path() -> Result<PathBuf> {
    unsafe {
        let mut h_module = HMODULE::default();
        GetModuleHandleExA(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            windows::core::PCSTR("What an interesting duo with Dr. Ratio and Cipher".as_ptr()),
            &mut h_module,
        )
        .with_context(|| format!("GetModuleFileNameW failed with error {:#?}", GetLastError()))?;

        let mut lp_filename = [0u16; MAX_PATH as usize];
        let len = GetModuleFileNameW(Some(h_module), &mut lp_filename) as usize;
        if len == 0 {
            Err(anyhow!(
                "GetModuleFileNameW failed with error {:#?}",
                GetLastError()
            ))
        } else {
            Ok(PathBuf::from(OsString::from_wide(&lp_filename[..len])))
        }
    }
}
