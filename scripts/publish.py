import subprocess
import toml
import os
import re
from tomlkit import parse, dumps

class Patch:
    def __init__(self, major, minor, revision):
        self.major = major
        self.minor = minor
        self.revision = revision

    def __str__(self):
        return f"{self.major}.{self.minor}.{self.revision}"

def update_cargo_toml(update: str):
    patch_re = re.compile(r"(\d+)\.(\d+)\.(\d+)")

    cargo_toml_path = "Cargo.toml"
    with open(cargo_toml_path, "r") as f:
        toml_str = f.read()

    doc = parse(toml_str)
    metadata = doc.get("package", {}).get("metadata")
    if metadata and "patch" in metadata:
        patch_str = metadata["patch"]
        match = patch_re.match(patch_str)
        if match:
            major_patch, minor_patch, revision_patch = map(int, match.groups())
            patch = Patch(major_patch, minor_patch, revision_patch)

            if update == "MAJOR_PATCH":
                patch.major += 1
                patch.minor = 0
                patch.revision = 1
                metadata["hotfix"] = "0.1"
            elif update == "MINOR_PATCH":
                patch.minor += 1
                patch.revision = 1
                metadata["hotfix"] = "0.1"
            elif update == "REVISION_PATCH":
                patch.revision += 1
                metadata["hotfix"] = "0.1"
            elif update == "HOTFIX_PATCH":
                hotfix = str(float(metadata.get("hotfix", "0.0")) + 0.1)
                metadata["hotfix"] = hotfix

            metadata["patch"] = str(patch)

        with open(cargo_toml_path, "w") as f:
            f.write(dumps(doc))

def build_with_cargo():
    print("Building the project with cargo...")
    subprocess.run(['cargo', 'build', '--release'], check=True)
    print("Build successful.")

def read_cargo_toml():
    try:
        with open('Cargo.toml', 'r') as file:
            config = toml.load(file)
            metadata = config.get('package', {}).get('metadata', {})
            return metadata
    except FileNotFoundError:
        print("Error: Cargo.toml not found.")
        exit(1)

def commit_and_tag(patch, region, is_beta, hotfix, is_nightly):
    commit_message = f"Update to {patch}"
    subprocess.run(['git', 'commit', '-am', commit_message], check=True)

    if is_beta:
        tag = f"{region}-beta"
    else:
        tag = f"{region}-prod"

    if is_nightly:
        tag = f"{tag}-nightly"

    tag = f"{tag}-{patch}-{hotfix}"

    subprocess.run(['git', 'tag', tag], check=True)
    print(f"Committed and tagged as {tag}")

    subprocess.run(['git', 'push', 'origin', 'HEAD'], check=True)
    subprocess.run(['git', 'push', 'origin', '--tags'], check=True)
    print("Pushed the commit and tags to the remote repository.")

def is_nightly_branch():
    result = subprocess.run(
        ['git', 'rev-parse', '--abbrev-ref', 'HEAD'],
        capture_output=True,
        text=True,
        check=True
    )
    branch_name = result.stdout.strip()
    return "nightly" in branch_name.lower()

def main():
    # Prompt for update type
    patch_map = {
        "major": "MAJOR_PATCH",
        "minor": "MINOR_PATCH",
        "rev": "REVISION_PATCH",
        "hotfix": "HOTFIX_PATCH"
    }

    while True:
        patch_input = input("What kind of update? (major/minor/rev/hotfix): ").strip().lower()
        if patch_input in patch_map:
            patch_type = patch_map[patch_input]
            break
        else:
            print("Invalid input. Please enter 'major', 'minor', 'rev', or 'hotfix'.")

    # Apply version patch and build
    update_cargo_toml(patch_type)
    build_with_cargo()

    metadata = read_cargo_toml()
    region = metadata.get('region')
    is_nightly = is_nightly_branch()
    is_beta = metadata.get('is_beta')
    patch = metadata.get('patch')
    hotfix = metadata.get('hotfix')

    commit_and_tag(patch, region, is_beta, hotfix, is_nightly)

if __name__ == "__main__":
    main()
