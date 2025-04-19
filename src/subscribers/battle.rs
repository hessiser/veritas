use crate::battle::BattleContext;
// use crate::kreide::native_types::*;
use crate::kreide::types::rpg::gamecore::*;
use crate::kreide::types::*;
use crate::kreide::*;
// use crate::kreide::types::rpg::client::*;
use crate::kreide::functions::rpg::gamecore::*;
// use crate::kreide::functions::rpg::client::*;
use crate::kreide::helpers::*;

use crate::models::events::OnBattleEndEvent;
use crate::models::events::Event;
use crate::models::events::OnDamageEvent;
use crate::models::events::OnUseSkillEvent;
use crate::models::events::OnSetLineupEvent;
use crate::models::events::OnTurnBeginEvent;
use crate::models::misc::Avatar;
use crate::GAMEASSEMBLY_HANDLE;

use anyhow::Result;
use anyhow::{anyhow, Error};
use function_name::named;
use retour::static_detour;
use std::ffi::c_void;

static_detour! {
    static ON_DAMAGE_Detour: fn(
        *const c_void,
        *const c_void,
        *const NOPBAAAGGLA,
        *const c_void,
        *const c_void,
        *const GameEntity,
        *const GameEntity,
        *const GameEntity,
        bool,
        *const c_void
    ) -> bool;
    static ON_COMBO_Detour: fn(*const MMNDIEBMDNL);
    static ON_USE_SKILL_Detour: fn(*const SkillCharacterComponent, i32, *const c_void, bool, i32);
    static ON_SET_LINEUP_Detour: fn(*const c_void, *const BattleLineupData);
    static ON_BATTLE_BEGIN_Detour: fn(*const TurnBasedGameMode);
    static ON_BATTLE_END_Detour: fn(*const TurnBasedGameMode);
    static ON_TURN_BEGIN_Detour: fn(*const TurnBasedGameMode);
    static ON_TURN_END_Detour: fn(*const c_void, i32) -> *const c_void;
}

static mut TURN_BASED_GAME_MODE_REF: Option<*const TurnBasedGameMode> = None;

#[named]
fn get_elapsed_av() -> f64 {
    unsafe {
        log::debug!(function_name!());
        match TURN_BASED_GAME_MODE_REF {
            Some(x) => fixpoint_to_raw(&(*x).ElapsedActionDelay__BackingField) * 10f64,
            None => panic!("There was no reference to RPG.GameCore.TurnBasedGameMode"),
        }
    }
}

// Called on any instance of damage
#[named]
fn on_damage(
    task_context: *const c_void,
    damage_by_attack_property: *const c_void,
    nopbaaaggla: *const NOPBAAAGGLA,
    turn_based_ability_component_1: *const c_void,
    turn_based_ability_component_2: *const c_void,
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
                            })),
                            Err(e) => {
                                log::error!("Servant Event Error: {}", e);
                                Err(anyhow!("{} Servant Event Error: {}", function_name!(), e))
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
        turn_based_ability_component_1,
        turn_based_ability_component_2,
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
) {
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

    ON_USE_SKILL_Detour.call(instance, skill_index, a3, a4, skill_extra_use_param);
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
fn on_set_lineup(instance: *const c_void, battle_lineup_data: *const BattleLineupData) {
    log::debug!(function_name!());
    unsafe {
        let light_team = (*battle_lineup_data).LightTeam;
        let mut avatars = Vec::<Avatar>::new();
        let mut errors = Vec::<Error>::new();
        for character_ptr in (*light_team).to_slice() {
            let character = *character_ptr;
            let avatar_id = (*character).CharacterID;
            log::debug!("{}", format!("AVATAR ID: {}", avatar_id));
            match helpers::get_avatar_from_id(avatar_id) {
                Ok(avatar) => avatars.push(avatar),
                Err(e) => {
                    log::error!("BattleLineup Error: {}", e);
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
            Ok(Event::OnSetLineup(OnSetLineupEvent { avatars }))
        };
        BattleContext::handle_event(event);
    }
    ON_SET_LINEUP_Detour.call(instance, battle_lineup_data);
}

#[named]
fn on_battle_begin(instance: *const TurnBasedGameMode) {
    log::debug!(function_name!());
    unsafe {
        ON_BATTLE_BEGIN_Detour.call(instance);
        TURN_BASED_GAME_MODE_REF = Some(instance);
        BattleContext::handle_event(Ok(Event::OnBattleBegin));
    }
}

#[named]
fn on_battle_end(instance: *const TurnBasedGameMode) {
    log::debug!(function_name!());
    unsafe {
        ON_BATTLE_END_Detour.call(instance);
        BattleContext::handle_event(Ok(Event::OnBattleEnd(OnBattleEndEvent {
            action_value: get_elapsed_av(),
        })));
        TURN_BASED_GAME_MODE_REF = None;
    }
}

#[named]
fn on_turn_begin(instance: *const TurnBasedGameMode) {
    log::debug!(function_name!());
    // Update AV first
    ON_TURN_BEGIN_Detour.call(instance);
    BattleContext::handle_event(Ok(Event::OnTurnBegin(OnTurnBeginEvent {
        action_value: get_elapsed_av(),
    })));
}

#[named]
fn on_turn_end(instance: *const c_void, a1: i32) -> *const c_void {
    log::debug!(function_name!());
    // Can match player v enemy turn w/
    // RPG.GameCore.TurnBasedGameMode.GetCurrentTurnTeam
    let res = ON_TURN_END_Detour.call(instance, a1);
    BattleContext::handle_event(Ok(Event::OnTurnEnd));
    return res;
}

pub fn subscribe() -> Result<()> {
    unsafe {
        subscribe_function!(
            ON_DAMAGE_Detour, * GAMEASSEMBLY_HANDLE + 0x680fa70, on_damage
        );
        subscribe_function!(
            ON_COMBO_Detour, * GAMEASSEMBLY_HANDLE + 0x645a040, on_combo
        );
        subscribe_function!(
            ON_USE_SKILL_Detour, * GAMEASSEMBLY_HANDLE + 0x666fb90, on_use_skill
        );
        subscribe_function!(
            ON_SET_LINEUP_Detour, * GAMEASSEMBLY_HANDLE + 0x7fa0f70, on_set_lineup
        );
        subscribe_function!(
            ON_BATTLE_BEGIN_Detour, * GAMEASSEMBLY_HANDLE + 0x5c5fc00, on_battle_begin
        );
        subscribe_function!(
            ON_BATTLE_END_Detour, * GAMEASSEMBLY_HANDLE + 0x5c5fd20, on_battle_end
        );
        subscribe_function!(
            ON_TURN_BEGIN_Detour, * GAMEASSEMBLY_HANDLE + 0x5c5aad0, on_turn_begin
        );
        subscribe_function!(
            ON_TURN_END_Detour, * GAMEASSEMBLY_HANDLE + 0x5c483a0, on_turn_end
        );
        Ok(())
    }
}
