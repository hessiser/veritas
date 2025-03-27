use crate::sr::types::rpg::gamecore::TurnBasedGameMode;

use super::misc::Avatar;

pub enum Event {
    BattleBegin(BattleBeginEvent),
    SetBattleLineup(SetBattleLineupEvent),
    OnDamage(OnDamageEvent),
    TurnEnd,
    OnKill(OnKillEvent),
    BattleEnd,
}

pub struct BattleBeginEvent {
    pub turn_based_game_mode: *const TurnBasedGameMode,
}

pub struct SetBattleLineupEvent {
    pub avatars: Vec<Avatar>,
}

pub struct OnDamageEvent {
    pub attacker: Avatar,
    pub damage: f32,
}

pub struct OnKillEvent {
    pub attacker: Avatar,
}