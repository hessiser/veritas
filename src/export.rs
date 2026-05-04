use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use directories::BaseDirs;
use chrono::DateTime;

use crate::battle::BattleContext;
use crate::kreide::types::RPG_GameCore_AbilityProperty;

#[derive(Clone, Debug, Serialize)]
pub struct ComprehensiveData {
    pub data_type: String,
    pub character_name: String,
    pub character_id: u32,
    pub total_damage: Option<f64>,
    pub damage_percentage: Option<f64>,
    pub dpav: Option<f64>,
    pub primary_skill_usage_count: Option<u32>,
    pub turns_taken: Option<u32>,
    pub average_damage_per_turn: Option<f64>,
    pub max_single_turn_damage: Option<f64>,
    pub first_turn_number: Option<u32>,
    pub last_turn_number: Option<u32>,
    pub turn_order: Option<u32>,
    pub turn_battle_id: Option<u32>,
    pub wave: Option<u32>,
    pub cycle: Option<u32>,
    pub action_value: Option<f64>,
    pub skill_name: Option<String>,
    pub skill_type: Option<String>,
    pub skill_type_name: Option<String>,
    pub skill_damage: Option<f64>,
    pub cumulative_damage: Option<f64>,
    pub cumulative_character_damage: Option<f64>,
    pub skill_damage_percentage: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExportAvatarBattleInfo {
    #[serde(rename = "avatarId")]
    pub avatar_id: u32,
    #[serde(rename = "isDie")]
    pub is_die: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExportTurnBattleInfo {
    #[serde(rename = "avatarId")]
    pub avatar_id: i32,
    #[serde(rename = "actionValue")]
    pub action_value: f64,
    #[serde(rename = "waveIndex")]
    pub wave_index: u32,
    #[serde(rename = "cycleIndex")]
    pub cycle_index: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExportDamageDetail {
    pub damage: f64,
    pub overkill_damage: f64,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExportSkillBattleInfo {
    #[serde(rename = "avatarId")]
    pub avatar_id: u32,
    #[serde(rename = "damageDetail")]
    pub damage_detail: Vec<ExportDamageDetail>,
    #[serde(rename = "totalDamage")]
    pub total_damage: f64,
    #[serde(rename = "skillType")]
    pub skill_type: String,
    #[serde(rename = "skillName")]
    pub skill_name: String,
    #[serde(rename = "turnBattleId")]
    pub turn_battle_id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExportAvatarDetail {
    pub id: u32,
    #[serde(rename = "isDie")]
    pub is_die: bool,
    #[serde(rename = "killer_uid")]
    pub killer_uid: i32,
    pub stats: HashMap<String, f64>,
    #[serde(rename = "statsHistory")]
    pub stats_history: Vec<ExportStatsHistory>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExportStatsHistory {
    pub stats: HashMap<String, f64>,
    #[serde(rename = "turnBattleId")]
    pub turn_battle_id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExportEnemyInfo {
    pub id: u32,
    pub name: String,
    #[serde(rename = "maxHP")]
    pub max_hp: f64,
    pub level: u32,
    #[serde(rename = "isDie")]
    pub is_die: bool,
    #[serde(rename = "positionIndex")]
    pub position_index: u32,
    #[serde(rename = "waveIndex")]
    pub wave_index: u32,
    #[serde(rename = "killer_uid")]
    pub killer_uid: i32,
    pub stats: HashMap<String, f64>,
    #[serde(rename = "statsHistory")]
    pub stats_history: Vec<ExportStatsHistory>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExportEnemyDetail {
    pub id: u32,
    #[serde(rename = "isDie")]
    pub is_die: bool,
    #[serde(rename = "killer_uid")]
    pub killer_uid: i32,
    #[serde(rename = "positionIndex")]
    pub position_index: u32,
    #[serde(rename = "waveIndex")]
    pub wave_index: u32,
    pub name: String,
    #[serde(rename = "maxHP")]
    pub max_hp: f64,
    pub level: u32,
    pub stats: HashMap<String, f64>,
    #[serde(rename = "statsHistory")]
    pub stats_history: Vec<ExportStatsHistory>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExportBattleData {
    pub lineup: Vec<ExportAvatarBattleInfo>,
    #[serde(rename = "turnHistory")]
    pub turn_history: Vec<ExportTurnBattleInfo>,
    #[serde(rename = "skillHistory")]
    pub skill_history: Vec<ExportSkillBattleInfo>,
    #[serde(rename = "dataAvatar")]
    pub data_avatar: Vec<serde_json::Value>,
    #[serde(rename = "totalAV")]
    pub total_av: f64,
    #[serde(rename = "totalDamage")]
    pub total_damage: f64,
    #[serde(rename = "damagePerAV")]
    pub damage_per_av: f64,
    #[serde(rename = "cycleIndex")]
    pub cycle_index: u32,
    #[serde(rename = "waveIndex")]
    pub wave_index: u32,
    #[serde(rename = "maxWave")]
    pub max_wave: u32,
    #[serde(rename = "maxCycle")]
    pub max_cycle: u32,
    pub version: String,
    #[serde(rename = "avatarDetail")]
    pub avatar_detail: HashMap<String, ExportAvatarDetail>,
    #[serde(rename = "enemyDetail")]
    pub enemy_detail: HashMap<String, ExportEnemyDetail>,
}

pub struct BattleDataExporter;

impl Default for BattleDataExporter {
    fn default() -> Self {
        Self
    }
}

impl BattleDataExporter {
    const INITIAL_TURN_ID: i32 = -1;
    const DEFAULT_KILLER_ID: i32 = -1;
    const INITIAL_TURN_BATTLE_ID: u32 = 0;

    pub fn new() -> Self {
        Self::default()
    }

    fn generate_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    fn build_stats_map(properties: &crate::models::misc::BattleStats) -> HashMap<String, f64> {
        let mut stats = HashMap::new();

        let tracked = [
            RPG_GameCore_AbilityProperty::CurrentHP,
            RPG_GameCore_AbilityProperty::MaxHP,
            RPG_GameCore_AbilityProperty::Attack,
            RPG_GameCore_AbilityProperty::Defence,
            RPG_GameCore_AbilityProperty::Speed,
            RPG_GameCore_AbilityProperty::ActionDelay,
        ];

        for prop in tracked {
            let key = prop.to_string();
            if let Some(value) = properties.get_value(&key) {
                stats.insert(key, value);
            }
        }

        stats
    }

    fn create_stats_history(stats: &HashMap<String, f64>) -> Vec<ExportStatsHistory> {
        if stats.is_empty() {
            Vec::new()
        } else {
            vec![ExportStatsHistory {
                stats: stats.clone(),
                turn_battle_id: Self::INITIAL_TURN_BATTLE_ID,
            }]
        }
    }

    fn calculate_damage_per_av(total_damage: f64, action_value: f64) -> f64 {
        if action_value > 0.0 {
            total_damage / action_value
        } else {
            0.0
        }
    }

    fn get_export_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
        Self::get_export_directory_with_custom_path(None, true)
    }
    
    pub fn get_export_directory_with_custom_path(
        custom_path: Option<&str>, 
        auto_create_date_folders: bool
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let base_path = if let Some(custom_path) = custom_path {
            PathBuf::from(custom_path)
        } else {
            if let Some(base_dirs) = BaseDirs::new() {
                base_dirs.data_local_dir()
                    .join(env!("CARGO_PKG_NAME"))
                    .join("battledata")
            } else {
                return Err("Could not determine local data directory".into());
            }
        };
        
        let export_dir = if auto_create_date_folders {
            let timestamp = Self::generate_timestamp();
            let date_folder = Self::format_date_from_timestamp(timestamp);
            base_path.join(date_folder)
        } else {
            base_path
        };
        
        std::fs::create_dir_all(&export_dir)?;
        Ok(export_dir)
    }

    fn format_date_from_timestamp(timestamp: u64) -> String {
        let datetime = DateTime::from_timestamp(timestamp as i64, 0)
            .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap());
        datetime.format("%Y-%m-%d").to_string()
    }

    pub fn get_export_directory_path() -> Result<String, Box<dyn std::error::Error>> {
        let dir = Self::get_export_directory()?;
        Ok(dir.to_string_lossy().to_string())
    }

    pub fn export_battle_data(&self, battle_context: &BattleContext) -> ExportBattleData {
        let lineup = battle_context
            .avatar_lineup
            .iter()
            .map(|avatar| ExportAvatarBattleInfo {
                avatar_id: avatar.id,
                is_die: false,
            })
            .collect();
        let mut turn_history = Vec::new();
        
        turn_history.push(ExportTurnBattleInfo {
            avatar_id: Self::INITIAL_TURN_ID,
            action_value: 0.0,
            wave_index: battle_context.wave,
            cycle_index: battle_context.max_cycle,
        });
        for (entity, action_value, wave, cycle) in &battle_context.entity_turn_history {
            turn_history.push(ExportTurnBattleInfo {
                avatar_id: entity.uid as i32,
                action_value: *action_value,
                wave_index: *wave,
                cycle_index: *cycle,
            });
        }

        let skill_history = battle_context
            .skill_history
            .iter()
            .map(|skill| ExportSkillBattleInfo {
                avatar_id: skill.avatar_id,
                damage_detail: skill
                    .damage_detail
                    .iter()
                    .map(|(damage, overkill_damage, damage_type)| ExportDamageDetail {
                        damage: *damage,
                        overkill_damage: *overkill_damage,
                        r#type: damage_type.to_string(),
                    })
                    .collect(),
                total_damage: skill.total_damage,
                skill_type: skill.skill_type.clone(),
                skill_name: skill.skill_name.clone(),
                turn_battle_id: skill.turn_battle_id,
            })
            .collect();

        let mut avatar_detail = HashMap::new();
        for avatar in &battle_context.avatar_lineup {
            let stats = battle_context
                .battle_avatars
                .iter()
                .find(|ba| ba.entity.uid == avatar.id)
                .map(|be| Self::build_stats_map(&be.properties))
                .unwrap_or_default();
            
            let stats_history = Self::create_stats_history(&stats);

            avatar_detail.insert(
                avatar.id.to_string(),
                ExportAvatarDetail {
                    id: avatar.id,
                    is_die: false,
                    killer_uid: Self::DEFAULT_KILLER_ID,
                    stats,
                    stats_history,
                },
            );
        }

        let mut enemy_detail = HashMap::new();
        for (index, enemy) in battle_context.enemies.iter().enumerate() {
            let stats = battle_context
                .battle_enemies
                .iter()
                .find(|be| be.entity.uid == enemy.uid)
                .map(|be| Self::build_stats_map(&be.properties))
                .unwrap_or_default();
            
            let stats_history = Self::create_stats_history(&stats);

            enemy_detail.insert(
                enemy.uid.to_string(),
                ExportEnemyDetail {
                    id: enemy.id,
                    is_die: false,
                    killer_uid: Self::DEFAULT_KILLER_ID,
                    position_index: index as u32,
                    wave_index: battle_context.wave,
                    name: enemy.name.clone(),
                    max_hp: enemy.base_stats.max_hp(),
                    level: enemy.base_stats.level(),
                    stats,
                    stats_history,
                },
            );
        }

        ExportBattleData {
            lineup,
            turn_history,
            skill_history,
            data_avatar: Vec::new(),
            total_av: battle_context.action_value,
            total_damage: battle_context.total_damage,
            damage_per_av: Self::calculate_damage_per_av(battle_context.total_damage, battle_context.action_value),
            cycle_index: battle_context.cycle,
            wave_index: battle_context.wave,
            max_wave: battle_context.max_waves,
            max_cycle: battle_context.max_cycle,
            version: env!("CARGO_PKG_VERSION").to_string(),
            avatar_detail,
            enemy_detail,
        }
    }

    pub fn export_to_file_with_custom_path(
        &self, 
        battle_context: &BattleContext, 
        filename: Option<String>,
        custom_path: Option<&str>,
        auto_create_date_folders: bool
    ) -> Result<String, Box<dyn std::error::Error>> {
        let export_data = self.export_battle_data(battle_context);
        let json = serde_json::to_string_pretty(&export_data)?;
        
        let export_dir = Self::get_export_directory_with_custom_path(custom_path, auto_create_date_folders)?;
        let filename = filename.unwrap_or_else(|| {
            format!("{}_{}/{}.json", env!("CARGO_PKG_NAME"), Self::generate_timestamp(), env!("CARGO_PKG_NAME"))
        });
        
        let full_path = export_dir.join(&filename);
        std::fs::write(&full_path, &json)?;
        Ok(full_path.to_string_lossy().to_string())
    }

    pub fn export_to_csv_with_custom_path(
        &self, 
        battle_context: &BattleContext, 
        filename: Option<String>,
        custom_path: Option<&str>,
        auto_create_date_folders: bool
    ) -> Result<String, Box<dyn std::error::Error>> {
        let export_dir = Self::get_export_directory_with_custom_path(custom_path, auto_create_date_folders)?;
        let filename = filename.unwrap_or_else(|| {
            format!("{}_{}/{}.csv", env!("CARGO_PKG_NAME"), Self::generate_timestamp(), env!("CARGO_PKG_NAME"))
        });
        
        let full_path = export_dir.join(&filename);
        let chart_data = self.generate_comprehensive_chart_data(battle_context);
        self.write_csv(&chart_data, &full_path.to_string_lossy())?;
        
        Ok(full_path.to_string_lossy().to_string())
    }

    pub fn generate_comprehensive_chart_data(&self, battle_context: &BattleContext) -> Vec<ComprehensiveData> {
        let mut all_data = Vec::new();
        let total_damage = battle_context.total_damage;
        let total_action_value = battle_context.action_value;

        let mut character_skills: HashMap<u32, HashMap<String, (u32, f64)>> = HashMap::new();
        for skill in &battle_context.skill_history {
            let char_skills = character_skills.entry(skill.avatar_id).or_default();
            let skill_entry = char_skills.entry(skill.skill_name.clone()).or_insert((0, 0.0));
            skill_entry.0 += 1;
            skill_entry.1 += skill.total_damage;
        }

        let mut character_turn_stats: HashMap<u32, (Vec<u32>, Vec<f64>)> = HashMap::new();
        for (turn_idx, turn_data) in battle_context.turn_history.iter().enumerate() {
            for (avatar_idx, avatar) in battle_context.avatar_lineup.iter().enumerate() {
                let turn_damage = turn_data.avatars_turn_damage.get(avatar_idx).copied().unwrap_or(0.0);
                if turn_damage > 0.0 {
                    let stats = character_turn_stats.entry(avatar.id).or_insert((Vec::new(), Vec::new()));
                    stats.0.push((turn_idx + 1) as u32);
                    stats.1.push(turn_damage);
                }
            }
        }
        for (i, avatar) in battle_context.avatar_lineup.iter().enumerate() {
            let character_damage = battle_context.real_time_damages.get(i).copied().unwrap_or(0.0);
            
            let damage_percentage = if total_damage > 0.0 {
                (character_damage / total_damage) * 100.0
            } else {
                0.0
            };
            
            let dpav = if total_action_value > 0.0 {
                character_damage / total_action_value
            } else {
                0.0
            };

            let primary_skill_usage = character_skills
                .get(&avatar.id)
                .and_then(|skills| {
                    skills.iter()
                        .max_by_key(|(_, (usage_count, _))| *usage_count)
                        .map(|(_, (usage, _))| *usage)
                })
                .unwrap_or(0);

            let (turn_numbers, turn_damages) = character_turn_stats
                .get(&avatar.id)
                .cloned()
                .unwrap_or((Vec::new(), Vec::new()));

            let turns_taken = turn_numbers.len() as u32;
            let average_damage_per_turn = if turns_taken > 0 {
                character_damage / turns_taken as f64
            } else {
                0.0
            };
            let max_single_turn_damage = turn_damages.iter().copied().fold(0.0, f64::max);
            let first_turn_number = turn_numbers.first().copied().unwrap_or(0);
            let last_turn_number = turn_numbers.last().copied().unwrap_or(0);

            all_data.push(ComprehensiveData {
                data_type: "character_summary".to_string(),
                character_name: avatar.name.clone(),
                character_id: avatar.id,
                total_damage: Some(character_damage),
                damage_percentage: Some(damage_percentage),
                dpav: Some(dpav),
                primary_skill_usage_count: Some(primary_skill_usage),
                turns_taken: Some(turns_taken),
                average_damage_per_turn: Some(average_damage_per_turn),
                max_single_turn_damage: Some(max_single_turn_damage),
                first_turn_number: Some(first_turn_number),
                last_turn_number: Some(last_turn_number),
                turn_order: None,
                turn_battle_id: None,
                wave: None,
                cycle: None,
                action_value: None,
                skill_name: None,
                skill_type: None,
                skill_type_name: None,
                skill_damage: None,
                cumulative_damage: None,
                cumulative_character_damage: None,
                skill_damage_percentage: None,
            });
        }

        let mut cumulative_total_damage = 0.0;
        let mut cumulative_character_damage: HashMap<u32, f64> = HashMap::new();

        for (turn_order, skill) in battle_context.skill_history.iter().enumerate() {
            cumulative_total_damage += skill.total_damage;
            
            let char_cumulative = cumulative_character_damage
                .entry(skill.avatar_id)
                .or_insert(0.0);
            *char_cumulative += skill.total_damage;

            let turn_info = battle_context.entity_turn_history
                .get(skill.turn_battle_id as usize)
                .map(|(_, av, wave, cycle)| (*av, *wave, *cycle))
                .unwrap_or((0.0, 1, 1));

            let character_name = battle_context.avatar_lineup
                .iter()
                .find(|avatar| avatar.id == skill.avatar_id)
                .map(|avatar| avatar.name.clone())
                .unwrap_or_else(|| format!("Avatar_{}", skill.avatar_id));

            all_data.push(ComprehensiveData {
                data_type: "skill_detail".to_string(),
                character_name,
                character_id: skill.avatar_id,
                total_damage: None,
                damage_percentage: None,
                dpav: None,
                primary_skill_usage_count: None,
                turns_taken: None,
                average_damage_per_turn: None,
                max_single_turn_damage: None,
                first_turn_number: None,
                last_turn_number: None,
                turn_order: Some((turn_order + 1) as u32),
                turn_battle_id: Some(skill.turn_battle_id),
                wave: Some(turn_info.1),
                cycle: Some(turn_info.2),
                action_value: Some(turn_info.0),
                skill_name: Some(skill.skill_name.clone()),
                skill_type: Some(skill.skill_type.clone()),
                skill_type_name: Some(skill.skill_type.clone()),
                skill_damage: Some(skill.total_damage),
                cumulative_damage: Some(cumulative_total_damage),
                cumulative_character_damage: Some(*char_cumulative),
                skill_damage_percentage: Some(if total_damage > 0.0 {
                    (skill.total_damage / total_damage) * 100.0
                } else {
                    0.0
                }),
            });
        }

        all_data
    }

    fn write_csv<T: Serialize>(&self, data: &[T], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_path(filename)?;
        
        for record in data {
            wtr.serialize(record)?;
        }
        
        wtr.flush()?;
        Ok(())
    }
}