use std::sync::{LazyLock, Mutex, MutexGuard};

use anyhow::{Context, Result};

use crate::{
    models::{
        events::*,
        misc::*,
        packets::Packet,
    },
    server,
};

#[derive(Clone, Debug)]
pub struct SkillHistoryEntry {
    pub avatar_id: u32,
    pub skill_name: String,
    pub skill_type: u32,
    pub total_damage: f64,
    pub damage_detail: Vec<(f64, isize)>,
    pub turn_battle_id: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DamageCategory {
    Basic,
    Skill,
    Ultimate,
    Qte,
    Dot,
    FollowUp,
    Technique,
    Additional,
    Break,
    Servant,
    TrueDamage,
    Elation,
    Other,
}

impl DamageCategory {
    pub const ALL: [Self; 13] = [
        Self::Basic,
        Self::Skill,
        Self::Ultimate,
        Self::Qte,
        Self::Dot,
        Self::FollowUp,
        Self::Technique,
        Self::Additional,
        Self::Break,
        Self::Servant,
        Self::TrueDamage,
        Self::Elation,
        Self::Other,
    ];

    const fn index(self) -> usize {
        match self {
            Self::Basic => 0,
            Self::Skill => 1,
            Self::Ultimate => 2,
            Self::Qte => 3,
            Self::Dot => 4,
            Self::FollowUp => 5,
            Self::Technique => 6,
            Self::Additional => 7,
            Self::Break => 8,
            Self::Servant => 9,
            Self::TrueDamage => 10,
            Self::Elation => 11,
            Self::Other => 12,
        }
    }

    pub fn from_attack_type(value: isize) -> Self {
        match value {
            1 => Self::Basic,
            2 => Self::Skill,
            3 => Self::Ultimate,
            4 => Self::Qte,
            5 => Self::Dot,
            6 => Self::Additional,
            7 | 8 => Self::Technique,
            9 => Self::FollowUp,
            10 | 11 => Self::Break,
            12 => Self::Servant,
            13 => Self::TrueDamage,
            14 => Self::Elation,
            _ => Self::Other,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Basic => "Basic",
            Self::Skill => "Skill",
            Self::Ultimate => "Ultimate",
            Self::Qte => "QTE",
            Self::Dot => "DoT",
            Self::FollowUp => "Follow-Up",
            Self::Technique => "Technique",
            Self::Additional => "Additional",
            Self::Break => "Break",
            Self::Servant => "Servant",
            Self::TrueDamage => "True Damage",
            Self::Elation => "Elation",
            Self::Other => "Other",
        }
    }
}

#[derive(Clone, Debug)]
pub struct DamageCategoryBreakdown {
    totals: [f64; DamageCategory::ALL.len()],
}

impl Default for DamageCategoryBreakdown {
    fn default() -> Self {
        Self {
            totals: [0.0; DamageCategory::ALL.len()],
        }
    }
}

impl DamageCategoryBreakdown {
    pub fn add(&mut self, category: DamageCategory, damage: f64) {
        self.totals[category.index()] += damage;
    }

    pub fn amount(&self, category: DamageCategory) -> f64 {
        self.totals[category.index()]
    }
}

#[derive(Clone, Copy)]
pub enum BattleState {
    Started,
    Ended,
}

// Data that aren't meant to be exposed in the API
// And is only for the overlay frontend
// pub struct BattleContextInternal {
//     pub relative_action_value: f64,
// }

#[derive(Default, Clone)]
pub struct BattleContext {
    pub state: Option<BattleState>,
    pub avatar_lineup: Vec<Avatar>,
    pub battle_avatars: Vec<BattleEntity>,
    pub enemies: Vec<Enemy>,
    pub enemy_lineup: Vec<Entity>,
    pub battle_enemies: Vec<BattleEntity>,
    pub turn_history: Vec<TurnInfo>,
    pub av_history: Vec<TurnInfo>,
    pub entity_turn_history: Vec<(Entity, f64, u32, u32)>,
    pub skill_history: Vec<SkillHistoryEntry>,
    pub current_turn_battle_id: u32,
    // This is really only relevant for MOC and 
    // is the relative AV
    pub last_wave_action_value: f64,
    pub action_value: f64,
    pub current_turn_info: TurnInfo,
    pub turn_count: usize,
    pub total_damage: f64,
    // Index w/ lineup index
    // Used to update UI damage when dmg occurs
    pub real_time_damages: Vec<f64>,
    // Index w/ lineup index, split by event attack type
    pub damage_by_category: Vec<DamageCategoryBreakdown>,
    // Index w/ lineup index
    // Used to update UI overkill damage when dmg occurs
    pub real_time_overkill_damages: Vec<f64>,
    pub max_waves: u32,
    pub wave: u32,
    pub cycle: u32,
    pub max_cycle: u32,
    pub stage_id: u32,
    pub battle_mode: BattleMode,

