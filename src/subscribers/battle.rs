use crate::battle::BattleContext;
use crate::kreide::helpers::*;
use crate::kreide::types::*;
use crate::kreide::*;

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
use il2cpp_runtime::Il2CppObject;
use std::ffi::c_void;

#[named]
unsafe fn get_elapsed_av(game_mode: RPG_GameCore_TurnBasedGameMode) -> Result<f64> {
    log::debug!(function_name!());
    Ok(fixpoint_to_raw(&*game_mode._ElapsedActionDelay_k__BackingField()?) * 10f64)
}

// Called on any instance of damage
#[named]
fn on_damage(
    task_context: *const c_void,
    damage_by_attack_property: *const c_void,
    damage_info: OHFGNONJNIG,
    attacker_ability: RPG_GameCore_TurnBasedAbilityComponent,
    defender_ability: RPG_GameCore_TurnBasedAbilityComponent,
    attacker: RPG_GameCore_GameEntity,
    defender: RPG_GameCore_GameEntity,
    attacker_task_single_target: RPG_GameCore_GameEntity,
    flag: bool,
    a10: *const c_void,
) -> bool {
    log::debug!(function_name!());

    let hp_initial = match defender_ability.get_property(RPG_GameCore_AbilityProperty::CurrentHP) {
        Ok(value) => value,
        Err(e) => {
            log::error!("{} HP initial error: {}", function_name!(), e);
            RPG_GameCore_FixPoint { m_rawValue: 0 }
        }
    };
    let res = ON_DAMAGE_Detour.call(
        task_context,
        damage_by_attack_property,
        damage_info,
        attacker_ability,
        defender_ability,
        attacker,
        defender,
        attacker_task_single_target,
        flag,
        a10,
    );
    let hp_final = match defender_ability.get_property(RPG_GameCore_AbilityProperty::CurrentHP) {
        Ok(value) => value,
        Err(e) => {
            log::error!("{} HP final error: {}", function_name!(), e);
            RPG_GameCore_FixPoint { m_rawValue: 0 }
        }
    };

    safe_call!(unsafe {
        let mut event: Option<Result<Event>> = None;
        match *attacker._Team()? {
            RPG_GameCore_TeamType::TeamLight => {
                // E8 56 66 26 F9
                // E8 Call instruction
                // Relative offset to the start of the function being called
                // Resolve to RVA of the function being called

                let damage = fixpoint_to_raw(&*damage_info.KJLBAGPFBDC()?);
                let hp_initial_raw = fixpoint_to_raw(&hp_initial);
                let hp_final_raw = fixpoint_to_raw(&hp_final);
                let overkill_damage = if hp_initial_raw <= 0.0 {
                    damage
                } else if hp_final_raw <= 0.0 {
                    (damage - hp_initial_raw).max(0.0)
                } else {
                    0.0
                };
                let damage_type = damage_info.DOODKEMMAPK()?;
                let attack_owner = {
                    let attack_owner = RPG_GameCore_AbilityStatic::get_actual_owner(attacker)?;
                    if !attack_owner.0.is_null() {
                        attack_owner
                    } else {
                        attacker
                    }
                };

                match *(attack_owner)._EntityType()? {
                    RPG_GameCore_EntityType::Avatar => {
                        let e = match helpers::get_avatar_from_entity(attack_owner) {
                            Ok(avatar) => Ok(Event::OnDamage(OnDamageEvent {
                                attacker: Entity {
                                    uid: avatar.id,
                                    team: Team::Player,
                                },
                                damage,
                                damage_type: *damage_type as isize,
                                overkill_damage,
                            })),
                            Err(e) => {
                                log::error!("Avatar Event Error: {}", e);
                                Err(anyhow!("{} Avatar Event Error: {}", function_name!(), e))
                            }
                        };
                        event = Some(e);
                    }
                    RPG_GameCore_EntityType::Servant => {
                        let e = match helpers::get_avatar_from_servant_entity(attack_owner) {
                            Ok(avatar) => Ok(Event::OnDamage(OnDamageEvent {
                                attacker: Entity {
                                    uid: avatar.id,
                                    team: Team::Player,
                                },
                                damage,
                                damage_type: *damage_type as isize,
                                overkill_damage,
                            })),
                            Err(e) => {
                                log::error!("Servant Event Error: {}", e);
                                Err(anyhow!("{} Servant Event Error: {}", function_name!(), e))
                            }
                        };
                        event = Some(e);
                    }
                    RPG_GameCore_EntityType::Snapshot => {
                        // Unsure if this is if only a servant died and inflicted a DOT
                        let character_data_comp = attacker_ability._CharacterDataRef()?;
                        let summoner_entity = character_data_comp.Summoner()?;
                        let e = match helpers::get_avatar_from_entity(summoner_entity) {
                            Ok(avatar) => Ok(Event::OnDamage(OnDamageEvent {
                                attacker: Entity {
                                    uid: avatar.id,
                                    team: Team::Player,
                                },
                                damage,
                                damage_type: *damage_type as isize,
                                overkill_damage,
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
                        *attacker._EntityType()? as usize
                    ),
                }
            }
            _ => {}
        }
        if let Some(event) = event {
            BattleContext::handle_event(event);
        }
        Ok(())
    });

    res
}

// Called when a manual skill is used. Does not account for insert skills (out of turn automatic skills)
#[named]
fn on_use_skill(
    instance: RPG_GameCore_SkillCharacterComponent,
    skill_index: i32,
    a3: *const c_void,
    a4: bool,
    a5: *const c_void,
    a6: *const c_void,
    skill_extra_use_param: i32,
) -> bool {
    log::debug!(function_name!());
    let res =
        ON_USE_SKILL_Detour.call(instance, skill_index, a3, a4, a5, a6, skill_extra_use_param);

    safe_call!(unsafe {
        let entity = instance.as_base()._OwnerRef()?;
        let skill_owner = {
            let skill_owner = RPG_GameCore_AbilityStatic::get_actual_owner(entity)?;
            if !skill_owner.0.is_null() {
                skill_owner
            } else {
                entity
            }
        };

        let mut event: Option<Result<Event>> = None;
        match *skill_owner._Team()? {
            RPG_GameCore_TeamType::TeamLight => {
                let skill_data = instance.get_skill_data(skill_index, skill_extra_use_param)?;

                if !skill_data.0.is_null() {
                    match *skill_owner._EntityType()? {
                        RPG_GameCore_EntityType::Avatar => {
                            let e = match get_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_entity(skill_owner) {
                                    Ok(avatar) => {
                                        if skill.name.is_empty() {
                                            return Ok(());
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
                        RPG_GameCore_EntityType::Servant => {
                            let e = match get_skill_from_skilldata(skill_data) {
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
                        RPG_GameCore_EntityType::BattleEvent => {
                            let battle_event_data_comp = RPG_GameCore_BattleEventDataComponent(
                                instance._CharacterDataRef()?.0,
                            );

                            let avatar_entity =
                                battle_event_data_comp._SourceCaster_k__BackingField()?;

                            let e = match get_skill_from_skilldata(skill_data) {
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
                            *skill_owner._EntityType()? as usize
                        ),
                    }
                }
            }
            _ => {}
        }
        if let Some(event) = event {
            BattleContext::handle_event(event);
        }
        Ok(())
    });

    res
}

// Insert skills are out of turn automatic skills
#[named]
fn on_combo(instance: FHPFLNJLDHP, game_mode: RPG_GameCore_TurnBasedGameMode) {
    log::debug!(function_name!());

    ON_COMBO_Detour.call(instance, game_mode);
    safe_call!(unsafe {
        let turn_based_ability_component = instance.MMALDILNGNJ()?;
        let skill_character_component = instance.HHOKFHMEFFF()?;
        let entity = skill_character_component.as_base()._OwnerRef()?;
        let skill_owner = {
            let skill_owner = RPG_GameCore_AbilityStatic::get_actual_owner(entity)?;
            if !skill_owner.0.is_null() {
                skill_owner
            } else {
                entity
            }
        };

        let mut event: Option<Result<Event>> = None;
        match *skill_owner._Team()? {
            RPG_GameCore_TeamType::TeamLight => {
                let ability_name = (instance.BIACLKKBFMM()?).AANENKIIIMF()?;

                let skill_name =
                    turn_based_ability_component.get_ability_mapped_skill(ability_name)?;

                let character_data_ref = turn_based_ability_component._CharacterDataRef()?;
                let character_config = character_data_ref._JsonConfig_k__BackingField()?;
                let skill_index = character_config.get_skill_index_by_trigger_key(skill_name)?;
                let skill_data = skill_character_component.get_skill_data(skill_index, -1)?;

                if !skill_data.0.is_null() {
                    match *skill_owner._EntityType()? {
                        RPG_GameCore_EntityType::Avatar => {
                            let e: std::result::Result<Event, anyhow::Error> =
                                match get_skill_from_skilldata(skill_data) {
                                    Ok(skill) => match get_avatar_from_entity(skill_owner) {
                                        Ok(avatar) => {
                                            if skill.name.is_empty() {
                                                return Ok(());
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
                        RPG_GameCore_EntityType::Servant => {
                            let e = match get_skill_from_skilldata(skill_data) {
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
                        RPG_GameCore_EntityType::BattleEvent => {
                            let battle_event_data_comp = RPG_GameCore_BattleEventDataComponent(
                                skill_character_component._CharacterDataRef()?.0,
                            );
                            let avatar_entity =
                                battle_event_data_comp._SourceCaster_k__BackingField()?;

                            let e = match get_skill_from_skilldata(skill_data) {
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
                            *skill_owner._EntityType()? as usize
                        ),
                    }
                }
            }
            _ => {}
        }
        if let Some(event) = event {
            BattleContext::handle_event(event);
        }
        Ok(())
    });
}

#[named]
fn on_set_lineup(
    instance: RPG_GameCore_BattleInstance,
    a1: *const c_void,
    a2: RPG_GameCore_BattleLineupData,
    a3: i32,
    a4: u32,
    a5: bool,
) {
    log::debug!(function_name!());
    safe_call!(unsafe {
        let light_team = a2.LightTeam()?;
        let mut avatars = Vec::<Avatar>::new();
        let mut errors = Vec::<Error>::new();
        for character in light_team.to_vec::<RPG_GameCore_LineUpCharacter>() {
            let avatar_id = character.CharacterID()?;
            match helpers::get_avatar_from_id((*avatar_id).into()) {
                Ok(avatar) => avatars.push(avatar),
                Err(e) => errors.push(e),
            }
        }

        // Unsure if you can have more than one support char
        let extra_team = a2.ExtraTeam()?;
        for character in extra_team.to_vec::<RPG_GameCore_LineUpCharacter>() {
            let avatar_id = character.CharacterID()?;
            match helpers::get_avatar_from_id((*avatar_id).into()) {
                Ok(avatar) => avatars.push(avatar),
                Err(e) => errors.push(e),
            }
        }

        let event = if !errors.is_empty() {
            let errors = errors
                .iter()
                .map(|e| format!("{}. ", e.to_string()))
                .collect::<String>();
            Err(anyhow!(errors))
        } else {
            Ok(Event::OnSetBattleLineup(OnSetLineupEvent { avatars }))
        };
        BattleContext::handle_event(event);
        Ok(())
    });
    ON_SET_LINEUP_Detour.call(instance, a1, a2, a3, a4, a5)
}

#[named]
fn on_battle_begin(instance: RPG_GameCore_TurnBasedGameMode) {
    log::debug!(function_name!());
    let res = ON_BATTLE_BEGIN_Detour.call(instance);
    safe_call!({
        BattleContext::handle_event(Ok(Event::OnBattleBegin(OnBattleBeginEvent {
            max_waves: u32::try_from(i32::from(
                &*instance._WaveMonsterMaxCount_k__BackingField()?,
            ))?,
            max_cycles: u32::from(&*instance._ChallengeTurnLimit_k__BackingField()?),
            stage_id: u32::from(&*instance._CurrentWaveStageID_k__BackingField()?),
        })));
        Ok(())
    });
    res
}

#[named]
fn on_battle_end(instance: RPG_GameCore_TurnBasedGameMode) {
    log::debug!(function_name!());
    let res = ON_BATTLE_END_Detour.call(instance);
    BattleContext::handle_event(Ok(Event::OnBattleEnd));
    res
}

#[named]
fn on_turn_begin(instance: RPG_GameCore_TurnBasedGameMode) {
    log::debug!(function_name!());
    // Update AV first
    let res = ON_TURN_BEGIN_Detour.call(instance);

    safe_call!(unsafe {
        let turn_owner = instance._CurrentTurnActionEntity()?;
        match *turn_owner._EntityType()? {
            RPG_GameCore_EntityType::Avatar => {
                let e = match helpers::get_avatar_from_entity(turn_owner) {
                    Ok(avatar) => Ok(Event::OnTurnBegin(OnTurnBeginEvent {
                        action_value: get_elapsed_av(instance)?,
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
            RPG_GameCore_EntityType::Monster => {
                let e = Ok(Event::OnTurnBegin(OnTurnBeginEvent {
                    action_value: get_elapsed_av(instance)?,
                    turn_owner: Some(Entity {
                        uid: (*turn_owner._RuntimeID_k__BackingField()?).into(),
                        team: Team::Enemy,
                    }),
                }));

                BattleContext::handle_event(e);
            }
            _ => {
                BattleContext::handle_event(Ok(Event::OnTurnBegin(OnTurnBeginEvent {
                    action_value: get_elapsed_av(instance)?,
                    turn_owner: None,
                })));
            }
        }
        Ok(())
    });
    res
}

#[named]
fn on_turn_end(instance: RPG_GameCore_TurnBasedAbilityComponent, a1: i32) {
    log::debug!(function_name!());
    // Can match player v enemy turn w/
    // RPG.GameCore.TurnBasedGameMode.GetCurrentTurnTeam
    BattleContext::handle_event(Ok(Event::OnTurnEnd));
    ON_TURN_END_Detour.call(instance, a1)
}

#[named]
pub fn on_update_wave(instance: RPG_GameCore_TurnBasedGameMode) {
    let res = ON_UPDATE_WAVE_Detour.call(instance);
    safe_call!({
        BattleContext::handle_event(Ok(Event::OnUpdateWave(OnUpdateWaveEvent {
            wave: u32::try_from(i32::from(&*instance._WaveMonsterCurrentCount()?))?,
        })));
        Ok(())
    });
    res
}

#[named]
pub fn on_update_cycle(instance: RPG_GameCore_TurnBasedGameMode) -> u32 {
    log::debug!(function_name!());
    let cycle = ON_UPDATE_CYCLE_Detour.call(instance);
    BattleContext::handle_event(Ok(Event::OnUpdateCycle(OnUpdateCycleEvent { cycle })));
    cycle
}

#[named]
fn handle_hp_change(turn_based_ability_component: RPG_GameCore_TurnBasedAbilityComponent) {
    log::debug!(function_name!());
    safe_call!(unsafe {
        let hp = fixpoint_to_raw(
            &turn_based_ability_component.get_property(RPG_GameCore_AbilityProperty::CurrentHP)?,
        );

        let entity = turn_based_ability_component.as_base()._OwnerRef()?;

        match *entity._EntityType()? {
            RPG_GameCore_EntityType::Avatar => {
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
            RPG_GameCore_EntityType::Monster => {
                BattleContext::handle_event(Ok(Event::OnStatChange(OnStatChangeEvent {
                    entity: Entity {
                        uid: (*entity._RuntimeID_k__BackingField()?).into(),
                        team: Team::Enemy,
                    },
                    stat: Stat::HP(hp),
                })));
            }
            _ => {}
        }
        Ok(())
    });
}

#[named]
pub fn on_direct_change_hp(
    instance: RPG_GameCore_TurnBasedAbilityComponent,
    a1: i32,
    a2: RPG_GameCore_FixPoint,
    a3: RPG_GameCore_FixPoint,
    a4: *const c_void
) {
    log::debug!(function_name!());
    let res = ON_DIRECT_CHANGE_HP_Detour.call(instance, a1, a2, a3, a4);
    handle_hp_change(instance);
    res
}

#[named]
pub fn on_direct_damage_hp(
    instance: RPG_GameCore_TurnBasedAbilityComponent,
    a1: RPG_GameCore_FixPoint,
    a2: RPG_GameCore_FixPoint,
    a3: i32,
    a4: *const c_void,
    a5: RPG_GameCore_FixPoint,
    a6: *const c_void,
) {
    log::debug!(function_name!());
    let res = ON_DIRECT_DAMAGE_HP_Detour.call(instance, a1, a2, a3, a4, a5, a6);
    handle_hp_change(instance);
    res
}

#[named]
pub fn on_stat_change(
    instance: RPG_GameCore_TurnBasedAbilityComponent,
    property: RPG_GameCore_AbilityProperty,
    a2: i32,
    new_stat: RPG_GameCore_FixPoint,
    a4: *const c_void,
) -> bool {
    log::debug!(function_name!());
    let res = ON_STAT_CHANGE_Detour.call(instance, property, a2, new_stat, a4);
    safe_call!(unsafe {
        let entity = instance.as_base()._OwnerRef()?;

        let stat = match property {
            RPG_GameCore_AbilityProperty::MaxHP => Some(Stat::MaxHP(fixpoint_to_raw(&new_stat))),
            RPG_GameCore_AbilityProperty::BaseHP => Some(Stat::BaseHP(fixpoint_to_raw(&new_stat))),
            RPG_GameCore_AbilityProperty::HPAddedRatio => {
                Some(Stat::HPAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::HPDelta => {
                Some(Stat::HPDelta(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::HPConvert => {
                Some(Stat::HPConvert(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::DirtyHPDelta => {
                Some(Stat::DirtyHPDelta(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::DirtyHPRatio => {
                Some(Stat::DirtyHPRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::RallyHP => {
                Some(Stat::RallyHP(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::NegativeHP => {
                Some(Stat::NegativeHP(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::CurrentHP => Some(Stat::HP(fixpoint_to_raw(&new_stat))),
            RPG_GameCore_AbilityProperty::MaxSP => Some(Stat::MaxSP(fixpoint_to_raw(&new_stat))),
            RPG_GameCore_AbilityProperty::CurrentSP => {
                Some(Stat::CurrentSP(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::MaxSpecialSP => {
                Some(Stat::MaxSpecialSP(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::CurrentSpecialSP => {
                Some(Stat::CurrentSpecialSP(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::AdditionalBP => {
                Some(Stat::AdditionalBP(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::Attack => Some(Stat::Attack(fixpoint_to_raw(&new_stat))),
            RPG_GameCore_AbilityProperty::BaseAttack => {
                Some(Stat::BaseAttack(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::AttackAddedRatio => {
                Some(Stat::AttackAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::AttackDelta => {
                Some(Stat::AttackDelta(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::AttackConvert => {
                Some(Stat::AttackConvert(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::Defence => {
                Some(Stat::Defense(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::BaseDefence => {
                Some(Stat::BaseDefence(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::DefenceAddedRatio => {
                Some(Stat::DefenceAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::DefenceDelta => {
                Some(Stat::DefenceDelta(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::DefenceConvert => {
                Some(Stat::DefenceConvert(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::DefenceOverride => {
                Some(Stat::DefenceOverride(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::Level => Some(Stat::Level(fixpoint_to_raw(&new_stat))),
            RPG_GameCore_AbilityProperty::Promotion => {
                Some(Stat::Promotion(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::Rank => Some(Stat::Rank(fixpoint_to_raw(&new_stat))),
            RPG_GameCore_AbilityProperty::Speed => Some(Stat::Speed(fixpoint_to_raw(&new_stat))),
            RPG_GameCore_AbilityProperty::BaseSpeed => {
                Some(Stat::BaseSpeed(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::SpeedAddedRatio => {
                Some(Stat::SpeedAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::SpeedDelta => {
                Some(Stat::SpeedDelta(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::SpeedConvert => {
                Some(Stat::SpeedConvert(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::SpeedOverride => {
                Some(Stat::SpeedOverride(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ActionDelay => {
                Some(Stat::AV(fixpoint_to_raw(&new_stat) * 10.0))
            }
            RPG_GameCore_AbilityProperty::ActionDelayAddedRatio => {
                Some(Stat::ActionDelayAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ActionDelayAddAttenuation => {
                Some(Stat::ActionDelayAddAttenuation(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::MaxStance => {
                Some(Stat::MaxStance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::CurrentStance => {
                Some(Stat::CurrentStance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::Level_FinalDamageAddedRatio => {
                Some(Stat::Level_AllDamageAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::AllDamageTypeAddedRatio => {
                Some(Stat::AllDamageTypeAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::AllDamageReduce => {
                Some(Stat::AllDamageReduce(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::DotDamageAddedRatio => {
                Some(Stat::DotDamageAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::FatigueRatio => {
                Some(Stat::FatigueRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::CriticalChance => {
                Some(Stat::CriticalChance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::CriticalChanceBase => {
                Some(Stat::CriticalChanceBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::CriticalChanceConvert => {
                Some(Stat::CriticalChanceConvert(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::CriticalDamage => {
                Some(Stat::CriticalDamage(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::CriticalDamageBase => {
                Some(Stat::CriticalDamageBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::CriticalDamageConvert => {
                Some(Stat::CriticalDamageConvert(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::CriticalResistance => {
                Some(Stat::CriticalResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::PhysicalAddedRatio => {
                Some(Stat::PhysicalAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::FireAddedRatio => {
                Some(Stat::FireAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::IceAddedRatio => {
                Some(Stat::IceAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ThunderAddedRatio => {
                Some(Stat::ThunderAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::QuantumAddedRatio => {
                Some(Stat::QuantumAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ImaginaryAddedRatio => {
                Some(Stat::ImaginaryAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::WindAddedRatio => {
                Some(Stat::WindAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::PhysicalResistance => {
                Some(Stat::PhysicalResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::FireResistance => {
                Some(Stat::FireResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::IceResistance => {
                Some(Stat::IceResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ThunderResistance => {
                Some(Stat::ThunderResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::QuantumResistance => {
                Some(Stat::QuantumResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ImaginaryResistance => {
                Some(Stat::ImaginaryResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::WindResistance => {
                Some(Stat::WindResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::PhysicalResistanceBase => {
                Some(Stat::PhysicalResistanceBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::FireResistanceBase => {
                Some(Stat::FireResistanceBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::IceResistanceBase => {
                Some(Stat::IceResistanceBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ThunderResistanceBase => {
                Some(Stat::ThunderResistanceBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::QuantumResistanceBase => {
                Some(Stat::QuantumResistanceBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ImaginaryResistanceBase => {
                Some(Stat::ImaginaryResistanceBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::WindResistanceBase => {
                Some(Stat::WindResistanceBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::PhysicalResistanceDelta => {
                Some(Stat::PhysicalResistanceDelta(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::FireResistanceDelta => {
                Some(Stat::FireResistanceDelta(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::IceResistanceDelta => {
                Some(Stat::IceResistanceDelta(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ThunderResistanceDelta => {
                Some(Stat::ThunderResistanceDelta(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::QuantumResistanceDelta => {
                Some(Stat::QuantumResistanceDelta(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ImaginaryResistanceDelta => {
                Some(Stat::ImaginaryResistanceDelta(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::WindResistanceDelta => {
                Some(Stat::WindResistanceDelta(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::AllDamageTypeResistance => {
                Some(Stat::AllDamageTypeResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::PhysicalPenetrate => {
                Some(Stat::PhysicalPenetrate(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::FirePenetrate => {
                Some(Stat::FirePenetrate(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::IcePenetrate => {
                Some(Stat::IcePenetrate(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ThunderPenetrate => {
                Some(Stat::ThunderPenetrate(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::QuantumPenetrate => {
                Some(Stat::QuantumPenetrate(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ImaginaryPenetrate => {
                Some(Stat::ImaginaryPenetrate(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::WindPenetrate => {
                Some(Stat::WindPenetrate(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::AllDamageTypePenetrate => {
                Some(Stat::AllDamageTypePenetrate(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::PhysicalTakenRatio => {
                Some(Stat::PhysicalTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::FireTakenRatio => {
                Some(Stat::FireTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::IceTakenRatio => {
                Some(Stat::IceTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ThunderTakenRatio => {
                Some(Stat::ThunderTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::QuantumTakenRatio => {
                Some(Stat::QuantumTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ImaginaryTakenRatio => {
                Some(Stat::ImaginaryTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::WindTakenRatio => {
                Some(Stat::WindTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::AllDamageTypeTakenRatio => {
                Some(Stat::AllDamageTypeTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::Monster_DamageTakenRatio => {
                Some(Stat::Monster_DamageTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::PhysicalAbsorb => {
                Some(Stat::PhysicalAbsorb(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::FireAbsorb => {
                Some(Stat::FireAbsorb(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::IceAbsorb => {
                Some(Stat::IceAbsorb(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ThunderAbsorb => {
                Some(Stat::ThunderAbsorb(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::QuantumAbsorb => {
                Some(Stat::QuantumAbsorb(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ImaginaryAbsorb => {
                Some(Stat::ImaginaryAbsorb(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::WindAbsorb => {
                Some(Stat::WindAbsorb(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::MinimumFatigueRatio => {
                Some(Stat::MinimumFatigueRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ForceStanceBreakRatio => {
                Some(Stat::ForceStanceBreakRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::StanceBreakAddedRatio => {
                Some(Stat::StanceBreakAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::StanceBreakResistance => {
                Some(Stat::StanceBreakResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::StanceBreakTakenRatio => {
                Some(Stat::StanceBreakTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::PhysicalStanceBreakTakenRatio => Some(
                Stat::PhysicalStanceBreakTakenRatio(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::FireStanceBreakTakenRatio => {
                Some(Stat::FireStanceBreakTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::IceStanceBreakTakenRatio => {
                Some(Stat::IceStanceBreakTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ThunderStanceBreakTakenRatio => Some(
                Stat::ThunderStanceBreakTakenRatio(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::WindStanceBreakTakenRatio => {
                Some(Stat::WindStanceBreakTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::QuantumStanceBreakTakenRatio => Some(
                Stat::QuantumStanceBreakTakenRatio(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::ImaginaryStanceBreakTakenRatio => Some(
                Stat::ImaginaryStanceBreakTakenRatio(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::StanceWeakAddedRatio => {
                Some(Stat::StanceWeakAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::StanceDefaultAddedRatio => {
                Some(Stat::StanceDefaultAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::HealRatio => {
                Some(Stat::HealRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::HealRatioBase => {
                Some(Stat::HealRatioBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::HealRatioConvert => {
                Some(Stat::HealRatioConvert(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::HealTakenRatio => {
                Some(Stat::HealTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::Shield => Some(Stat::Shield(fixpoint_to_raw(&new_stat))),
            RPG_GameCore_AbilityProperty::MaxShield => {
                Some(Stat::MaxShield(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ShieldAddedRatio => {
                Some(Stat::ShieldAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ShieldTakenRatio => {
                Some(Stat::ShieldTakenRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::StatusProbability => {
                Some(Stat::StatusProbability(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::StatusProbabilityBase => {
                Some(Stat::StatusProbabilityBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::StatusProbabilityConvert => {
                Some(Stat::StatusProbabilityConvert(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::StatusResistance => {
                Some(Stat::StatusResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::StatusResistanceBase => {
                Some(Stat::StatusResistanceBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::StatusResistanceConvert => {
                Some(Stat::StatusResistanceConvert(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::SPRatio => {
                Some(Stat::SPRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::SPRatioBase => {
                Some(Stat::SPRatioBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::SPRatioConvert => {
                Some(Stat::SPRatioConvert(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::SPRatioOverride => {
                Some(Stat::SPRatioOverride(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::BreakDamageAddedRatio => {
                Some(Stat::BreakDamageAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::BreakDamageAddedRatioBase => {
                Some(Stat::BreakDamageAddedRatioBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::BreakDamageAddedRatioConvert => Some(
                Stat::BreakDamageAddedRatioConvert(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::BreakDamageExtraAddedRatio => {
                Some(Stat::BreakDamageExtraAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::PhysicalStanceBreakResistance => Some(
                Stat::PhysicalStanceBreakResistance(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::FireStanceBreakResistance => {
                Some(Stat::FireStanceBreakResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::IceStanceBreakResistance => {
                Some(Stat::IceStanceBreakResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ThunderStanceBreakResistance => Some(
                Stat::ThunderStanceBreakResistance(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::WindStanceBreakResistance => {
                Some(Stat::WindStanceBreakResistance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::QuantumStanceBreakResistance => Some(
                Stat::QuantumStanceBreakResistance(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::ImaginaryStanceBreakResistance => Some(
                Stat::ImaginaryStanceBreakResistance(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::AggroBase => {
                Some(Stat::AggroBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::AggroAddedRatio => {
                Some(Stat::AggroAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::AggroDelta => {
                Some(Stat::AggroDelta(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::RelicValueExtraAdditionRatio => Some(
                Stat::RelicValueExtraAdditionRatio(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::EquipValueExtraAdditionRatio => Some(
                Stat::EquipValueExtraAdditionRatio(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::EquipExtraRank => {
                Some(Stat::EquipExtraRank(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::AvatarExtraRank => {
                Some(Stat::AvatarExtraRank(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::Combo => Some(Stat::Combo(fixpoint_to_raw(&new_stat))),
            RPG_GameCore_AbilityProperty::NormalBattleCount => {
                Some(Stat::NormalBattleCount(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraAttackAddedRatio1 => {
                Some(Stat::ExtraAttackAddedRatio1(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraAttackAddedRatio2 => {
                Some(Stat::ExtraAttackAddedRatio2(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraAttackAddedRatio3 => {
                Some(Stat::ExtraAttackAddedRatio3(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraAttackAddedRatio4 => {
                Some(Stat::ExtraAttackAddedRatio4(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraDefenceAddedRatio1 => {
                Some(Stat::ExtraDefenceAddedRatio1(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraDefenceAddedRatio2 => {
                Some(Stat::ExtraDefenceAddedRatio2(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraDefenceAddedRatio3 => {
                Some(Stat::ExtraDefenceAddedRatio3(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraDefenceAddedRatio4 => {
                Some(Stat::ExtraDefenceAddedRatio4(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraHPAddedRatio1 => {
                Some(Stat::ExtraHPAddedRatio1(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraHPAddedRatio2 => {
                Some(Stat::ExtraHPAddedRatio2(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraHPAddedRatio3 => {
                Some(Stat::ExtraHPAddedRatio3(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraHPAddedRatio4 => {
                Some(Stat::ExtraHPAddedRatio4(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraHealAddedRatio => {
                Some(Stat::ExtraHealAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraAllDamageTypeAddedRatio1 => Some(
                Stat::ExtraAllDamageTypeAddedRatio1(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::ExtraAllDamageTypeAddedRatio2 => Some(
                Stat::ExtraAllDamageTypeAddedRatio2(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::ExtraAllDamageTypeAddedRatio3 => Some(
                Stat::ExtraAllDamageTypeAddedRatio3(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::ExtraAllDamageTypeAddedRatio4 => Some(
                Stat::ExtraAllDamageTypeAddedRatio4(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::ExtraAllDamageReduce => {
                Some(Stat::ExtraAllDamageReduce(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraShieldAddedRatio => {
                Some(Stat::ExtraShieldAddedRatio(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraSpeedAddedRatio1 => {
                Some(Stat::ExtraSpeedAddedRatio1(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraSpeedAddedRatio2 => {
                Some(Stat::ExtraSpeedAddedRatio2(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraSpeedAddedRatio3 => {
                Some(Stat::ExtraSpeedAddedRatio3(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraSpeedAddedRatio4 => {
                Some(Stat::ExtraSpeedAddedRatio4(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraLuckChance => {
                Some(Stat::ExtraLuckChance(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraLuckDamage => {
                Some(Stat::ExtraLuckDamage(fixpoint_to_raw(&new_stat)))
            }
            // RPG_GameCore_AbilityProperty::ExtraFrontPower => {
            //     Some(Stat::ExtraFrontPower(fixpoint_raw_to_raw(&new_stat)))
            // }
            RPG_GameCore_AbilityProperty::ExtraFrontPowerBase => {
                Some(Stat::ExtraFrontPowerBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraFrontPowerAddedRatio1 => {
                Some(Stat::ExtraFrontPowerAddedRatio1(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraFrontPowerAddedRatio2 => {
                Some(Stat::ExtraFrontPowerAddedRatio2(fixpoint_to_raw(&new_stat)))
            }
            // RPG_GameCore_AbilityProperty::ExtraBackPower => {
            //     Some(Stat::ExtraBackPower(fixpoint_raw_to_raw(&new_stat)))
            // }
            RPG_GameCore_AbilityProperty::ExtraBackPowerBase => {
                Some(Stat::ExtraBackPowerBase(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraBackPowerAddedRatio1 => {
                Some(Stat::ExtraBackPowerAddedRatio1(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraBackPowerAddedRatio2 => {
                Some(Stat::ExtraBackPowerAddedRatio2(fixpoint_to_raw(&new_stat)))
            }
            RPG_GameCore_AbilityProperty::ExtraUltraDamageAddedRatio1 => Some(
                Stat::ExtraUltraDamageAddedRatio1(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::ExtraSkillDamageAddedRatio1 => Some(
                Stat::ExtraSkillDamageAddedRatio1(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::ExtraNormalDamageAddedRatio1 => Some(
                Stat::ExtraNormalDamageAddedRatio1(fixpoint_to_raw(&new_stat)),
            ),
            RPG_GameCore_AbilityProperty::ExtraInsertDamageAddedRatio1 => Some(
                Stat::ExtraInsertDamageAddedRatio1(fixpoint_to_raw(&new_stat)),
            ),
            // RPG_GameCore_AbilityProperty::ExtraDOTDamageAddedRatio1 => {
            //     Some(Stat::ExtraDOTDamageAddedRatio1(fixpoint_to_raw(&new_stat)))
            // }
            _ => None,
        };

        if let Some(stat) = stat {
            match *entity._EntityType()? {
                RPG_GameCore_EntityType::Avatar => {
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
                RPG_GameCore_EntityType::Monster => {
                    BattleContext::handle_event(Ok(Event::OnStatChange(OnStatChangeEvent {
                        entity: Entity {
                            uid: (*entity._RuntimeID_k__BackingField()?).into(),
                            team: Team::Enemy,
                        },
                        stat,
                    })));
                }
                _ => {}
            }
        }
        Ok(())
    });
    res
}

#[named]
pub fn on_entity_defeated(instance: RPG_GameCore_TurnBasedGameMode, a2: FGFFLOAEKKA) -> bool {
    log::debug!(function_name!());
    let res = ON_ENTITY_DEFEATED_Detour.call(instance, a2);
    safe_call!(unsafe {
        // update; need to verify correct entity
        let defeated_entity = a2.LFGAFLLHGCO()?;
        let killer_entity = a2.GHCPGPKNBGF()?;

        if res && *defeated_entity._AliveState()? == RPG_GameCore_AliveState::Dying {
            if *killer_entity._EntityType()? == RPG_GameCore_EntityType::Avatar {
                let e = match helpers::get_avatar_from_entity(killer_entity) {
                    Ok(avatar) => Ok(Event::OnEntityDefeated(OnEntityDefeatedEvent {
                        killer: Entity {
                            uid: avatar.id,
                            team: Team::Player,
                        },
                        entity_defeated: Entity {
                            uid: (*defeated_entity._RuntimeID_k__BackingField()?).into(),
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
        Ok(())
    });
    res
}

#[named]
pub fn on_update_team_formation(instance: RPG_GameCore_TeamFormationComponent) {
    log::debug!(function_name!());
    let res = ON_UPDATE_TEAM_FORMATION_Detour.call(instance);
    safe_call!({
        if *instance._Team()? == RPG_GameCore_TeamType::TeamDark {
            let team = instance._TeamFormationDatas()?;
            let entities = team
                .to_vec::<RPG_GameCore_GameComponentBase>()
                .iter()
                .map(|entity_formation| Entity {
                    uid: (*entity_formation
                        ._OwnerRef()
                        .unwrap()
                        ._RuntimeID_k__BackingField()
                        .unwrap())
                    .into(),
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
        Ok(())
    });
    res
}

#[named]
pub fn on_initialize_enemy(
    instance: RPG_GameCore_MonsterDataComponent,
    turn_based_ability_component: RPG_GameCore_TurnBasedAbilityComponent,
) {
    log::debug!(function_name!());
    let res = ON_INITIALIZE_ENEMY_Detour.call(instance, turn_based_ability_component);
    safe_call!({
        let row_data = instance._MonsterRowData()?;
        let row = row_data._Row()?;
        let monster_id = instance.get_monster_id()?;
        let base_stats = Stats {
            level: row_data.get_Level()?,
            hp: fixpoint_to_raw(&*instance._DefaultMaxHP()?),
        };

        let name_id = row.MonsterName()?;
        let monster_name = get_textmap_content(&name_id)?;
        let entity = instance._OwnerRef()?;

        BattleContext::handle_event(Ok(Event::OnInitializeEnemy(OnInitializeEnemyEvent {
            enemy: Enemy {
                id: monster_id,
                uid: (*entity._RuntimeID_k__BackingField().unwrap()).into(),
                name: (*monster_name).to_string(),
                base_stats,
            },
        })));
        Ok(())
    });
    res
}

retour::static_detour! {
    static ON_DAMAGE_Detour: fn(*const c_void, *const c_void, OHFGNONJNIG, RPG_GameCore_TurnBasedAbilityComponent, RPG_GameCore_TurnBasedAbilityComponent, RPG_GameCore_GameEntity, RPG_GameCore_GameEntity, RPG_GameCore_GameEntity, bool, *const c_void) -> bool;
    static ON_COMBO_Detour: fn(FHPFLNJLDHP, RPG_GameCore_TurnBasedGameMode);
    static ON_USE_SKILL_Detour: fn(RPG_GameCore_SkillCharacterComponent, i32, *const c_void, bool, *const c_void, *const c_void, i32) -> bool;
    static ON_SET_LINEUP_Detour: fn(RPG_GameCore_BattleInstance, *const c_void, RPG_GameCore_BattleLineupData, i32, u32, bool);
    static ON_BATTLE_BEGIN_Detour: fn(RPG_GameCore_TurnBasedGameMode);
    static ON_BATTLE_END_Detour: fn(RPG_GameCore_TurnBasedGameMode);
    static ON_TURN_BEGIN_Detour: fn(RPG_GameCore_TurnBasedGameMode);
    static ON_TURN_END_Detour: fn(RPG_GameCore_TurnBasedAbilityComponent, i32);
    static ON_UPDATE_WAVE_Detour: fn(RPG_GameCore_TurnBasedGameMode);
    static ON_UPDATE_CYCLE_Detour: fn(RPG_GameCore_TurnBasedGameMode) -> u32;
    static ON_DIRECT_CHANGE_HP_Detour: fn(RPG_GameCore_TurnBasedAbilityComponent, i32, RPG_GameCore_FixPoint, RPG_GameCore_FixPoint, *const c_void);
    static ON_DIRECT_DAMAGE_HP_Detour: fn(RPG_GameCore_TurnBasedAbilityComponent, RPG_GameCore_FixPoint, RPG_GameCore_FixPoint, i32, *const c_void, RPG_GameCore_FixPoint, *const c_void);
    static ON_STAT_CHANGE_Detour: fn(RPG_GameCore_TurnBasedAbilityComponent, RPG_GameCore_AbilityProperty, i32, RPG_GameCore_FixPoint, *const c_void) -> bool;
    static ON_ENTITY_DEFEATED_Detour: fn(RPG_GameCore_TurnBasedGameMode, FGFFLOAEKKA) -> bool;
    static ON_UPDATE_TEAM_FORMATION_Detour: fn(RPG_GameCore_TeamFormationComponent);
    static ON_INITIALIZE_ENEMY_Detour: fn(RPG_GameCore_MonsterDataComponent, RPG_GameCore_TurnBasedAbilityComponent);
}

pub fn subscribe() -> Result<()> {
    unsafe {
        subscribe_function!(
            ON_DAMAGE_Detour,
            EBKLINDPMKM::get_class_static()?
                .find_method(
                    "LCKBMHEANKL",
                    vec![
                        "RPG.GameCore.TaskContext",
                        "RPG.GameCore.DamageByAttackProperty",
                        "*",
                        "RPG.GameCore.TurnBasedAbilityComponent",
                        "RPG.GameCore.TurnBasedAbilityComponent",
                        "RPG.GameCore.GameEntity",
                        "RPG.GameCore.GameEntity",
                        "RPG.GameCore.GameEntity",
                        "bool",
                        "*"
                    ]
                )?
                .va(),
            on_damage
        )?;

        subscribe_function!(
            ON_COMBO_Detour,
            FHPFLNJLDHP::get_class_static()?
                .find_method("HFKBBMIDMOJ", vec!["RPG.GameCore.TurnBasedGameMode"])?
                .va(),
            on_combo
        )?;

        subscribe_function!(
            ON_USE_SKILL_Detour,
            RPG_GameCore_SkillCharacterComponent::get_class_static()?
                .find_method(
                    "UseSkill",
                    vec![
                        "int",
                        "RPG.GameCore.AbilityCursorInfo",
                        "bool",
                        "System.Collections.Generic.List<RPG.GameCore.AbilityDynamicFloatInjection>",
                        "System.Collections.Generic.List<RPG.GameCore.AbilityDynamicStringInjection>",
                        "int"
                    ]
                )?
                .va(),
            on_use_skill
        )?;

        subscribe_function!(
            ON_SET_LINEUP_Detour,
            RPG_GameCore_BattleInstance::get_class_static()?
                .find_method(
                    ".ctor",
                    vec!["*", "RPG.GameCore.BattleLineupData", "int", "uint", "bool"]
                )?
                .va(),
            on_set_lineup
        )?;
        subscribe_function!(
            ON_BATTLE_BEGIN_Detour,
            RPG_GameCore_TurnBasedGameMode::get_class_static()?
                .find_method("_GameModeBegin", vec![])?
                .va(),
            on_battle_begin
        )?;
        subscribe_function!(
            ON_BATTLE_END_Detour,
            RPG_GameCore_TurnBasedGameMode::get_class_static()?
                .find_method("_GameModeEnd", vec![])?
                .va(),
            on_battle_end
        )?;
        subscribe_function!(
            ON_TURN_BEGIN_Detour,
            RPG_GameCore_TurnBasedGameMode::get_class_static()?
                .find_method("DoTurnPrepareStartWork", vec![])?
                .va(),
            on_turn_begin
        )?;
        subscribe_function!(
            ON_TURN_END_Detour,
            RPG_GameCore_TurnBasedAbilityComponent::get_class_static()?
                .find_method("ProcessOnLevelTurnActionEndEvent", vec!["int"])?
                .va(),
            on_turn_end
        )?;
        subscribe_function!(
            ON_UPDATE_WAVE_Detour,
            RPG_GameCore_TurnBasedGameMode::get_class_static()?
                .find_method("UpdateCurrentWaveCount", vec![])?
                .va(),
            on_update_wave
        )?;
        subscribe_function!(
            ON_UPDATE_CYCLE_Detour,
            RPG_GameCore_TurnBasedGameMode::get_class_static()?
                .find_method("get_ChallengeTurnLeft", vec![])?
                .va(),
            on_update_cycle
        )?;
        subscribe_function!(
            ON_DIRECT_CHANGE_HP_Detour,
            RPG_GameCore_TurnBasedAbilityComponent::get_class_static()?
                .find_method(
                    "DirectChangeHP",
                    vec![
                        "RPG.GameCore.PropertyModifyFunction",
                        "RPG.GameCore.FixPoint",
                        "RPG.GameCore.FixPoint",
                        "*"
                    ],
                )?
                .va(),
            on_direct_change_hp
        )?;
        subscribe_function!(
            ON_DIRECT_DAMAGE_HP_Detour,
            RPG_GameCore_TurnBasedAbilityComponent::get_class_static()?
                // Not sure if I need keyword out
                .find_method(
                    "DirectDamageHP",
                    vec![
                        "RPG.GameCore.FixPoint",
                        "RPG.GameCore.FixPoint",
                        "RPG.GameCore.AntiLockHPStrength",
                        "*",
                        "RPG.GameCore.FixPoint&",
                        "System.Nullable<RPG.GameCore.FixPoint>"
                    ],
                )?
                .va(),
            on_direct_damage_hp
        )?;
        subscribe_function!(
            ON_STAT_CHANGE_Detour,
            RPG_GameCore_TurnBasedAbilityComponent::get_class_static()?
                .find_method(
                    "ModifyProperty",
                    vec![
                        "RPG.GameCore.AbilityProperty",
                        "RPG.GameCore.PropertyModifyFunction",
                        "RPG.GameCore.FixPoint",
                        "*"
                    ]
                )?
                .va(),
            on_stat_change
        )?;
        subscribe_function!(
            ON_ENTITY_DEFEATED_Detour,
            RPG_GameCore_TurnBasedGameMode::get_class_static()
                .unwrap()
                .find_method("_MakeLimboEntityDie", vec!["*"])?
                .va(),
            on_entity_defeated
        )?;
        subscribe_function!(
            ON_UPDATE_TEAM_FORMATION_Detour,
            RPG_GameCore_TeamFormationComponent::get_class_static()?
                .find_method("_RefreshTeammateIndex", vec![])?
                .va(),
            on_update_team_formation
        )?;
        subscribe_function!(
            ON_INITIALIZE_ENEMY_Detour,
            RPG_GameCore_MonsterDataComponent::get_class_static()?
                .find_method(
                    "OnAbilityCharacterInitialized",
                    vec!["RPG.GameCore.TurnBasedAbilityComponent"],
                )?
                .va(),
            on_initialize_enemy
        )?;
        Ok(())
    }
}
