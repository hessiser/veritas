use std::collections::HashMap;
use std::fmt;

use crate::kreide::types::RPG_GameCore_AbilityProperty;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Avatar {
    pub id: u32,
    pub name: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Enemy {
    pub id: u32,
    pub uid: u32,
    pub name: String,
    pub base_stats: BattleStats,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BattleEntity {
    pub entity: Entity,
    pub properties: BattleStats
}


#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct BattleStats {
    pub properties: HashMap<String, f64>,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entity {
    pub uid: u32,
    pub team: Team
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum Team {
    Player,
    Enemy
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Property {
    pub value: f64,
    pub kind: String,
}

impl BattleStats {
    pub fn set_property(&mut self, property: Property) {
        self.properties.insert(property.kind, property.value);
    }

    pub fn set_value<S: Into<String>>(&mut self, kind: S, value: f64) {
        self.properties.insert(kind.into(), value);
    }

    pub fn get_value(&self, kind: &str) -> Option<f64> {
        self.properties.get(kind).copied()
    }

    pub fn current_hp(&self) -> f64 {
        let key = RPG_GameCore_AbilityProperty::CurrentHP.to_string();
        self.get_value(&key).unwrap_or_default()
    }

    pub fn av(&self) -> f64 {
        let key = RPG_GameCore_AbilityProperty::ActionDelay.to_string();
        self.get_value(&key).unwrap_or_default()
    }

    pub fn max_hp(&self) -> f64 {
        let key = RPG_GameCore_AbilityProperty::MaxHP.to_string();
        self.get_value(&key).unwrap_or_default()
    }

    pub fn level(&self) -> u32 {
        self.get_value("Level").unwrap_or_default() as u32
    }
}

impl fmt::Display for Avatar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Skill {
    pub name: String,
    #[serde(rename = "type")]
    pub skill_type: String,
    pub skill_config_id: isize
}

impl fmt::Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.skill_type, self.name)
    }
}


#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct TurnInfo {
    pub action_value: f64,
    pub cycle: u32,
    pub wave: u32,
    pub avatars_turn_damage: Vec<f64>,
    pub total_damage: f64,
}

