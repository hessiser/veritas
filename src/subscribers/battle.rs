use crate::battle::BattleContext;
use crate::kreide::types::rpg::client::TextID;
// use crate::kreide::native_types::*;
use crate::kreide::types::rpg::gamecore::*;
use crate::kreide::types::*;
use crate::kreide::*;
// use crate::kreide::types::rpg::client::*;
use crate::kreide::functions::rpg::client::*;
use crate::kreide::functions::rpg::gamecore::*;
use crate::kreide::helpers::*;
use crate::kreide::types::rpg::client::BattleAssetPreload;

use crate::GAMEASSEMBLY_HANDLE;
use crate::models::events::*;
use crate::models::misc::Avatar;
use crate::models::misc::Enemy;
use crate::models::misc::Entity;
use crate::models::misc::Stat;
use crate::models::misc::Stats;
use crate::models::misc::Team;

use anyhow::Result;
use anyhow::{Error, anyhow};
use function_name::named;
use retour::static_detour;
use std::ffi::c_void;
use std::ptr::null;

#[named]
unsafe fn get_elapsed_av(game_mode: *const TurnBasedGameMode) -> f64 {
    log::debug!(function_name!());
    unsafe { fixpoint_to_raw(&(*game_mode).ElapsedActionDelay__BackingField) * 10f64 }
}

// Called on any instance of damage
#[named]
fn on_damage(
    task_context: *const c_void,
    damage_by_attack_property: *const c_void,
    nopbaaaggla: *const NOPBAAAGGLA,
    attacker_ability: *const TurnBasedAbilityComponent,
    defender_ability: *const TurnBasedAbilityComponent,
    attacker: *const GameEntity,
    defender: *const GameEntity,
    attacker_task_single_target: *const GameEntity,
    flag: bool,
    obkbghmgbne: *const c_void,
) -> bool {
    unsafe {
        log::debug!(function_name!());
        let res = ON_DAMAGE_Detour.call(
            task_context,
            damage_by_attack_property,
            nopbaaaggla,
            attacker_ability,
            defender_ability,
            attacker,
            defender,
            attacker_task_single_target,
            flag,
            obkbghmgbne,
        );

        let mut event: Option<Result<Event>> = None;
        match (*attacker)._Team {
            TeamType::TeamLight => {
                let damage = fixpoint_to_raw(&(*nopbaaaggla).JFKEEOMKMLI);
                let damage_type = (*nopbaaaggla).APDDLHNGGIM;
                let attack_owner = {
                    let attack_owner = AbilityStatic_GetActualOwner(attacker);
                    if attack_owner.is_null() {
                        attacker
                    } else {
                        attack_owner
                    }
                };

                match (*attack_owner)._EntityType {
                    EntityType::Avatar => {
                        let e = match helpers::get_avatar_from_entity(attack_owner) {
                            Ok(avatar) => Ok(Event::OnDamage(OnDamageEvent {
                                attacker: Entity {
                                    uid: avatar.id,
                                    team: Team::Player,
                                },
                                damage,
                                damage_type: damage_type as isize,
                            })),
                            Err(e) => {
                                log::error!("Avatar Event Error: {}", e);
                                Err(anyhow!("{} Avatar Event Error: {}", function_name!(), e))
                            }
                        };
                        event = Some(e);
                    }
                    EntityType::Servant => {
                        let e = match helpers::get_avatar_from_servant_entity(attack_owner) {
                            Ok(avatar) => Ok(Event::OnDamage(OnDamageEvent {
                                attacker: Entity {
                                    uid: avatar.id,
                                    team: Team::Player,
                                },
                                damage,
                                damage_type: damage_type as isize,
                            })),
                            Err(e) => {
                                log::error!("Servant Event Error: {}", e);
                                Err(anyhow!("{} Servant Event Error: {}", function_name!(), e))
                            }
                        };
                        event = Some(e);
                    }
                    EntityType::Snapshot => {
                        // Unsure if this is if only a servant died and inflicted a DOT
                        let character_data_comp = (*attacker_ability)._CharacterDataRef;
                        let summoner_entity = (*character_data_comp).Summoner;
                        let e = match helpers::get_avatar_from_entity(summoner_entity) {
                            Ok(avatar) => Ok(Event::OnDamage(OnDamageEvent {
                                attacker: Entity {
                                    uid: avatar.id,
                                    team: Team::Player,
                                },
                                damage,
                                damage_type: damage_type as isize,
                            })),
                            Err(e) => {
                                log::error!("Snapshot Event Error: {}", e);
                                Err(anyhow!("{} Snapshot Event Error: {}", function_name!(), e))
                            }
                        };
                        event = Some(e);
                    }
                    _ => log::warn!(
                        "Light entity type {} was not matched",
                        (*attacker)._EntityType as usize
                    ),
                }
            }
            _ => {}
        }
        if let Some(event) = event {
            if !defender_ability.is_null() {
                let hp = TurnBasedAbilityComponent_GetProperty(
                    defender_ability,
                    AbilityProperty::CurrentHP,
                );
                log::info!("Monster HP: {}", fixpoint_to_raw(&hp));
            }

            BattleContext::handle_event(event);
        }
        res
    }
}