    // TODO: Move everything not meant to be exposed in the API here
    // pub internal: BattleContextInternal,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum BattleMode {
    MOC,
    PF,
    AS,
    #[default]
    Other,
}

static BATTLE_CONTEXT: LazyLock<Mutex<BattleContext>> =
    LazyLock::new(|| Mutex::new(BattleContext::default()));

static EXPORT_DATA_READY: LazyLock<Mutex<Option<crate::export::ExportBattleData>>> =
    LazyLock::new(|| Mutex::new(None));

static CSV_DATA_READY: LazyLock<Mutex<Option<Vec<crate::export::ComprehensiveData>>>> =
    LazyLock::new(|| Mutex::new(None));

impl BattleContext {
    pub fn get_instance() -> MutexGuard<'static, Self> {
        BATTLE_CONTEXT.lock().unwrap()
    }

    pub fn take_prepared_export_data() -> Option<crate::export::ExportBattleData> {
        EXPORT_DATA_READY.lock().ok()?.take()
    }

    pub fn take_prepared_csv_data() -> Option<Vec<crate::export::ComprehensiveData>> {
        CSV_DATA_READY.lock().ok()?.take()
    }

    fn find_lineup_index_by_avatar_id(
        battle_context: &MutexGuard<'static, Self>,
        avatar_id: u32,
    ) -> Option<usize> {
        let res = battle_context
            .avatar_lineup
            .iter()
            .enumerate()
            .find(|(_index, avatar)| avatar.id == avatar_id);
        res.map_or(None, |(index, _)| Some(index))
    }

    fn initialize_battle_context(battle_context: &mut MutexGuard<'static, Self>) {
        battle_context.current_turn_info = TurnInfo::default();
        battle_context.turn_history = Vec::new();
        battle_context.av_history = Vec::new();
        battle_context.entity_turn_history = Vec::new();
        battle_context.skill_history = Vec::new();
        battle_context.current_turn_battle_id = 0;

        battle_context.enemies = Vec::new();
        battle_context.battle_enemies = Vec::new();

        battle_context.turn_count = 0;
        battle_context.total_damage = 0.;
        battle_context.last_wave_action_value = 0.;
        battle_context.action_value = 0.;
        battle_context.damage_by_category = Vec::new();
        battle_context.max_waves = 0;
        battle_context.max_cycle = 0;
        battle_context.wave = 0;
        battle_context.cycle = 0;
        battle_context.stage_id = 0;
    }

    fn get_battle_mode(stage_id: u32) -> BattleMode {
        match stage_id {
            30010000..=31000000 => {
                match stage_id % 100 {
                    21 | 22 => BattleMode::MOC,
                    41 | 42 => BattleMode::PF,
                    _ => BattleMode::Other,
                }
            }
            420101..=420999 => BattleMode::AS,
            _ => BattleMode::Other,
        }
    }

    // A word of caution:
    // The lineup is setup first
    fn handle_on_battle_begin_event(
        e: OnBattleBeginEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        log::info!("Battle has started");
        log::info!("Max Waves: {}", e.max_waves);
        battle_context.max_waves = e.max_waves;

        battle_context.battle_mode = BattleContext::get_battle_mode(e.stage_id);

        Ok(Packet::OnBattleBegin {
            max_waves: e.max_waves,
            max_cycles: e.max_cycles,
            stage_id: e.stage_id,
        })
    }

