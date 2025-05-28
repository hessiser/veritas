use crate::battle::BattleContext;
use crate::kreide::types::rpg::client::TextID;
// use crate::kreide::native_types::*;
use crate::kreide::types::rpg::gamecore::*;
use crate::kreide::types::*;
use crate::kreide::*;
// use crate::kreide::types::rpg::client::*;
use crate::kreide::functions::rpg::gamecore::*;
// use crate::kreide::functions::rpg::client::*;
use crate::kreide::helpers::*;
use crate::kreide::types::rpg::client::BattleAssetPreload;

use crate::models::events::*;
use crate::models::misc::Avatar;
use crate::GAMEASSEMBLY_HANDLE;

use anyhow::Result;
use anyhow::{anyhow, Error};
use function_name::named;
use retour::static_detour;
use std::ffi::c_void;

#[named]
unsafe fn get_elapsed_av(game_mode: *const TurnBasedGameMode) -> f64 {
    log::debug!(function_name!());
    unsafe {
        fixpoint_to_raw(&(*game_mode).ElapsedActionDelay__BackingField) * 10f64
    }
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
        let mut event: Option<Result<Event>> = None;
        match (*attacker)._Team {
            TeamType::TeamLight => {
                let damage = fixpoint_to_raw(&(*nopbaaaggla).JFKEEOMKMLI);
                let damage_type = (*nopbaaaggla).APDDLHNGGIM; 
                let attack_owner = {
                    let attack_owner = AbilityStatic_GetActualOwner(attacker);
                    if attack_owner.is_null() {
                        attacker
                    }
                    else {
                        attack_owner
                    }
                };

                match (*attack_owner)._EntityType {
                    EntityType::Avatar => {
                        let e = match helpers::get_avatar_from_entity(attack_owner) {
                            Ok(avatar) => Ok(Event::OnDamage(OnDamageEvent {
                                attacker: avatar,
                                damage,
                                damage_type: damage_type as isize
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
                                attacker: avatar,
                                damage,
                                damage_type: damage_type as isize
                            })),
                            Err(e) => {
                                log::error!("Servant Event Error: {}", e);
                                Err(anyhow!("{} Servant Event Error: {}", function_name!(), e))
                            }
                        };
                        event = Some(e);
                    },
                    EntityType::Snapshot => {
                        // Unsure if this is if only a servant died and inflicted a DOT
                        let character_data_comp = (*attacker_ability)._CharacterDataRef;
                        let summoner_entity = (*character_data_comp).Summoner;
                        let e = match helpers::get_avatar_from_entity(summoner_entity) {
                            Ok(avatar) => Ok(Event::OnDamage(OnDamageEvent {
                                attacker: avatar,
                                damage,
                                damage_type: damage_type as isize
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
            BattleContext::handle_event(event);
        }
    }
    return ON_DAMAGE_Detour.call(
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
            }
            else {
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
                                            return ON_USE_SKILL_Detour.call(instance, skill_index, a3, a4, skill_extra_use_param);
                                        }
                                        Ok(Event::OnUseSkill(OnUseSkillEvent { avatar, skill }))
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
                                    log::error!("Avatar Skill Event Error: {}", e);
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
                                    Ok(avatar) => {
                                        Ok(Event::OnUseSkill(OnUseSkillEvent { avatar, skill }))
                                    }
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
                            let battle_event_data_comp =
                                (*instance)._CharacterDataRef as *const BattleEventDataComponent;
                            let avatar_entity: *const GameEntity =
                                (*battle_event_data_comp).SourceCaster__BackingField;

                            let e = match get_battle_event_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_entity(avatar_entity) {
                                    Ok(avatar) => {
                                        Ok(Event::OnUseSkill(OnUseSkillEvent { avatar, skill }))
                                    }
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
            }
            else {
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
                                            Ok(Event::OnUseSkill(OnUseSkillEvent { avatar, skill }))
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
                                    Ok(avatar) => {
                                        Ok(Event::OnUseSkill(OnUseSkillEvent { avatar, skill }))
                                    }
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
                                    Ok(avatar) => {
                                        Ok(Event::OnUseSkill(OnUseSkillEvent { avatar, skill }))
                                    }
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
                    log::error!("BattleLineup Error: {}", e);
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
            stage_id: (*instance).CurrentWaveStageID__BackingField
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
                    Ok(avatar) => {
                        Ok(Event::OnTurnBegin(OnTurnBeginEvent {
                            action_value: get_elapsed_av(instance),
                            turn_owner: Some(avatar)
                        }))
                    },
                    Err(e) => {
                        log::error!("Avatar Event Error: {}", e);
                        Err(anyhow!("{} Avatar Event Error: {}", function_name!(), e))
                    }
                };

                BattleContext::handle_event(e);
            },
            _ => {
                BattleContext::handle_event(Ok(Event::OnTurnBegin(OnTurnBeginEvent {
                    action_value: get_elapsed_av(instance),
                    turn_owner: None
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

pub fn on_update_wave(instance: *const TurnBasedGameMode) {
    let res = ON_UPDATE_WAVE_Detour.call(instance);
    unsafe {
        BattleContext::handle_event(Ok(Event::OnUpdateWave(OnUpdateWaveEvent {
            wave: (*instance)._WaveMonsterCurrentCount as _,
        })));
    }
    res
}
pub fn on_update_cycle(instance: *const TurnBasedGameMode) -> u32 {
    let cycle = ON_UPDATE_CYCLE_Detour.call(instance);
    BattleContext::handle_event(Ok(Event::OnUpdateCycle(OnUpdateCycleEvent {
        cycle
    })));
    cycle
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
	static ON_DIRECT_DAMAGE_HP_Detour: fn(*const TurnBasedAbilityComponent, FixPoint, i32, *const c_void, FixPoint, c_void);
	static ON_STAT_CHANGE_Detour: fn(*const TurnBasedAbilityComponent, i32, i32, FixPoint, *const c_void);
	static ON_ENTITY_DEFEATED_Detour: fn(*const TurnBasedAbilityComponent, *const GameEntity);
	static ON_UPDATE_TEAM_FORMATION_Detour: fn(*const TeamFormationComponent);
	static ON_INITIALIZE_ENEMY_Detour: fn(*const MonsterDataComponent, *const TurnBasedAbilityComponent);
}
pub fn subscribe() -> Result<()> {
    unsafe {
        subscribe_function!(
            ON_DAMAGE_Detour, * GAMEASSEMBLY_HANDLE + 0x748a2d0, on_damage
        );
        subscribe_function!(
            ON_COMBO_Detour, * GAMEASSEMBLY_HANDLE + 0xc563df0, on_combo
        );
        subscribe_function!(
            ON_USE_SKILL_Detour, * GAMEASSEMBLY_HANDLE + 0x7075d40, on_use_skill
        );
        subscribe_function!(
            ON_SET_LINEUP_Detour, * GAMEASSEMBLY_HANDLE + 0xcb883d0, on_set_lineup
        );
        subscribe_function!(
            ON_BATTLE_BEGIN_Detour, * GAMEASSEMBLY_HANDLE + 0x71434c0, on_battle_begin
        );
        subscribe_function!(
            ON_BATTLE_END_Detour, * GAMEASSEMBLY_HANDLE + 0x71436a0, on_battle_end
        );
        subscribe_function!(
            ON_TURN_BEGIN_Detour, * GAMEASSEMBLY_HANDLE + 0x713c7c0, on_turn_begin
        );
        subscribe_function!(
            ON_TURN_END_Detour, * GAMEASSEMBLY_HANDLE + 0x71235d0, on_turn_end
        );
        subscribe_function!(
            ON_UPDATE_WAVE_Detour, * GAMEASSEMBLY_HANDLE + 0x7142840, on_update_wave
        );
        subscribe_function!(
            ON_UPDATE_CYCLE_Detour, * GAMEASSEMBLY_HANDLE + 0x714b120, on_update_cycle
        );
        // subscribe_function!(
        //     ON_DIRECT_CHANGE_HP_Detour, * GAMEASSEMBLY_HANDLE + 0x71140f0,
        //     on_direct_change_hp
        // );
        // subscribe_function!(
        //     ON_DIRECT_DAMAGE_HP_Detour, * GAMEASSEMBLY_HANDLE + 0x7113490,
        //     on_direct_damage_hp
        // );
        // subscribe_function!(
        //     ON_STAT_CHANGE_Detour, * GAMEASSEMBLY_HANDLE + 0x71067c0, on_stat_change
        // );
        // subscribe_function!(
        //     ON_ENTITY_DEFEATED_Detour, * GAMEASSEMBLY_HANDLE + 0x71283f0,
        //     on_entity_defeated
        // );
        // subscribe_function!(
        //     ON_UPDATE_TEAM_FORMATION_Detour, * GAMEASSEMBLY_HANDLE + 0x70a4650,
        //     on_update_team_formation
        // );
        // subscribe_function!(
        //     ON_INITIALIZE_ENEMY_Detour, * GAMEASSEMBLY_HANDLE + 0x703a400,
        //     on_initialize_enemy
        // );
        Ok(())
    }
}