// Called when a manual skill is used. Does not account for insert skills (out of turn automatic skills)
#[named]
fn on_use_skill(
    instance: *const SkillCharacterComponent,
    skill_index: i32,
    a3: *const c_void,
    a4: bool,
    skill_extra_use_param: i32,
) -> bool {
    log::debug!(function_name!());
    unsafe {
        let entity = ((*instance)._parent_object)._OwnerRef;
        let skill_owner = {
            let skill_owner = AbilityStatic_GetActualOwner(entity);
            if skill_owner.is_null() {
                entity
            } else {
                skill_owner
            }
        };

        let mut event: Option<Result<Event>> = None;
        match (*skill_owner)._Team {
            TeamType::TeamLight => {
                let skill_data = SkillCharacterComponent_GetSkillData(
                    instance,
                    skill_index,
                    skill_extra_use_param,
                );


                if !skill_data.is_null() {
                    match (*skill_owner)._EntityType {
                        EntityType::Avatar => {
                            let e = match get_avatar_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_entity(skill_owner) {
                                    Ok(avatar) => {
                                        if skill.name.is_empty() {
                                            return ON_USE_SKILL_Detour.call(
                                                instance,
                                                skill_index,
                                                a3,
                                                a4,
                                                skill_extra_use_param,
                                            );
                                        }
                                        Ok(Event::OnUseSkill(OnUseSkillEvent {
                                            avatar: Entity {
                                                uid: avatar.id,
                                                team: Team::Player,
                                            },
                                            skill,
                                        }))
                                    }
                                    Err(e) => {
                                        log::error!("Avatar Event Error: {}", e);

                                        Err(anyhow!(
                                            "{} Avatar Event Error: {}",
                                            function_name!(),
                                            e
                                        ))
                                    }
                                },
                                Err(e) => {
                                    log::error!("Avatar Event Error: {}", e);
                                    Err(anyhow!(
                                        "{} Avatar Skill Event Error: {}",
                                        function_name!(),
                                        e
                                    ))
                                }
                            };
                            event = Some(e)
                        }
                        EntityType::Servant => {
                            let e = match get_servant_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_servant_entity(skill_owner) {
                                    Ok(avatar) => Ok(Event::OnUseSkill(OnUseSkillEvent {
                                        avatar: Entity {
                                            uid: avatar.id,
                                            team: Team::Player,
                                        },
                                        skill,
                                    })),
                                    Err(e) => {
                                        log::error!("Servant Event Error: {}", e);
                                        Err(anyhow!(
                                            "{} Servant Event Error: {}",
                                            function_name!(),
                                            e
                                        ))
                                    }
                                },
                                Err(e) => {
                                    log::error!("Servant Skill Error: {}", e);
                                    Err(anyhow!(
                                        "{} Servant Skill Event Error: {}",
                                        function_name!(),
                                        e
                                    ))
                                }
                            };
                            event = Some(e);
                        }
                        EntityType::BattleEvent => {
                            let battle_event_data_comp =
                                (*instance)._CharacterDataRef as *const BattleEventDataComponent;
                            let avatar_entity: *const GameEntity =
                                (*battle_event_data_comp).SourceCaster__BackingField;

                            let e = match get_battle_event_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_entity(avatar_entity) {
                                    Ok(avatar) => Ok(Event::OnUseSkill(OnUseSkillEvent {
                                        avatar: Entity {
                                            uid: avatar.id,
                                            team: Team::Player,
                                        },
                                        skill,
                                    })),
                                    Err(e) => {
                                        log::error!("Summon Event Error: {}", e);
                                        Err(anyhow!(
                                            "{} Summon Event Error: {}",
                                            function_name!(),
                                            e
                                        ))
                                    }
                                },
                                Err(e) => {
                                    log::error!("Summon Skill Event Error: {}", e);
                                    Err(anyhow!(
                                        "{} Summon Skill Event Error: {}",
                                        function_name!(),
                                        e
                                    ))
                                }
                            };
                            event = Some(e);
                        }
                        _ => log::warn!(
                            "Light entity type {} was not matched",
                            (*skill_owner)._EntityType as usize
                        ),
                    }
                }
            }
            _ => {}
        }
        if let Some(event) = event {
            BattleContext::handle_event(event);
        }
    }

    ON_USE_SKILL_Detour.call(instance, skill_index, a3, a4, skill_extra_use_param)
}

// Insert skills are out of turn automatic skills
#[named]
fn on_combo(instance: *const MMNDIEBMDNL) {
    log::debug!(function_name!());

    ON_COMBO_Detour.call(instance);
    unsafe {
        let turn_based_ability_component = (*instance).FIMNOPAAFEP;
        let skill_character_component = (*instance).HECCDOHIAFD;
        let entity = (*skill_character_component)._parent_object._OwnerRef;
        let skill_owner = {
            let skill_owner = AbilityStatic_GetActualOwner(entity);
            if skill_owner.is_null() {
                entity
            } else {
                skill_owner
            }
        };

        let mut event: Option<Result<Event>> = None;
        match (*skill_owner)._Team {
            TeamType::TeamLight => {
                let ability_name = ((*(instance)).HMCDHMFHABF).FKHHOBBFMEH;

                let skill_name = TurnBasedAbilityComponent_GetAbilityMappedSkill(
                    turn_based_ability_component,
                    ability_name,
                );

                let character_data_ref = (*turn_based_ability_component)._CharacterDataRef;
                let character_config = (*character_data_ref).JsonConfig__BackingField;
                let skill_index =
                    CharacterConfig_GetSkillIndexByTriggerKey(character_config, skill_name);
                let skill_data = SkillCharacterComponent_GetSkillData(
                    skill_character_component,
                    skill_index,
                    -1,
                );

                if !skill_data.is_null() {
                    match (*skill_owner)._EntityType {
                        EntityType::Avatar => {
                            let e: std::result::Result<Event, anyhow::Error> =
                                match get_avatar_skill_from_skilldata(skill_data) {
                                    Ok(skill) => match get_avatar_from_entity(skill_owner) {
                                        Ok(avatar) => {
                                            if skill.name.is_empty() {
                                                return;
                                            }
                                            Ok(Event::OnUseSkill(OnUseSkillEvent {
                                                avatar: Entity {
                                                    uid: avatar.id,
                                                    team: Team::Player,
                                                },
                                                skill,
                                            }))
                                        }
                                        Err(e) => {
                                            log::error!("Avatar Event Error: {}", e);
                                            Err(anyhow!(
                                                "{} Avatar Event Error: {}",
                                                function_name!(),
                                                e
                                            ))
                                        }
                                    },
                                    Err(e) => {
                                        log::error!("Avatar Combo Skill Event Error: {}", e);
                                        Err(anyhow!(
                                            "{} Avatar Combo Skill Event Error: {}",
                                            function_name!(),
                                            e
                                        ))
                                    }
                                };
                            event = Some(e);
                        }
                        EntityType::Servant => {
                            let e = match get_servant_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_servant_entity(skill_owner) {
                                    Ok(avatar) => Ok(Event::OnUseSkill(OnUseSkillEvent {
                                        avatar: Entity {
                                            uid: avatar.id,
                                            team: Team::Player,
                                        },
                                        skill,
                                    })),
                                    Err(e) => {
                                        log::error!("Servant Event Error: {}", e);
                                        Err(anyhow!(
                                            "{} Servant Event Error: {}",
                                            function_name!(),
                                            e
                                        ))
                                    }
                                },
                                Err(e) => {
                                    log::error!("Servant Skill Event Error: {}", e);
                                    Err(anyhow!(
                                        "{} Servant Skill Event Error: {}",
                                        function_name!(),
                                        e
                                    ))
                                }
                            };
                            event = Some(e);
                        }
                        EntityType::BattleEvent => {
                            let battle_event_data_comp = (*skill_character_component)
                                ._CharacterDataRef
                                as *const BattleEventDataComponent;
                            let avatar_entity: *const GameEntity =
                                (*battle_event_data_comp).SourceCaster__BackingField;

                            let e = match get_battle_event_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_entity(avatar_entity) {
                                    Ok(avatar) => Ok(Event::OnUseSkill(OnUseSkillEvent {
                                        avatar: Entity {
                                            uid: avatar.id,
                                            team: Team::Player,
                                        },
                                        skill,
                                    })),
                                    Err(e) => {
                                        log::error!("Summon Event Error: {}", e);
                                        Err(anyhow!(
                                            "{} Summon Event Error: {}",
                                            function_name!(),
                                            e
                                        ))
                                    }
                                },
                                Err(e) => {
                                    log::error!("Summon Skill Error: {}", e);
                                    Err(anyhow!(
                                        "{} Summon Skill Event Error: {}",
                                        function_name!(),
                                        e
                                    ))
                                }
                            };
                            event = Some(e);
                        }
                        _ => log::warn!(
                            "Light entity type {} was not matched",
                            (*skill_owner)._EntityType as usize
                        ),
                    }
                }
            }
            _ => {}
        }
        if let Some(event) = event {
            BattleContext::handle_event(event);
        }
    }
}

