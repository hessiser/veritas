use std::{ffi::c_void, ptr::null, sync::{LazyLock, Mutex, MutexGuard}};

use anyhow::{ Context, Ok, Result, };

use crate::{models::{events::Event, misc::{Avatar, TurnInfo}, packets::{EventPacket, Packet}}, server, sr::{helpers::fixpoint_to_raw, types::rpg::gamecore::TurnBasedGameMode}};

pub enum BattleState {
    Preparing,
    Started,
    Ended,
}

impl Default for BattleState {
    fn default() -> Self {
        Self::Preparing
    }
}

#[derive(Default)]
pub struct BattleContext {
    state: BattleState,
    lineup: Vec<Avatar>,
    turn_history: Vec<TurnInfo>,
    current_turn_info: TurnInfo,
    turn_count: usize,
}

static BATTLE_CONTEXT: LazyLock<Mutex<BattleContext>> = LazyLock::new(|| Mutex::new(BattleContext::default()));
static mut TURN_BASED_GAME_MODE_REF: Option<*const TurnBasedGameMode> = None;

impl BattleContext {
    fn get_instance() -> MutexGuard<'static, Self> {
        BATTLE_CONTEXT.lock().unwrap()
    }

    fn find_lineup_index_by_avatar_id(battle_context: &MutexGuard<'static, Self>, avatar_id: u32) -> Option<usize> {
        let res = battle_context.lineup.iter().enumerate().find(|(_index, avatar)| avatar.id == avatar_id);
        res.map_or(None, |(index, _)| Some(index))
    }

    fn initialize_battle_context(battle_context: &mut MutexGuard<'static, Self>, lineup: Vec<Avatar>) {
        battle_context.current_turn_info = TurnInfo::default();
        battle_context.current_turn_info.avatars_damage = vec![0f32; lineup.len()];
        battle_context.current_turn_info.avatars_damage_chunks = vec![Vec::new(); lineup.len()];
        battle_context.turn_history = Vec::new();
        battle_context.turn_count = 0;
        battle_context.lineup = lineup;
    }

    // Consumes the event
    pub fn handle_event(event: Event) -> Result<()> {
        let mut battle_context = Self::get_instance();
        let packet: Packet;
        match event {
            Event::BattleBegin(e) => {
                unsafe { TURN_BASED_GAME_MODE_REF = Some(e.turn_based_game_mode) };

                let packet_body = EventPacket::BattleBegin {  };
                packet = Packet::from_event_packet(packet_body)?;
            },
            Event::SetBattleLineup(e) => {
                battle_context.state = BattleState::Started;
                Self::initialize_battle_context(&mut battle_context, e.avatars);

                for avatar in &battle_context.lineup {
                    println!("[VERITAS] ({}: {}) was loaded in lineup", avatar.id, avatar.name);
                }

                let packet_body = EventPacket::SetBattleLineup {
                    avatars: battle_context.lineup.clone()
                };
                packet = Packet::from_event_packet(packet_body)?;
            }
            Event::OnDamage(e) => {
                let lineup_index = Self::find_lineup_index_by_avatar_id(&battle_context, e.attacker.id)
                    .with_context(|| format!("Could not find avatar ({}: {}) in lineup", e.attacker.id, e.attacker.name))?;
                let turn = &mut battle_context.current_turn_info;
                // Record character damage chunk
                turn.avatars_damage_chunks[lineup_index].push(e.damage);

                println!("[VERITAS] ({}: {}) dealt {} damage", e.attacker.id, e.attacker.name, e.damage);

                let packet_body = EventPacket::OnDamage {
                    attacker: e.attacker,
                    damage: e.damage,
                };
                packet = Packet::from_event_packet(packet_body)?;
            }
            Event::TurnEnd => {
                let action_value = unsafe {
                    match TURN_BASED_GAME_MODE_REF {
                        Some(x) => {
                            fixpoint_to_raw(&(*x).ElapsedActionDelay__BackingField) * 10f32
                        },
                        None => panic!("There was no reference to RPG.GameCore.TurnBasedGameMode"),
                    }
                };

                battle_context.current_turn_info.action_value = action_value;
                let mut turn_info = battle_context.current_turn_info.clone();

                // Calculate net damages
                let avatars_damage = turn_info.avatars_damage_chunks
                    .iter()
                    .map(|avatar_dmg_string| avatar_dmg_string.iter().sum())
                    .collect::<Vec<f32>>();
                turn_info.total_damage = avatars_damage.iter().sum();
                turn_info.avatars_damage = avatars_damage;
                battle_context.turn_history.push(turn_info.clone());

                let packet_body = EventPacket::TurnEnd {
                    action_value,
                    avatars: battle_context.lineup.clone(),
                    avatars_damage: turn_info.avatars_damage,
                    total_damage: turn_info.total_damage,
                };
                packet = Packet::from_event_packet(packet_body)?;
                println!("AV: {:.2}", action_value);
                // Restart turn info
                battle_context.current_turn_info = TurnInfo::default();
                battle_context.current_turn_info.avatars_damage = vec![0f32; battle_context.lineup.len()];
                battle_context.current_turn_info.avatars_damage_chunks = vec![Vec::new(); battle_context.lineup.len()];
                battle_context.turn_count += 1;
            }
            Event::OnKill(e) => {
                println!("[VERITAS] ({}: {}) has killed", e.attacker.id, e.attacker.name);

                let packet_body = EventPacket::OnKill {
                    attacker: e.attacker,
                };

                packet = Packet::from_event_packet(packet_body)?;
            }
            Event::BattleEnd => {
                let total_damage = battle_context.turn_history
                    .iter()
                    .map(|x| x.total_damage)
                    .sum();
                battle_context.state = BattleState::Ended;
                let packet_body = EventPacket::BattleEnd {
                    avatars: battle_context.lineup.clone(),
                    turn_history: battle_context.turn_history.clone(),
                    turn_count: battle_context.turn_count,
                    total_damage
                };
                unsafe { TURN_BASED_GAME_MODE_REF = None };
                packet = Packet::from_event_packet(packet_body)?;
            }
        }
        drop(battle_context);
        server::broadcast(packet);
        Ok(())
    }
}