    fn handle_on_set_lineup_event(
        e: OnSetLineupEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        battle_context.state = Some(BattleState::Started);
        Self::initialize_battle_context(&mut battle_context);
        battle_context.current_turn_info.avatars_turn_damage = vec![0f64; e.avatars.len()];
        battle_context.real_time_damages = vec![0f64; e.avatars.len()];
        battle_context.damage_by_category = vec![DamageCategoryBreakdown::default(); e.avatars.len()];
        battle_context.real_time_overkill_damages = vec![0f64; e.avatars.len()];
        battle_context.avatar_lineup = e.avatars;

        let mut battle_avatars = Vec::new();
        for avatar in &battle_context.avatar_lineup {
            battle_avatars.push(BattleEntity {
                entity: Entity {
                    uid: avatar.id,
                    team: Team::Player,
                },
                battle_stats: BattleStats::default(),
            });
        }
        battle_context.battle_avatars = battle_avatars;

        for avatar in &battle_context.avatar_lineup {
            log::info!("{} was loaded in lineup", avatar);
        }

        Ok(Packet::OnSetBattleLineup {
            avatars: battle_context.avatar_lineup.clone(),
        })
    }

    fn handle_on_damage_event(
        e: OnDamageEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        let lineup_index = Self::find_lineup_index_by_avatar_id(&battle_context, e.attacker.uid)
            .with_context(|| format!("Could not find avatar {} in lineup", e.attacker.uid))?;
        let turn = &mut battle_context.current_turn_info;
        // Record character damage chunk
        turn.avatars_turn_damage[lineup_index] += e.damage;
        battle_context.real_time_damages[lineup_index] += e.damage as f64;
        battle_context.damage_by_category[lineup_index]
            .add(DamageCategory::from_attack_type(e.damage_type), e.damage as f64);
        battle_context.real_time_overkill_damages[lineup_index] += e.overkill_damage as f64;
        battle_context.total_damage += e.damage as f64;

        if let Some(last_skill) = battle_context.skill_history.iter_mut().rev().find(|skill| skill.avatar_id == e.attacker.uid) {
            last_skill.damage_detail.push((e.damage as f64, e.damage_type as isize));
            last_skill.total_damage += e.damage as f64;
        }

        Ok(Packet::OnDamage {
            attacker: e.attacker,
            damage: e.damage,
            damage_type: e.damage_type,
        })
    }

    fn handle_on_turn_begin_event(
        e: OnTurnBeginEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        battle_context.action_value = e.action_value;
        battle_context.current_turn_info.action_value = e.action_value;

        battle_context.current_turn_battle_id += 1;

        if let Some(turn_owner) = &e.turn_owner {
            let wave = battle_context.wave;
            let cycle = battle_context.cycle;
            battle_context.entity_turn_history.push((
                turn_owner.clone(),
                e.action_value,
                wave,
                cycle,
            ));
        }

        log::info!("AV: {:.2}", e.action_value);

        Ok(Packet::OnTurnBegin {
            action_value: e.action_value,
            turn_owner: e.turn_owner,
        })
    }

    fn handle_on_turn_end_event(
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        battle_context.current_turn_info.wave = battle_context.wave;
        battle_context.current_turn_info.cycle = battle_context.cycle;

        let mut turn_info = battle_context.current_turn_info.clone();

        // Calculate net damages
        turn_info.total_damage = if turn_info.avatars_turn_damage.is_empty() {
            0.0
        } else {
            turn_info.avatars_turn_damage.iter().sum()
        };
        battle_context.turn_history.push(turn_info.clone());

        // If same AV, update damage
        if let Some(last_turn) = battle_context.av_history.last_mut() {
            if last_turn.action_value == turn_info.action_value {
                for (i, incoming_dmg) in turn_info.avatars_turn_damage.iter().enumerate() {
                    last_turn.avatars_turn_damage[i] += incoming_dmg;
                }
            } else {
                battle_context.av_history.push(turn_info.clone());
            }
        } else {
            battle_context.av_history.push(turn_info.clone());
        }

        // Logging
        for (i, avatar) in battle_context.avatar_lineup.iter().enumerate() {
            if turn_info.avatars_turn_damage[i] > 0.0 {
                log::info!(
                    "Turn Summary: {} has dealt {:.2} damage",
                    avatar,
                    turn_info.avatars_turn_damage[i]
                );
            }
        }

        if turn_info.total_damage > 0.0 {
            log::info!(
                "Turn Summary: Total damage of {:.2}",
                turn_info.total_damage
            );
        }

        // Restart turn info
        // battle_context.current_turn_info.total_damage = 0.0;
        battle_context.current_turn_info.avatars_turn_damage =
            vec![0f64; battle_context.avatar_lineup.len()];
        battle_context.turn_count += 1;

        Ok(Packet::OnTurnEnd { turn_info })
    }

