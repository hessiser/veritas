use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Avatar {
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct TurnInfo {
    pub action_value: f32,
    pub avatars_damage_chunks: Vec<Vec<f32>>,
    pub avatars_damage: Vec<f32>,
    pub total_damage: f32,
    // pub turn_owner: Avatar
}
