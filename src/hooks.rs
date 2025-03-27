#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::{ffi::c_void, mem};
use retour::static_detour;
use crate::{battle::BattleContext, models::{events::{BattleBeginEvent, Event, OnDamageEvent, OnKillEvent, SetBattleLineupEvent}, misc::Avatar}, sr::{functions::rpg::{client::{AvatarData_get_AvatarName, UIGameEntityUtils_GetAvatarID}, gamecore::{AbilityStatic_GetActualOwner, EntityManager__GetEntitySummoner, GamePlayStatic_GetEntityManager}}, helpers::{self, fixpoint_to_raw}, types::{rpg::gamecore::{BattleLineupData, EntityType, GameEntity, TeamType, TurnBasedGameMode}, HBIAGLPHICO, NOPBAAAGGLA}}, GAMEASSEMBLY_HANDLE};
use anyhow::Result;

// NOTE: Wrap logic in unsafe block even when safe, else might encounter some UB

macro_rules! hook_function {
    (
        $detour:ident,
        $target:expr,
        $reroute:ident
    ) => {
        $detour.initialize($target, $reroute)?;
        $detour.enable()?;
    };
}

static_detour! {
    static DMFMLMJKKHB_OMPLOLLELLK_Detour: fn(
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

    static RPG_GameCore_TurnBasedGameMode__GameModeBegin_Detour: fn(*const TurnBasedGameMode);
    static RPG_GameCore_TurnBasedGameMode__GameModeEnd_Detour: fn(*const TurnBasedGameMode);
    static RPG_Client_BattleAssetPreload_SetBattleLineupData_Detour: fn(*const c_void, *const BattleLineupData);
    static RPG_GameCore_TurnBasedGameMode__MakeLimboEntityDie_Detour: fn(*const c_void, *const HBIAGLPHICO) -> bool;
    static RPG_GameCore_TurnBasedAbilityComponent_ProcessOnLevelTurnActionEndEvent_Detour: fn(*const c_void, i32) -> *const c_void;
}

fn game_mode_begin(instance: *const TurnBasedGameMode) {
    BattleContext::handle_event(Event::BattleBegin(BattleBeginEvent { turn_based_game_mode: instance })).unwrap();
    RPG_GameCore_TurnBasedGameMode__GameModeBegin_Detour.call(instance)
}

fn set_battle_lineup_data(instance: *const c_void, battle_lineup_data: *const BattleLineupData) {
    unsafe {
        let light_team = (*battle_lineup_data).LightTeam;
        let mut avatars = Vec::<Avatar>::new();
        for character_ptr in (*light_team).to_slice() {
            let character = *character_ptr;
            let avatar_id = (*character).CharacterID;
            let avatar_data = helpers::get_avatar_data_by_id(avatar_id);
            let avatar_name = (*AvatarData_get_AvatarName(avatar_data)).to_string().unwrap();

            avatars.push(Avatar {
                id: avatar_id,
                name: avatar_name.clone(),
            });
        }
        BattleContext::handle_event(Event::SetBattleLineup(SetBattleLineupEvent { avatars })).unwrap();
    }
    RPG_Client_BattleAssetPreload_SetBattleLineupData_Detour.call(instance, battle_lineup_data);
}

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
        match (*attacker)._Team {
            // Light
            TeamType::TeamLight => {
                // Unsure of the relevance of the last 32 bits; not null
                // Might contain the decimal part of DMG?
                let damage = fixpoint_to_raw(&(*nopbaaaggla ).JFKEEOMKMLI);
                let attack_owner = AbilityStatic_GetActualOwner(attacker);
    
                match (*attack_owner)._EntityType {
                    EntityType::Avatar => {
                        let avatar_id = UIGameEntityUtils_GetAvatarID(attack_owner);
                        let avatar_data = helpers::get_avatar_data_by_id(avatar_id);
                        let avatar_name = (*AvatarData_get_AvatarName(avatar_data)).to_string().unwrap();
                    
                        BattleContext::handle_event(Event::OnDamage(OnDamageEvent {
                            attacker: Avatar {
                                id: avatar_id,
                                name: avatar_name,
                            },
                            damage,
                        })).unwrap();
                    }
                    EntityType::Servant => {
                        // can actually just save ref of battle and access this member thru battleinstance worldinstance
                        let entity_manager = GamePlayStatic_GetEntityManager();
                        let avatar_entity = EntityManager__GetEntitySummoner(entity_manager, attack_owner);
    
                        let avatar_id = UIGameEntityUtils_GetAvatarID(avatar_entity);
                        let avatar_data = helpers::get_avatar_data_by_id(avatar_id);
                        let avatar_name = (*AvatarData_get_AvatarName(avatar_data)).to_string().unwrap();
    
                        BattleContext::handle_event(Event::OnDamage(OnDamageEvent {
                            attacker: Avatar {
                                id: avatar_id,
                                name: avatar_name,
                            },
                            damage,
                        })).unwrap();
                    }
                    _ => (),
                }
            }
            _ => {}
        }
        
    }
    return DMFMLMJKKHB_OMPLOLLELLK_Detour.call(
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

fn ProcessOnLevelTurnActionEndEvent(instance: *const c_void, a1: i32) -> *const c_void {
    // Can match player v enemy turn w/
    // RPG.GameCore.TurnBasedGameMode.GetCurrentTurnTeam
    BattleContext::handle_event(Event::TurnEnd).unwrap();
    return RPG_GameCore_TurnBasedAbilityComponent_ProcessOnLevelTurnActionEndEvent_Detour.call(instance, a1);
}

fn _MakeLimboEntityDie(instance: *const c_void, a1: *const HBIAGLPHICO) -> bool {
    // This isn't general kills
    unsafe {
        let attacker = (*a1).JKCOIOLCMEP;
        match (*attacker)._Team {
            TeamType::TeamLight => match (*attacker)._EntityType {
                EntityType::Avatar => {
                    let avatar_id = UIGameEntityUtils_GetAvatarID((*a1).JKCOIOLCMEP);
                    let avatar_data = helpers::get_avatar_data_by_id(avatar_id);
                    let avatar_name = (*AvatarData_get_AvatarName(avatar_data)).to_string().unwrap();
                    BattleContext::handle_event(Event::OnKill(OnKillEvent {
                        attacker: Avatar {
                            id: avatar_id,
                            name: avatar_name,
                        },
                    })).unwrap();
                }
                _ => {}
            },
            _ => {}
        }    
    }
    return RPG_GameCore_TurnBasedGameMode__MakeLimboEntityDie_Detour.call(instance, a1);
}

fn game_mode_end(instance: *const TurnBasedGameMode) {
    BattleContext::handle_event(Event::BattleEnd).unwrap();
    RPG_GameCore_TurnBasedGameMode__GameModeEnd_Detour.call(instance);
}


pub fn install_hooks() -> Result<()> {
    unsafe {
        hook_function!(
            DMFMLMJKKHB_OMPLOLLELLK_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x75d1360),
            on_damage
        );
        hook_function!(
            RPG_GameCore_TurnBasedGameMode__GameModeBegin_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x943eab0),
            game_mode_begin
        );

        hook_function!(
            RPG_GameCore_TurnBasedGameMode__GameModeEnd_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x943ebd0),
            game_mode_end
        );
        hook_function!(
            RPG_Client_BattleAssetPreload_SetBattleLineupData_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x762dba0),
            set_battle_lineup_data
        );
        hook_function!(
            RPG_GameCore_TurnBasedGameMode__MakeLimboEntityDie_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x943d6c0),
            _MakeLimboEntityDie
        );
        hook_function!(
            RPG_GameCore_TurnBasedAbilityComponent_ProcessOnLevelTurnActionEndEvent_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x9400f10),
            ProcessOnLevelTurnActionEndEvent
        );
        Ok(())
    }
}