    fn handle_on_entity_defeated_event(
        e: OnEntityDefeatedEvent,
        mut _battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        // log::info!("{} has defeated {}", e.attacker);

        Ok(Packet::OnEntityDefeated {
            killer: e.killer,
            entity_defeated: e.entity_defeated,
        })
    }

    fn handle_on_battle_end_event(
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        battle_context.state = Some(BattleState::Ended);
        
        let exporter = crate::export::BattleDataExporter::new();
        
        match std::panic::catch_unwind(|| {
            let export_data = exporter.export_battle_data(&battle_context);
            let csv_data = exporter.generate_comprehensive_chart_data(&battle_context);
            (export_data, csv_data)
        }) {
            Ok((export_data, csv_data)) => {
                if let Ok(mut export_storage) = EXPORT_DATA_READY.lock() {
                    *export_storage = Some(export_data);
                }
                if let Ok(mut csv_storage) = CSV_DATA_READY.lock() {
                    *csv_storage = Some(csv_data);
                }
                log::info!("Export data prepared successfully");
            }
            Err(e) => {
                log::error!("Failed to prepare export data: {:?}", e);
            }
        }

        Ok(Packet::OnBattleEnd {
            avatars: battle_context.avatar_lineup.clone(),
            turn_history: battle_context.turn_history.clone(),
            av_history: battle_context.av_history.clone(),
            turn_count: battle_context.turn_count,
            total_damage: battle_context.total_damage as f64,
            action_value: battle_context.action_value,
            cycle: battle_context.cycle,
            wave: battle_context.wave,
            stage_id: battle_context.stage_id,
        })
    }

    fn handle_on_use_skill_event(
        e: OnUseSkillEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        let turn_battle_id = battle_context.entity_turn_history.len() as u32;
        
        battle_context.skill_history.push(SkillHistoryEntry {
            avatar_id: e.avatar.uid,
            skill_name: e.skill.name.clone(),
            skill_type: e.skill.skill_type as u32,
            total_damage: 0.0,
            damage_detail: Vec::new(),
            turn_battle_id,
        });

        Ok(Packet::OnUseSkill {
            avatar: e.avatar,
            skill: e.skill,
        })
    }

    fn handle_on_update_wave_event(
        e: OnUpdateWaveEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        log::info!("Wave: {}", e.wave);

        if battle_context.battle_mode == BattleMode::MOC {
            battle_context.last_wave_action_value = battle_context.action_value;
        }

        battle_context.wave = e.wave;
        Ok(Packet::OnUpdateWave { wave: e.wave })
    }

    fn handle_on_update_cycle_event(
        e: OnUpdateCycleEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        log::info!("Cycle: {}", e.cycle);

        battle_context.cycle = e.cycle;
        if e.cycle > battle_context.max_cycle {
            battle_context.max_cycle = e.cycle;
        }
        Ok(Packet::OnUpdateCycle { cycle: e.cycle })
    }

