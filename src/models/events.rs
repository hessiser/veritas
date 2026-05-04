
use super::types::{Avatar, Enemy, Entity, Skill, Property, Team};

pub enum Event {
    OnBattleBegin(OnBattleBeginEvent),
    OnSetBattleLineup(OnSetLineupEvent),
    OnDamage(OnDamageEvent),
    OnTurnBegin(OnTurnBeginEvent),
    OnTurnEnd,
    OnUseSkill(OnUseSkillEvent),
    OnBattleEnd,
    OnUpdateWave(OnUpdateWaveEvent),
    OnUpdateCycle(OnUpdateCycleEvent),
    OnStatChange(OnStatChangeEvent),
    OnEntityDefeated(OnEntityDefeatedEvent),
    OnUpdateTeamFormation(OnUpdateTeamFormationEvent),
    OnInitializeEnemy(OnInitializeEnemyEvent)
}

pub struct OnBattleBeginEvent {
    pub max_waves: u32,
    pub max_cycles: u32,
    pub stage_id: u32
}

pub struct OnUpdateWaveEvent {
    pub wave: u32,
}

pub struct OnUpdateCycleEvent {
    pub cycle: u32,
}

pub struct OnTurnBeginEvent {
    pub action_value: f64,
    pub turn_owner: Option<Entity>
}

pub struct OnUseSkillEvent {
    pub avatar: Entity,
    pub skill: Skill
}

pub struct OnSetLineupEvent {
    pub avatars: Vec<Avatar>,
}

pub struct OnDamageEvent {
    pub attacker: Entity,
    pub damage: f64,
    pub overkill_damage: f64,
    pub r#type: String,
}

pub struct OnEntityDefeatedEvent {
    pub killer: Entity,
    pub entity_defeated: Entity
}

pub struct OnStatChangeEvent {
    pub entity: Entity,
    pub property: Property
}

pub struct OnUpdateTeamFormationEvent {
    pub entities: Vec<Entity>,
    pub team: Team
}

pub struct OnInitializeEnemyEvent {
    pub enemy: Enemy
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}