#[named]
fn on_set_lineup(instance: *const BattleAssetPreload, a1: bool, a2: *const c_void) {
    log::debug!(function_name!());
    unsafe {
        let battle_lineup_data = (*instance)._LineupData;

        let light_team = (*battle_lineup_data).LightTeam;
        let mut avatars = Vec::<Avatar>::new();
        let mut errors = Vec::<Error>::new();
        for character_ptr in (*light_team).to_slice() {
            let character = *character_ptr;
            let avatar_id = (*character).CharacterID;
            match helpers::get_avatar_from_id(avatar_id) {
                Ok(avatar) => avatars.push(avatar),
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        // Unsure if you can have more than one support char
        let extra_team = (*battle_lineup_data).ExtraTeam;
        for character_ptr in (*extra_team).to_slice() {
            let character = *character_ptr;
            let avatar_id = (*character).CharacterID;
            match helpers::get_avatar_from_id(avatar_id) {
                Ok(avatar) => avatars.push(avatar),
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        let event = if !errors.is_empty() {
            let errors = errors
                .iter()
                .map(|e| format!("{}\n", e.to_string()))
                .collect::<String>();
            Err(anyhow!(errors))
        } else {
            Ok(Event::OnSetBattleLineup(OnSetLineupEvent { avatars }))
        };
        BattleContext::handle_event(event);
    }
    ON_SET_LINEUP_Detour.call(instance, a1, a2)
}

#[named]
fn on_battle_begin(instance: *const TurnBasedGameMode) {
    log::debug!(function_name!());
    let res = ON_BATTLE_BEGIN_Detour.call(instance);
    unsafe {
        BattleContext::handle_event(Ok(Event::OnBattleBegin(OnBattleBeginEvent {
            max_waves: (*instance).WaveMonsterMaxCount__BackingField as _,
            max_cycles: (*instance).ChallengeTurnLimit__BackingField,
            stage_id: (*instance).CurrentWaveStageID__BackingField,
        })));
    }
    res
}

#[named]
fn on_battle_end(instance: *const TurnBasedGameMode) {
    log::debug!(function_name!());
    let res = ON_BATTLE_END_Detour.call(instance);
    BattleContext::handle_event(Ok(Event::OnBattleEnd));
    res
}

#[named]
fn on_turn_begin(instance: *const TurnBasedGameMode) {
    log::debug!(function_name!());
    // Update AV first
    let res = ON_TURN_BEGIN_Detour.call(instance);

    unsafe {
        let turn_owner = (*instance)._CurrentTurnActionEntity;
        match (*turn_owner)._EntityType {
            EntityType::Avatar => {
                let e = match helpers::get_avatar_from_entity(turn_owner) {
                    Ok(avatar) => Ok(Event::OnTurnBegin(OnTurnBeginEvent {
                        action_value: get_elapsed_av(instance),
                        turn_owner: Some(Entity {
                            uid: avatar.id,
                            team: Team::Player,
                        }),
                    })),
                    Err(e) => {
                        log::error!("Avatar Event Error: {}", e);
                        Err(anyhow!("{} Avatar Event Error: {}", function_name!(), e))
                    }
                };

                BattleContext::handle_event(e);
            }
            EntityType::Monster => {
                let e = Ok(Event::OnTurnBegin(OnTurnBeginEvent {
                    action_value: get_elapsed_av(instance),
                    turn_owner: Some(Entity {
                        uid: (*turn_owner).RuntimeID__BackingField,
                        team: Team::Enemy,
                    }),
                }));

                BattleContext::handle_event(e);
            }
            _ => {
                BattleContext::handle_event(Ok(Event::OnTurnBegin(OnTurnBeginEvent {
                    action_value: get_elapsed_av(instance),
                    turn_owner: None,
                })));
            }
        }
    }
    res
}

#[named]
fn on_turn_end(instance: *const TurnBasedAbilityComponent, a1: i32) {
    log::debug!(function_name!());
    // Can match player v enemy turn w/
    // RPG.GameCore.TurnBasedGameMode.GetCurrentTurnTeam
    BattleContext::handle_event(Ok(Event::OnTurnEnd));
    ON_TURN_END_Detour.call(instance, a1)
}

#[named]
pub fn on_update_wave(instance: *const TurnBasedGameMode) {
    let res = ON_UPDATE_WAVE_Detour.call(instance);
    unsafe {
        BattleContext::handle_event(Ok(Event::OnUpdateWave(OnUpdateWaveEvent {
            wave: (*instance)._WaveMonsterCurrentCount as _,
        })));
    }
    res
}

#[named]
pub fn on_update_cycle(instance: *const TurnBasedGameMode) -> u32 {
    log::debug!(function_name!());
    let cycle = ON_UPDATE_CYCLE_Detour.call(instance);
    BattleContext::handle_event(Ok(Event::OnUpdateCycle(OnUpdateCycleEvent { cycle })));
    cycle
}

#[named]
fn handle_hp_change(turn_based_ability_component: *const TurnBasedAbilityComponent) {
    log::debug!(function_name!());
    unsafe {
        let hp = fixpoint_to_raw(&TurnBasedAbilityComponent_GetProperty(
            turn_based_ability_component,
            AbilityProperty::CurrentHP,
        ));
        let entity = (*turn_based_ability_component)._parent_object._OwnerRef;

        match (*entity)._EntityType {
            EntityType::Avatar => {
                let e = match helpers::get_avatar_from_entity(entity) {
                    Ok(avatar) => Ok(Event::OnStatChange(OnStatChangeEvent {
                        entity: Entity {
                            uid: avatar.id,
                            team: Team::Player,
                        },
                        stat: Stat::HP(hp),
                    })),
                    Err(e) => {
                        log::error!("Avatar Event Error: {}", e);

                        Err(anyhow!("{} Avatar Event Error: {}", function_name!(), e))
                    }
                };
                BattleContext::handle_event(e);
            }
            EntityType::Monster => {
                BattleContext::handle_event(Ok(Event::OnStatChange(OnStatChangeEvent {
                    entity: Entity {
                        uid: (*entity).RuntimeID__BackingField,
                        team: Team::Enemy,
                    },
                    stat: Stat::HP(hp),
                })));
            }
            _ => {}
        }
    }
}

#[named]
pub fn on_direct_change_hp(
    instance: *const TurnBasedAbilityComponent,
    a1: i32,
    a2: FixPoint,
    a3: *const c_void,
) {
    log::debug!(function_name!());
    let res = ON_DIRECT_CHANGE_HP_Detour.call(instance, a1, a2, a3);
    handle_hp_change(instance);
    res
}

#[named]
pub fn on_direct_damage_hp(
    instance: *const TurnBasedAbilityComponent,
    a1: FixPoint,
    a2: i32,
    a3: *const c_void,
    a4: FixPoint,
    a5: *const c_void,
) {
    log::debug!(function_name!());
    let res = ON_DIRECT_DAMAGE_HP_Detour.call(instance, a1, a2, a3, a4, a5);
    handle_hp_change(instance);
    res
}

#[named]
pub fn on_stat_change(
    instance: *const TurnBasedAbilityComponent,
    property: AbilityProperty,
    a2: i32,
    new_stat: FixPoint,
    a4: *const c_void,
) {
    log::debug!(function_name!());
    let res = ON_STAT_CHANGE_Detour.call(instance, property, a2, new_stat, a4);
    unsafe {
        let entity = (*instance)._parent_object._OwnerRef;

        let stat = match property {
            AbilityProperty::MaxHP => Some(Stat::MaxHP(fixpoint_to_raw(&new_stat))),
            AbilityProperty::BaseHP => Some(Stat::BaseHP(fixpoint_to_raw(&new_stat))),
            AbilityProperty::HPAddedRatio => Some(Stat::HPAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::HPDelta => Some(Stat::HPDelta(fixpoint_to_raw(&new_stat))),
            AbilityProperty::HPConvert => Some(Stat::HPConvert(fixpoint_to_raw(&new_stat))),
            AbilityProperty::DirtyHPDelta => Some(Stat::DirtyHPDelta(fixpoint_to_raw(&new_stat))),
            AbilityProperty::DirtyHPRatio => Some(Stat::DirtyHPRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::RallyHP => Some(Stat::RallyHP(fixpoint_to_raw(&new_stat))),
            AbilityProperty::NegativeHP => Some(Stat::NegativeHP(fixpoint_to_raw(&new_stat))),
            AbilityProperty::CurrentHP => Some(Stat::HP(fixpoint_to_raw(&new_stat))),
            AbilityProperty::MaxSP => Some(Stat::MaxSP(fixpoint_to_raw(&new_stat))),
            AbilityProperty::CurrentSP => Some(Stat::CurrentSP(fixpoint_to_raw(&new_stat))),
            AbilityProperty::MaxSpecialSP => Some(Stat::MaxSpecialSP(fixpoint_to_raw(&new_stat))),
            AbilityProperty::CurrentSpecialSP => Some(Stat::CurrentSpecialSP(fixpoint_to_raw(&new_stat))),
            AbilityProperty::AdditionalBP => Some(Stat::AdditionalBP(fixpoint_to_raw(&new_stat))),
            AbilityProperty::Attack => Some(Stat::Attack(fixpoint_to_raw(&new_stat))),
            AbilityProperty::BaseAttack => Some(Stat::BaseAttack(fixpoint_to_raw(&new_stat))),
            AbilityProperty::AttackAddedRatio => Some(Stat::AttackAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::AttackDelta => Some(Stat::AttackDelta(fixpoint_to_raw(&new_stat))),
            AbilityProperty::AttackConvert => Some(Stat::AttackConvert(fixpoint_to_raw(&new_stat))),
            AbilityProperty::Defence => Some(Stat::Defense(fixpoint_to_raw(&new_stat))),
            AbilityProperty::BaseDefence => Some(Stat::BaseDefence(fixpoint_to_raw(&new_stat))),
            AbilityProperty::DefenceAddedRatio => Some(Stat::DefenceAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::DefenceDelta => Some(Stat::DefenceDelta(fixpoint_to_raw(&new_stat))),
            AbilityProperty::DefenceConvert => Some(Stat::DefenceConvert(fixpoint_to_raw(&new_stat))),
            AbilityProperty::DefenceOverride => Some(Stat::DefenceOverride(fixpoint_to_raw(&new_stat))),
            AbilityProperty::Level => Some(Stat::Level(fixpoint_to_raw(&new_stat))),
            AbilityProperty::Promotion => Some(Stat::Promotion(fixpoint_to_raw(&new_stat))),
            AbilityProperty::Rank => Some(Stat::Rank(fixpoint_to_raw(&new_stat))),
            AbilityProperty::Speed => Some(Stat::Speed(fixpoint_to_raw(&new_stat))),
            AbilityProperty::BaseSpeed => Some(Stat::BaseSpeed(fixpoint_to_raw(&new_stat))),
            AbilityProperty::SpeedAddedRatio => Some(Stat::SpeedAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::SpeedDelta => Some(Stat::SpeedDelta(fixpoint_to_raw(&new_stat))),
            AbilityProperty::SpeedConvert => Some(Stat::SpeedConvert(fixpoint_to_raw(&new_stat))),
            AbilityProperty::SpeedOverride => Some(Stat::SpeedOverride(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ActionDelay => Some(Stat::AV(fixpoint_to_raw(&new_stat) * 10.0)),
            AbilityProperty::ActionDelayAddedRatio => Some(Stat::ActionDelayAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ActionDelayAddAttenuation => Some(Stat::ActionDelayAddAttenuation(fixpoint_to_raw(&new_stat))),
            AbilityProperty::MaxStance => Some(Stat::MaxStance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::CurrentStance => Some(Stat::CurrentStance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::Level_AllDamageAddedRatio => Some(Stat::Level_AllDamageAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::AllDamageTypeAddedRatio => Some(Stat::AllDamageTypeAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::AllDamageReduce => Some(Stat::AllDamageReduce(fixpoint_to_raw(&new_stat))),
            AbilityProperty::DotDamageAddedRatio => Some(Stat::DotDamageAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::FatigueRatio => Some(Stat::FatigueRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::CriticalChance => Some(Stat::CriticalChance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::CriticalChanceBase => Some(Stat::CriticalChanceBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::CriticalChanceConvert => Some(Stat::CriticalChanceConvert(fixpoint_to_raw(&new_stat))),
            AbilityProperty::CriticalDamage => Some(Stat::CriticalDamage(fixpoint_to_raw(&new_stat))),
            AbilityProperty::CriticalDamageBase => Some(Stat::CriticalDamageBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::CriticalDamageConvert => Some(Stat::CriticalDamageConvert(fixpoint_to_raw(&new_stat))),
            AbilityProperty::CriticalResistance => Some(Stat::CriticalResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::PhysicalAddedRatio => Some(Stat::PhysicalAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::FireAddedRatio => Some(Stat::FireAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::IceAddedRatio => Some(Stat::IceAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ThunderAddedRatio => Some(Stat::ThunderAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::QuantumAddedRatio => Some(Stat::QuantumAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ImaginaryAddedRatio => Some(Stat::ImaginaryAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::WindAddedRatio => Some(Stat::WindAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::PhysicalResistance => Some(Stat::PhysicalResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::FireResistance => Some(Stat::FireResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::IceResistance => Some(Stat::IceResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ThunderResistance => Some(Stat::ThunderResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::QuantumResistance => Some(Stat::QuantumResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ImaginaryResistance => Some(Stat::ImaginaryResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::WindResistance => Some(Stat::WindResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::PhysicalResistanceBase => Some(Stat::PhysicalResistanceBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::FireResistanceBase => Some(Stat::FireResistanceBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::IceResistanceBase => Some(Stat::IceResistanceBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ThunderResistanceBase => Some(Stat::ThunderResistanceBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::QuantumResistanceBase => Some(Stat::QuantumResistanceBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ImaginaryResistanceBase => Some(Stat::ImaginaryResistanceBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::WindResistanceBase => Some(Stat::WindResistanceBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::PhysicalResistanceDelta => Some(Stat::PhysicalResistanceDelta(fixpoint_to_raw(&new_stat))),
            AbilityProperty::FireResistanceDelta => Some(Stat::FireResistanceDelta(fixpoint_to_raw(&new_stat))),
            AbilityProperty::IceResistanceDelta => Some(Stat::IceResistanceDelta(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ThunderResistanceDelta => Some(Stat::ThunderResistanceDelta(fixpoint_to_raw(&new_stat))),
            AbilityProperty::QuantumResistanceDelta => Some(Stat::QuantumResistanceDelta(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ImaginaryResistanceDelta => Some(Stat::ImaginaryResistanceDelta(fixpoint_to_raw(&new_stat))),
            AbilityProperty::WindResistanceDelta => Some(Stat::WindResistanceDelta(fixpoint_to_raw(&new_stat))),
            AbilityProperty::AllDamageTypeResistance => Some(Stat::AllDamageTypeResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::PhysicalPenetrate => Some(Stat::PhysicalPenetrate(fixpoint_to_raw(&new_stat))),
            AbilityProperty::FirePenetrate => Some(Stat::FirePenetrate(fixpoint_to_raw(&new_stat))),
            AbilityProperty::IcePenetrate => Some(Stat::IcePenetrate(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ThunderPenetrate => Some(Stat::ThunderPenetrate(fixpoint_to_raw(&new_stat))),
            AbilityProperty::QuantumPenetrate => Some(Stat::QuantumPenetrate(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ImaginaryPenetrate => Some(Stat::ImaginaryPenetrate(fixpoint_to_raw(&new_stat))),
            AbilityProperty::WindPenetrate => Some(Stat::WindPenetrate(fixpoint_to_raw(&new_stat))),
            AbilityProperty::AllDamageTypePenetrate => Some(Stat::AllDamageTypePenetrate(fixpoint_to_raw(&new_stat))),
            AbilityProperty::PhysicalTakenRatio => Some(Stat::PhysicalTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::FireTakenRatio => Some(Stat::FireTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::IceTakenRatio => Some(Stat::IceTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ThunderTakenRatio => Some(Stat::ThunderTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::QuantumTakenRatio => Some(Stat::QuantumTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ImaginaryTakenRatio => Some(Stat::ImaginaryTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::WindTakenRatio => Some(Stat::WindTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::AllDamageTypeTakenRatio => Some(Stat::AllDamageTypeTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::Monster_DamageTakenRatio => Some(Stat::Monster_DamageTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::PhysicalAbsorb => Some(Stat::PhysicalAbsorb(fixpoint_to_raw(&new_stat))),
            AbilityProperty::FireAbsorb => Some(Stat::FireAbsorb(fixpoint_to_raw(&new_stat))),
            AbilityProperty::IceAbsorb => Some(Stat::IceAbsorb(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ThunderAbsorb => Some(Stat::ThunderAbsorb(fixpoint_to_raw(&new_stat))),
            AbilityProperty::QuantumAbsorb => Some(Stat::QuantumAbsorb(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ImaginaryAbsorb => Some(Stat::ImaginaryAbsorb(fixpoint_to_raw(&new_stat))),
            AbilityProperty::WindAbsorb => Some(Stat::WindAbsorb(fixpoint_to_raw(&new_stat))),
            AbilityProperty::MinimumFatigueRatio => Some(Stat::MinimumFatigueRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ForceStanceBreakRatio => Some(Stat::ForceStanceBreakRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::StanceBreakAddedRatio => Some(Stat::StanceBreakAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::StanceBreakResistance => Some(Stat::StanceBreakResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::StanceBreakTakenRatio => Some(Stat::StanceBreakTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::PhysicalStanceBreakTakenRatio => Some(Stat::PhysicalStanceBreakTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::FireStanceBreakTakenRatio => Some(Stat::FireStanceBreakTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::IceStanceBreakTakenRatio => Some(Stat::IceStanceBreakTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ThunderStanceBreakTakenRatio => Some(Stat::ThunderStanceBreakTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::WindStanceBreakTakenRatio => Some(Stat::WindStanceBreakTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::QuantumStanceBreakTakenRatio => Some(Stat::QuantumStanceBreakTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ImaginaryStanceBreakTakenRatio => Some(Stat::ImaginaryStanceBreakTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::StanceWeakAddedRatio => Some(Stat::StanceWeakAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::StanceDefaultAddedRatio => Some(Stat::StanceDefaultAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::HealRatio => Some(Stat::HealRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::HealRatioBase => Some(Stat::HealRatioBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::HealRatioConvert => Some(Stat::HealRatioConvert(fixpoint_to_raw(&new_stat))),
            AbilityProperty::HealTakenRatio => Some(Stat::HealTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::Shield => Some(Stat::Shield(fixpoint_to_raw(&new_stat))),
            AbilityProperty::MaxShield => Some(Stat::MaxShield(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ShieldAddedRatio => Some(Stat::ShieldAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ShieldTakenRatio => Some(Stat::ShieldTakenRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::StatusProbability => Some(Stat::StatusProbability(fixpoint_to_raw(&new_stat))),
            AbilityProperty::StatusProbabilityBase => Some(Stat::StatusProbabilityBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::StatusProbabilityConvert => Some(Stat::StatusProbabilityConvert(fixpoint_to_raw(&new_stat))),
            AbilityProperty::StatusResistance => Some(Stat::StatusResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::StatusResistanceBase => Some(Stat::StatusResistanceBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::StatusResistanceConvert => Some(Stat::StatusResistanceConvert(fixpoint_to_raw(&new_stat))),
            AbilityProperty::SPRatio => Some(Stat::SPRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::SPRatioBase => Some(Stat::SPRatioBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::SPRatioConvert => Some(Stat::SPRatioConvert(fixpoint_to_raw(&new_stat))),
            AbilityProperty::SPRatioOverride => Some(Stat::SPRatioOverride(fixpoint_to_raw(&new_stat))),
            AbilityProperty::BreakDamageAddedRatio => Some(Stat::BreakDamageAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::BreakDamageAddedRatioBase => Some(Stat::BreakDamageAddedRatioBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::BreakDamageAddedRatioConvert => Some(Stat::BreakDamageAddedRatioConvert(fixpoint_to_raw(&new_stat))),
            AbilityProperty::BreakDamageExtraAddedRatio => Some(Stat::BreakDamageExtraAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::PhysicalStanceBreakResistance => Some(Stat::PhysicalStanceBreakResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::FireStanceBreakResistance => Some(Stat::FireStanceBreakResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::IceStanceBreakResistance => Some(Stat::IceStanceBreakResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ThunderStanceBreakResistance => Some(Stat::ThunderStanceBreakResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::WindStanceBreakResistance => Some(Stat::WindStanceBreakResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::QuantumStanceBreakResistance => Some(Stat::QuantumStanceBreakResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ImaginaryStanceBreakResistance => Some(Stat::ImaginaryStanceBreakResistance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::AggroBase => Some(Stat::AggroBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::AggroAddedRatio => Some(Stat::AggroAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::AggroDelta => Some(Stat::AggroDelta(fixpoint_to_raw(&new_stat))),
            AbilityProperty::RelicValueExtraAdditionRatio => Some(Stat::RelicValueExtraAdditionRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::EquipValueExtraAdditionRatio => Some(Stat::EquipValueExtraAdditionRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::EquipExtraRank => Some(Stat::EquipExtraRank(fixpoint_to_raw(&new_stat))),
            AbilityProperty::AvatarExtraRank => Some(Stat::AvatarExtraRank(fixpoint_to_raw(&new_stat))),
            AbilityProperty::Combo => Some(Stat::Combo(fixpoint_to_raw(&new_stat))),
            AbilityProperty::NormalBattleCount => Some(Stat::NormalBattleCount(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraAttackAddedRatio1 => Some(Stat::ExtraAttackAddedRatio1(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraAttackAddedRatio2 => Some(Stat::ExtraAttackAddedRatio2(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraAttackAddedRatio3 => Some(Stat::ExtraAttackAddedRatio3(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraAttackAddedRatio4 => Some(Stat::ExtraAttackAddedRatio4(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraDefenceAddedRatio1 => Some(Stat::ExtraDefenceAddedRatio1(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraDefenceAddedRatio2 => Some(Stat::ExtraDefenceAddedRatio2(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraDefenceAddedRatio3 => Some(Stat::ExtraDefenceAddedRatio3(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraDefenceAddedRatio4 => Some(Stat::ExtraDefenceAddedRatio4(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraHPAddedRatio1 => Some(Stat::ExtraHPAddedRatio1(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraHPAddedRatio2 => Some(Stat::ExtraHPAddedRatio2(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraHPAddedRatio3 => Some(Stat::ExtraHPAddedRatio3(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraHPAddedRatio4 => Some(Stat::ExtraHPAddedRatio4(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraHealAddedRatio => Some(Stat::ExtraHealAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraAllDamageTypeAddedRatio1 => Some(Stat::ExtraAllDamageTypeAddedRatio1(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraAllDamageTypeAddedRatio2 => Some(Stat::ExtraAllDamageTypeAddedRatio2(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraAllDamageTypeAddedRatio3 => Some(Stat::ExtraAllDamageTypeAddedRatio3(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraAllDamageTypeAddedRatio4 => Some(Stat::ExtraAllDamageTypeAddedRatio4(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraAllDamageReduce => Some(Stat::ExtraAllDamageReduce(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraShieldAddedRatio => Some(Stat::ExtraShieldAddedRatio(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraSpeedAddedRatio1 => Some(Stat::ExtraSpeedAddedRatio1(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraSpeedAddedRatio2 => Some(Stat::ExtraSpeedAddedRatio2(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraSpeedAddedRatio3 => Some(Stat::ExtraSpeedAddedRatio3(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraSpeedAddedRatio4 => Some(Stat::ExtraSpeedAddedRatio4(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraLuckChance => Some(Stat::ExtraLuckChance(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraLuckDamage => Some(Stat::ExtraLuckDamage(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraFrontPower => Some(Stat::ExtraFrontPower(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraFrontPowerBase => Some(Stat::ExtraFrontPowerBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraFrontPowerAddedRatio1 => Some(Stat::ExtraFrontPowerAddedRatio1(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraFrontPowerAddedRatio2 => Some(Stat::ExtraFrontPowerAddedRatio2(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraBackPower => Some(Stat::ExtraBackPower(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraBackPowerBase => Some(Stat::ExtraBackPowerBase(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraBackPowerAddedRatio1 => Some(Stat::ExtraBackPowerAddedRatio1(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraBackPowerAddedRatio2 => Some(Stat::ExtraBackPowerAddedRatio2(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraUltraDamageAddedRatio1 => Some(Stat::ExtraUltraDamageAddedRatio1(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraSkillDamageAddedRatio1 => Some(Stat::ExtraSkillDamageAddedRatio1(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraNormalDamageAddedRatio1 => Some(Stat::ExtraNormalDamageAddedRatio1(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraInsertDamageAddedRatio1 => Some(Stat::ExtraInsertDamageAddedRatio1(fixpoint_to_raw(&new_stat))),
            AbilityProperty::ExtraDOTDamageAddedRatio1 => Some(Stat::ExtraDOTDamageAddedRatio1(fixpoint_to_raw(&new_stat))),
            _ => None,
        };

        if let Some(stat) = stat {
            match (*entity)._EntityType {
                EntityType::Avatar => {
                    let e = match helpers::get_avatar_from_entity(entity) {
                        Ok(avatar) => Ok(Event::OnStatChange(OnStatChangeEvent {
                            entity: Entity {
                                uid: avatar.id,
                                team: Team::Player,
                            },
                            stat,
                        })),
                        Err(e) => {
                            log::error!("Avatar Event Error: {}", e);

                            Err(anyhow!("{} Avatar Event Error: {}", function_name!(), e))
                        }
                    };
                    BattleContext::handle_event(e);
                }
                EntityType::Monster => {
                    BattleContext::handle_event(Ok(Event::OnStatChange(OnStatChangeEvent {
                        entity: Entity {
                            uid: (*entity).RuntimeID__BackingField,
                            team: Team::Enemy,
                        },
                        stat,
                    })));
                }
                _ => {}
            }
        }
    }
    res
}

#[named]
pub fn on_entity_defeated(instance: *const TurnBasedAbilityComponent, entity: *const GameEntity) {
    log::debug!(function_name!());
    let res = ON_ENTITY_DEFEATED_Detour.call(instance, entity);
    unsafe {
        let defeated_entity = (*instance)._parent_object._OwnerRef;
        // if !TurnBasedAbilityComponent_TryCheckLimboWaitHeal(
        //     instance,
        //     (*instance)._parent_object._OwnerRef,
        // ) {
        //     if (*entity)._EntityType == EntityType::Avatar {
        //         let e = match helpers::get_avatar_from_entity(entity) {
        //             Ok(avatar) => Ok(Event::OnEntityDefeated(OnEntityDefeatedEvent {
        //                 killer: Entity {
        //                     uid: avatar.id,
        //                     team: Team::Player,
        //                 },
        //                 entity_defeated: Entity {
        //                     uid: (*defeated_entity).RuntimeID__BackingField,
        //                     team: Team::Enemy,
        //                 },
        //             })),
        //             Err(e) => {
        //                 log::error!("Avatar Event Error: {}", e);

        //                 Err(anyhow!("{} Avatar Event Error: {}", function_name!(), e))
        //             }
        //         };
        //         BattleContext::handle_event(e);
        //     };
        // }
        if (*entity)._EntityType == EntityType::Avatar {
            let e = match helpers::get_avatar_from_entity(entity) {
                Ok(avatar) => Ok(Event::OnEntityDefeated(OnEntityDefeatedEvent {
                    killer: Entity {
                        uid: avatar.id,
                        team: Team::Player,
                    },
                    entity_defeated: Entity {
                        uid: (*defeated_entity).RuntimeID__BackingField,
                        team: Team::Enemy,
                    },
                })),
                Err(e) => {
                    log::error!("Avatar Event Error: {}", e);

                    Err(anyhow!("{} Avatar Event Error: {}", function_name!(), e))
                }
            };
            BattleContext::handle_event(e);
        };

    }
    res
}

#[named]
pub fn on_update_team_formation(instance: *const TeamFormationComponent) {
    log::debug!(function_name!());
    let res = ON_UPDATE_TEAM_FORMATION_Detour.call(instance);
    unsafe {
        if (*instance)._Team == TeamType::TeamDark {
            let team = (*instance)._TeamFormationDatas;
            // Is this necessary?
            if !team.is_null() {
                let entities = (*team)
                    .to_slice()
                    .iter()
                    .map(|entity_formation| Entity {
                        uid: (*(**entity_formation)._parent_object._OwnerRef)
                            .RuntimeID__BackingField,
                        team: Team::Enemy,
                    })
                    .collect::<Vec<Entity>>();

                BattleContext::handle_event(Ok(Event::OnUpdateTeamFormation(
                    OnUpdateTeamFormationEvent {
                        entities,
                        team: Team::Enemy,
                    },
                )));
            }
        }
    }
}

#[named]
pub fn on_initialize_enemy(
    instance: *const MonsterDataComponent,
    turn_based_ability_component: *const TurnBasedAbilityComponent
) {
    log::debug!(function_name!());
    let res = ON_INITIALIZE_ENEMY_Detour.call(instance, turn_based_ability_component);
    unsafe {
        let row_data = (*instance)._MonsterRowData;
        let monster_id = MonsterDataComponent_GetMonsterID(instance as _);
        let base_stats = Stats {
            level: MonsterRowData_get_Level(row_data),
            hp: fixpoint_to_raw(&(*(instance))._DefaultMaxHP),
        };

        let name_id = std::mem::zeroed::<TextID>();
        MonsterRowData_get_CharacterName(&name_id, row_data);

        let monster_name = TextmapStatic_GetText(&name_id, null());

        let entity = (*(instance as *const MonsterDataComponent))
            ._parent_object
            ._parent_object
            ._OwnerRef;

        BattleContext::handle_event(Ok(Event::OnInitializeEnemy(OnInitializeEnemyEvent {
            enemy: Enemy {
                id: monster_id,
                uid: (*entity).RuntimeID__BackingField,
                name: (*monster_name).to_string(),
                base_stats,
            },
        })));
    }
    res
}

static_detour! {
	static ON_DAMAGE_Detour: fn(*const c_void, *const c_void, *const NOPBAAAGGLA, *const TurnBasedAbilityComponent, *const TurnBasedAbilityComponent, *const GameEntity, *const GameEntity, *const GameEntity, bool, *const c_void) -> bool;
	static ON_COMBO_Detour: fn(*const MMNDIEBMDNL);
	static ON_USE_SKILL_Detour: fn(*const SkillCharacterComponent, i32, *const c_void, bool, i32) -> bool;
	static ON_SET_LINEUP_Detour: fn(*const BattleAssetPreload, bool, *const c_void);
	static ON_BATTLE_BEGIN_Detour: fn(*const TurnBasedGameMode);
	static ON_BATTLE_END_Detour: fn(*const TurnBasedGameMode);
	static ON_TURN_BEGIN_Detour: fn(*const TurnBasedGameMode);
	static ON_TURN_END_Detour: fn(*const TurnBasedAbilityComponent, i32);
	static ON_UPDATE_WAVE_Detour: fn(*const TurnBasedGameMode);
	static ON_UPDATE_CYCLE_Detour: fn(*const TurnBasedGameMode) -> u32;
	static ON_DIRECT_CHANGE_HP_Detour: fn(*const TurnBasedAbilityComponent, i32, FixPoint, *const c_void);
	static ON_DIRECT_DAMAGE_HP_Detour: fn(*const TurnBasedAbilityComponent, FixPoint, i32, *const c_void, FixPoint, *const c_void);
	static ON_STAT_CHANGE_Detour: fn(*const TurnBasedAbilityComponent, AbilityProperty, i32, FixPoint, *const c_void);
	static ON_ENTITY_DEFEATED_Detour: fn(*const TurnBasedAbilityComponent, *const GameEntity);
	static ON_UPDATE_TEAM_FORMATION_Detour: fn(*const TeamFormationComponent);
	static ON_INITIALIZE_ENEMY_Detour: fn(*const MonsterDataComponent, *const TurnBasedAbilityComponent);
}
pub fn subscribe() -> Result<()> {
    unsafe {
        subscribe_function!(
            ON_DAMAGE_Detour,
            *GAMEASSEMBLY_HANDLE + 0x7490630,
            on_damage
        );
        subscribe_function!(ON_COMBO_Detour, *GAMEASSEMBLY_HANDLE + 0xc5627b0, on_combo);
        subscribe_function!(
            ON_USE_SKILL_Detour,
            *GAMEASSEMBLY_HANDLE + 0x7076490,
            on_use_skill
        );
        subscribe_function!(
            ON_SET_LINEUP_Detour,
            *GAMEASSEMBLY_HANDLE + 0xcb82710,
            on_set_lineup
        );
        subscribe_function!(
            ON_BATTLE_BEGIN_Detour,
            *GAMEASSEMBLY_HANDLE + 0x7145c80,
            on_battle_begin
        );
        subscribe_function!(
            ON_BATTLE_END_Detour,
            *GAMEASSEMBLY_HANDLE + 0x7145e60,
            on_battle_end
        );
        subscribe_function!(
            ON_TURN_BEGIN_Detour,
            *GAMEASSEMBLY_HANDLE + 0x713ef50,
            on_turn_begin
        );
        subscribe_function!(
            ON_TURN_END_Detour,
            *GAMEASSEMBLY_HANDLE + 0x71258b0,
            on_turn_end
        );
        subscribe_function!(
            ON_UPDATE_WAVE_Detour,
            *GAMEASSEMBLY_HANDLE + 0x7144fd0,
            on_update_wave
        );
        subscribe_function!(
            ON_UPDATE_CYCLE_Detour,
            *GAMEASSEMBLY_HANDLE + 0x714d950,
            on_update_cycle
        );
        subscribe_function!(
            ON_DIRECT_CHANGE_HP_Detour,
            *GAMEASSEMBLY_HANDLE + 0x7116390,
            on_direct_change_hp
        );
        subscribe_function!(
            ON_DIRECT_DAMAGE_HP_Detour,
            *GAMEASSEMBLY_HANDLE + 0x7115730,
            on_direct_damage_hp
        );
        subscribe_function!(
            ON_STAT_CHANGE_Detour,
            *GAMEASSEMBLY_HANDLE + 0x7108790,
            on_stat_change
        );
        subscribe_function!(
            ON_ENTITY_DEFEATED_Detour,
            *GAMEASSEMBLY_HANDLE + 0x712a9a0,
            on_entity_defeated
        );
        subscribe_function!(
            ON_UPDATE_TEAM_FORMATION_Detour,
            *GAMEASSEMBLY_HANDLE + 0x70a56f0,
            on_update_team_formation
        );
        subscribe_function!(
            ON_INITIALIZE_ENEMY_Detour,
            *GAMEASSEMBLY_HANDLE + 0x703abb0,
            on_initialize_enemy
        );

        Ok(())
    }
}