    fn handle_on_stat_change_event(
        e: OnStatChangeEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        match e.entity.team {
            Team::Player => {
                if let Some(avatar) = battle_context
                    .battle_avatars
                    .iter_mut()
                    .find(|x| x.entity == e.entity)
                {
                    match e.stat {
                        crate::models::misc::Stat::HP(stat) => avatar.battle_stats.hp = stat,
                        crate::models::misc::Stat::Attack(stat) => avatar.battle_stats.attack = stat,
                        crate::models::misc::Stat::Defense(stat) => {
                            avatar.battle_stats.defense = stat
                        }
                        crate::models::misc::Stat::Speed(stat) => avatar.battle_stats.speed = stat,
                        crate::models::misc::Stat::AV(stat) => avatar.battle_stats.av = stat,
                        _ => {}
                    }
                }
            },
            Team::Enemy => {
                if let Some(enemy) = battle_context
                    .battle_enemies
                    .iter_mut()
                    .find(|x| x.entity == e.entity)
                {
                    match e.stat {
                        crate::models::misc::Stat::HP(stat) => enemy.battle_stats.hp = stat,
                        crate::models::misc::Stat::Attack(stat) => enemy.battle_stats.attack = stat,
                        crate::models::misc::Stat::Defense(stat) => {
                            enemy.battle_stats.defense = stat
                        }
                        crate::models::misc::Stat::Speed(stat) => enemy.battle_stats.speed = stat,
                        crate::models::misc::Stat::AV(stat) => enemy.battle_stats.av = stat,
                        _ => {}
                    }
                }
            }
        }

        Ok(Packet::OnStatChange {
            entity: e.entity,
            stat: e.stat,
        })
    }

    fn handle_on_initialize_enemy_event(
        e: OnInitializeEnemyEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        battle_context.enemies.push(e.enemy.clone());
        battle_context.battle_enemies.push(BattleEntity {
            entity: Entity {
                uid: e.enemy.uid,
                team: Team::Enemy,
            },
            battle_stats: BattleStats {
                hp: e.enemy.base_stats.hp,
                ..Default::default()
            },
        });
        Ok(Packet::OnInitializeEnemy { enemy: e.enemy })
    }

    fn handle_on_update_team_formation_event(
        e: OnUpdateTeamFormationEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        match e.team {
            Team::Player => {}
            Team::Enemy => {
                battle_context.enemy_lineup = e.entities.clone();
            }
        }
        Ok(Packet::OnUpdateTeamFormation {
            entities: e.entities,
            team: e.team,
        })
    }

    pub fn handle_event(event: Result<Event>) {
        let battle_context = Self::get_instance();
        let packet = match event {
            Result::Ok(event) => match event {
                Event::OnBattleBegin(e) => Self::handle_on_battle_begin_event(e, battle_context),
                Event::OnSetBattleLineup(e) => Self::handle_on_set_lineup_event(e, battle_context),
                Event::OnDamage(e) => Self::handle_on_damage_event(e, battle_context),
                Event::OnTurnBegin(e) => Self::handle_on_turn_begin_event(e, battle_context),
                Event::OnTurnEnd => Self::handle_on_turn_end_event(battle_context),
                Event::OnEntityDefeated(e) => {
                    Self::handle_on_entity_defeated_event(e, battle_context)
                }
                Event::OnBattleEnd => Self::handle_on_battle_end_event(battle_context),
                Event::OnUseSkill(e) => Self::handle_on_use_skill_event(e, battle_context),
                Event::OnUpdateWave(e) => Self::handle_on_update_wave_event(e, battle_context),
                Event::OnUpdateCycle(e) => {
                    if e.cycle == battle_context.cycle {
                        return;
                    }
                    Self::handle_on_update_cycle_event(e, battle_context)
                }
                Event::OnStatChange(e) => Self::handle_on_stat_change_event(e, battle_context),
                Event::OnInitializeEnemy(e) => {
                    Self::handle_on_initialize_enemy_event(e, battle_context)
                }
                Event::OnUpdateTeamFormation(e) => {
                    Self::handle_on_update_team_formation_event(e, battle_context)
                }
            },
            Err(e) => Ok({
                log::error!("{}", e);
                Packet::Error { msg: e.to_string() }
            }),
        };

        match packet {
            Result::Ok(packet) => {
                server::broadcast(packet);
            }
            Err(e) => log::error!("Packet Error: {}", e),
        };
    }
}