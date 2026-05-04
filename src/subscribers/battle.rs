use crate::battle::BattleContext;
use crate::kreide::helpers::*;
use crate::kreide::types::*;
use crate::kreide::*;

use crate::models::events::*;
use crate::models::types::Avatar;
use crate::models::types::BattleStats;
use crate::models::types::Enemy;
use crate::models::types::Entity;
use crate::models::types::Property;
use crate::models::types::Team;

use anyhow::Result;
use anyhow::{Error, anyhow};
use function_name::named;
use il2cpp_runtime::Il2CppClass;
use il2cpp_runtime::Il2CppObject;
use il2cpp_runtime::api::il2cpp_class_get_fields;
use il2cpp_runtime::api::il2cpp_field_get_name;
use il2cpp_runtime::api::il2cpp_field_get_offset;
use il2cpp_runtime::api::il2cpp_field_get_type;
use il2cpp_runtime::get_cached_class;
use il2cpp_runtime::types::Il2CppString;
use std::collections::{HashMap, VecDeque};
use std::ffi::c_void;
use std::ptr::null;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

#[named]
unsafe fn get_elapsed_av(game_mode: RPG_GameCore_TurnBasedGameMode) -> Result<f64> {
    Ok(fixpoint_to_raw(&*game_mode._ElapsedActionDelay_k__BackingField()?) * 10f64)
}

#[derive(Clone, Copy)]
struct ComboFieldOffsets {
    turn_based_ability_component: usize,
    skill_character_component: usize,
    ability_name_outer: usize,
    ability_name_inner: usize,
}

static COMBO_FIELD_OFFSETS: OnceLock<ComboFieldOffsets> = OnceLock::new();
static ATTACK_TYPE_OFFSET: OnceLock<usize> = OnceLock::new();
// Cached TurnBasedAbilityComponent runtime type for hot-path component lookup.
static TBASC_RUNTIME_TYPE: OnceLock<usize> = OnceLock::new();

/// Resolve TurnBasedAbilityComponent through the normal component lookup path while reusing a
/// cached type handle so the hot path does not redo name-based resolution.
unsafe fn find_tba_component(
    entity: RPG_GameCore_GameEntity,
) -> Option<RPG_GameCore_TurnBasedAbilityComponent> {
    let ty = il2cpp_runtime::System_RuntimeType(*TBASC_RUNTIME_TYPE.get()? as *const c_void);
    let component = unsafe { entity.get_component(ty).ok()? };
    if component.0.is_null() {
        return None;
    }

    Some(RPG_GameCore_TurnBasedAbilityComponent(component.0))
}

fn parse_il2cpp_enum<TObj, TEnum>(enum_obj: TObj) -> Result<TEnum>
where
    TObj: Il2CppObject,
    TEnum: Copy,
{
    if enum_obj.as_ptr().is_null() {
        return Err(anyhow!(
            "Null enum object while parsing {}",
            std::any::type_name::<TEnum>()
        ));
    }

    Ok(unsafe { *((enum_obj.as_ptr() as *const u8).add(0x10) as *const TEnum) })
}

fn attack_type_name(value: i32) -> String {
    match value {
        x if x == RPG_GameCore_AttackType::Unknown as i32 => RPG_GameCore_AttackType::Unknown.to_string(),
        x if x == RPG_GameCore_AttackType::Normal as i32 => RPG_GameCore_AttackType::Normal.to_string(),
        x if x == RPG_GameCore_AttackType::BPSkill as i32 => RPG_GameCore_AttackType::BPSkill.to_string(),
        x if x == RPG_GameCore_AttackType::Ultra as i32 => RPG_GameCore_AttackType::Ultra.to_string(),
        x if x == RPG_GameCore_AttackType::QTE as i32 => RPG_GameCore_AttackType::QTE.to_string(),
        x if x == RPG_GameCore_AttackType::DOT as i32 => RPG_GameCore_AttackType::DOT.to_string(),
        x if x == RPG_GameCore_AttackType::Pursued as i32 => RPG_GameCore_AttackType::Pursued.to_string(),
        x if x == RPG_GameCore_AttackType::Maze as i32 => RPG_GameCore_AttackType::Maze.to_string(),
        x if x == RPG_GameCore_AttackType::MazeNormal as i32 => RPG_GameCore_AttackType::MazeNormal.to_string(),
        x if x == RPG_GameCore_AttackType::Insert as i32 => RPG_GameCore_AttackType::Insert.to_string(),
        x if x == RPG_GameCore_AttackType::ElementDamage as i32 => RPG_GameCore_AttackType::ElementDamage.to_string(),
        x if x == RPG_GameCore_AttackType::Level as i32 => RPG_GameCore_AttackType::Level.to_string(),
        x if x == RPG_GameCore_AttackType::Servant as i32 => RPG_GameCore_AttackType::Servant.to_string(),
        x if x == RPG_GameCore_AttackType::TrueDamage as i32 => RPG_GameCore_AttackType::TrueDamage.to_string(),
        x if x == RPG_GameCore_AttackType::ElationDamage as i32 => RPG_GameCore_AttackType::ElationDamage.to_string(),
        _ => format!("Unknown({value})"),
    }
}

#[derive(Clone)]
struct PendingDamageContext {
    id: u64,
    attacker: Entity,
    r#type: String,
    raw_damage: f64,
    raw_damage_fixpoint: i64,
    hp_before_raw: f64,
}

static PENDING_DAMAGE_CONTEXTS: OnceLock<Mutex<HashMap<u32, VecDeque<PendingDamageContext>>>> =
    OnceLock::new();
static DAMAGE_EVENT_SEQ: AtomicU64 = AtomicU64::new(1);

fn pending_damage_contexts() -> &'static Mutex<HashMap<u32, VecDeque<PendingDamageContext>>> {
    PENDING_DAMAGE_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

unsafe fn resolve_combo_field_offsets(class: Il2CppClass) -> Result<ComboFieldOffsets> {
    let field_iter_1: *const c_void = null();
    let mut turn_based_ability_component_offset = None;
    let mut skill_character_component_offset = None;
    let mut ability_name_outer_offset = None;
    let mut ability_name_inner_offset = None;

    loop {
        let field = il2cpp_class_get_fields(class, &field_iter_1);
        if field.0.is_null() {
            break;
        }

        let field_type = il2cpp_field_get_type(field);
        if field_type.name() == RPG_GameCore_TurnBasedAbilityComponent::ffi_name() {
            turn_based_ability_component_offset = Some(il2cpp_field_get_offset(field) as usize);
        } else if field_type.name() == RPG_GameCore_SkillCharacterComponent::ffi_name() {
            skill_character_component_offset = Some(il2cpp_field_get_offset(field) as usize);
        } else if is_obfuscated_name(field_type.name()) {
            ability_name_outer_offset = Some(il2cpp_field_get_offset(field) as usize);

            let field_iter_2: *const c_void = null();
            loop {
                let field_inner = il2cpp_class_get_fields(field_type.class(), &field_iter_2);
                if field_inner.0.is_null() {
                    break;
                }

                let field_inner_type = il2cpp_field_get_type(field_inner);
                if field_inner_type.name() == Il2CppString::ffi_name() {
                    ability_name_inner_offset = Some(il2cpp_field_get_offset(field_inner) as usize);
                    break;
                }
            }
        }
    }

    Ok(ComboFieldOffsets {
        turn_based_ability_component: turn_based_ability_component_offset
            .context("Failed to find TurnBasedAbilityComponent field offset")?,
        skill_character_component: skill_character_component_offset
            .context("Failed to find SkillCharacterComponent field offset")?,
        ability_name_outer: ability_name_outer_offset
            .context("Failed to find obfuscated ability-name container field offset")?,
        ability_name_inner: ability_name_inner_offset
            .context("Failed to find Il2CppString ability-name field offset")?,
    })
}

unsafe fn get_combo_field_offsets(class: Il2CppClass) -> Result<ComboFieldOffsets> {
    if let Some(offsets) = COMBO_FIELD_OFFSETS.get() {
        return Ok(*offsets);
    }

    let offsets = unsafe { resolve_combo_field_offsets(class)? };
    let _ = COMBO_FIELD_OFFSETS.set(offsets);
    COMBO_FIELD_OFFSETS
        .get()
        .copied()
        .ok_or_else(|| anyhow!("Failed to cache on_combo field offsets"))
}

unsafe fn resolve_attack_type_offset(class: Il2CppClass) -> Result<usize> {
    const NCJILFFNINI_ATTACK_TYPE_OFFSET: usize = 0x4c0;
    const NCJILFFNINI_ATTACK_TYPE_FIELD_NAME: &str = "JLCBDNLOHGI";

    let class_name = class.name();
    let field_iter: *const c_void = null();
    loop {
        let field = il2cpp_class_get_fields(class, &field_iter);
        if field.0.is_null() {
            break;
        }

        let field_offset = il2cpp_field_get_offset(field) as usize;
        let field_name = unsafe { il2cpp_runtime::utils::cstr_to_str(il2cpp_field_get_name(field)) };
        let field_type = il2cpp_field_get_type(field);
        let field_type_name = field_type.name();
        if field_type_name == "AttackType"
            || field_type_name.ends_with(".AttackType")
            || (class_name == "NCJILFFNINI" && field_name == NCJILFFNINI_ATTACK_TYPE_FIELD_NAME)
        {
            return Ok(field_offset);
        }
    }

    if class_name == "NCJILFFNINI" {
        return Ok(NCJILFFNINI_ATTACK_TYPE_OFFSET);
    }

    Err(anyhow!(
        "Failed to find AttackType field offset in damage info class {}",
        class_name
    ))
}

unsafe fn get_attack_type_offset(class: Il2CppClass) -> Result<usize> {
    if let Some(offset) = ATTACK_TYPE_OFFSET.get() {
        return Ok(*offset);
    }

    let offset = unsafe { resolve_attack_type_offset(class)? };
    let _ = ATTACK_TYPE_OFFSET.set(offset);
    ATTACK_TYPE_OFFSET
        .get()
        .copied()
        .ok_or_else(|| anyhow!("Failed to cache attack type offset"))
}

// Called on any damage instance
#[named]
fn on_damage(
    task_context: *const c_void,
    damage_by_attack_property: *const c_void,
    damage_info: *const c_void,
    attacker_ability: RPG_GameCore_TurnBasedAbilityComponent,
    defender_ability: RPG_GameCore_TurnBasedAbilityComponent,
    attacker: RPG_GameCore_GameEntity,
    defender: RPG_GameCore_GameEntity,
    attacker_task_single_target: RPG_GameCore_GameEntity,
    flag: bool,
    a10: *const c_void,
) -> bool {
    let hp_initial = match unsafe {
        defender_ability.get_property(RPG_GameCore_AbilityProperty::CurrentHP)
    } {
        Ok(value) => value,
        Err(_) => RPG_GameCore_FixPoint { m_rawValue: 0 },
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

    let hp_final = match unsafe {
        defender_ability.get_property(RPG_GameCore_AbilityProperty::CurrentHP)
    } {
        Ok(value) => value,
        Err(_) => RPG_GameCore_FixPoint { m_rawValue: 0 },
    };

    safe_call!(unsafe {
        if damage_info.is_null() {
            return Ok(());
        }

        let attacker_team_value: RPG_GameCore_TeamType = parse_il2cpp_enum(attacker._Team()?)?;
        if attacker_team_value != RPG_GameCore_TeamType::TeamLight {
            return Ok(());
        }

        let raw_damage = if let Some(damage_offset) = get_damage_field_offset() {
            let damage_ptr = damage_info.byte_offset(damage_offset as isize) as *const RPG_GameCore_FixPoint;
            if damage_ptr.is_null() {
                0.0
            } else {
                fixpoint_to_raw(&*damage_ptr).max(0.0)
            }
        } else {
            read_damage_fixpoint(damage_info)
                .map(|fp| fixpoint_to_raw(&fp).max(0.0))
                .unwrap_or(0.0)
        };

        let hp_initial_raw = fixpoint_to_raw(&hp_initial);
        let hp_final_raw = fixpoint_to_raw(&hp_final);
        let hp_damage = (hp_initial_raw - hp_final_raw).max(0.0);

        let damage = if hp_damage > 0.0 { hp_damage } else { raw_damage };

        if damage <= 0.0 {
            return Ok(());
        }

        let overkill_damage = if hp_initial_raw <= 0.0 {
            raw_damage.max(damage)
        } else if hp_final_raw <= 0.0 {
            (raw_damage.max(damage) - hp_initial_raw).max(0.0)
        } else {
            0.0
        };

        let attack_type_offset = get_attack_type_offset(Il2CppClass(*(damage_info as *const *const c_void)))?;
        let attack_type = *(damage_info.byte_offset(attack_type_offset as isize) as *const i32);
        let attack_owner = {
            let ao = RPG_GameCore_AbilityStatic::get_actual_owner(attacker)?;
            if !ao.0.is_null() { ao } else { attacker }
        };

        let attack_owner_entity_value: RPG_GameCore_EntityType =
            parse_il2cpp_enum(attack_owner._EntityType()?)?;

        match attack_owner_entity_value {
            RPG_GameCore_EntityType::Avatar
            | RPG_GameCore_EntityType::Servant
            | RPG_GameCore_EntityType::Snapshot
            | RPG_GameCore_EntityType::BattleEvent => {
                if let Ok(avatar) = helpers::get_avatar_from_owner_entity(attack_owner) {
                    BattleContext::handle_event(Ok(Event::OnDamage(OnDamageEvent {
                        attacker: Entity {
                            uid: avatar.id,
                            team: Team::Player,
                        },
                        damage,
                        overkill_damage,
                        r#type: attack_type_name(attack_type),
                    })));
                }
            }
            _ => {}
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
        let skill_owner_team_value: RPG_GameCore_TeamType =
            parse_il2cpp_enum(skill_owner._Team()?)?;

        match skill_owner_team_value {
            RPG_GameCore_TeamType::TeamLight => {
                let skill_data = instance.get_skill_data(skill_index, skill_extra_use_param)?;

                if !skill_data.0.is_null() {
                    let entity_value: RPG_GameCore_EntityType =
                        parse_il2cpp_enum(skill_owner._EntityType()?)?;
                    match entity_value {
                        RPG_GameCore_EntityType::Avatar => {
                            let e = (|| -> Result<Option<Event>> {
                                let avatar = get_avatar_from_entity(skill_owner).map_err(|e| {
                                    log::error!("Avatar Event Error: {}", e);
                                    anyhow!("{} Avatar Event Error: {}", function_name!(), e)
                                })?;

                                let skill = get_skill_from_skilldata(skill_data).map_err(|e| {
                                    log::error!("Avatar Event Error: {}", e);
                                    anyhow!("{} Avatar Skill Event Error: {}", function_name!(), e)
                                })?;

                                if skill.name.is_empty() {
                                    return Ok(None);
                                }

                                Ok(Some(Event::OnUseSkill(OnUseSkillEvent {
                                    avatar: Entity {
                                        uid: avatar.id,
                                        team: Team::Player,
                                    },
                                    skill,
                                })))
                            })();
                            match e {
                                Ok(Some(e)) => event = Some(Ok(e)),
                                Ok(None) => {}
                                Err(e) => event = Some(Err(e)),
                            }
                        }
                        RPG_GameCore_EntityType::Servant => {
                            let e = (|| -> Result<Event> {
                                let avatar = get_avatar_from_entity(
                                    instance._CharacterDataRef()?.Summoner()?,
                                )
                                .map_err(|e| {
                                    log::error!("Servant Event Error: {}", e);
                                    anyhow!("{} Servant Event Error: {}", function_name!(), e)
                                })?;

                                let skill = get_skill_from_skilldata(skill_data).map_err(|e| {
                                    log::error!("Servant Skill Error: {}", e);
                                    anyhow!("{} Servant Skill Event Error: {}", function_name!(), e)
                                })?;

                                Ok(Event::OnUseSkill(OnUseSkillEvent {
                                    avatar: Entity {
                                        uid: avatar.id,
                                        team: Team::Player,
                                    },
                                    skill,
                                }))
                            })();
                            event = Some(e);
                        }
                        RPG_GameCore_EntityType::BattleEvent => {
                            let battle_event_data_comp = RPG_GameCore_BattleEventDataComponent(
                                instance._CharacterDataRef()?.0,
                            );
                            let avatar_entity = match battle_event_data_comp._SourceCaster_k__BackingField() {
                                Ok(e) if !e.0.is_null() => e,
                                _ => return Ok(()), // SourceCaster not set; skip silently
                            };

                            let e = match get_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_owner_entity(avatar_entity) {
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
fn on_combo(instance: *const c_void, game_mode: RPG_GameCore_TurnBasedGameMode) {

    ON_COMBO_Detour.call(instance, game_mode);
    safe_call!(unsafe {
        if instance.is_null() {
            return Err(anyhow!("on_combo received null instance pointer"));
        }

        let instance_class_ptr = *(instance as *const *const c_void);
        if instance_class_ptr.is_null() {
            return Err(anyhow!("on_combo instance has null class pointer"));
        }

        let offsets = get_combo_field_offsets(Il2CppClass(instance_class_ptr))?;

        let turn_based_ability_component_ptr = *((instance
            .byte_offset(offsets.turn_based_ability_component as isize))
            as *const *const c_void);
        if turn_based_ability_component_ptr.is_null() {
            return Err(anyhow!(
                "on_combo resolved null TurnBasedAbilityComponent pointer"
            ));
        }
        let turn_based_ability_component =
            RPG_GameCore_TurnBasedAbilityComponent(turn_based_ability_component_ptr);

        let skill_character_component_ptr = *((instance
            .byte_offset(offsets.skill_character_component as isize))
            as *const *const c_void);
        if skill_character_component_ptr.is_null() {
            return Err(anyhow!(
                "on_combo resolved null SkillCharacterComponent pointer"
            ));
        }
        let skill_character_component =
            RPG_GameCore_SkillCharacterComponent(skill_character_component_ptr);

        let ability_name_container =
            *((instance.byte_offset(offsets.ability_name_outer as isize)) as *const *const c_void);
        if ability_name_container.is_null() {
            return Ok(());
        }

        let ability_name_ptr = *(ability_name_container
            .byte_offset(offsets.ability_name_inner as isize)
            as *const *const c_void);
        if ability_name_ptr.is_null() {
            return Ok(());
        }

        let ability_name = Il2CppString(ability_name_ptr);

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
        let skill_owner_team_value: RPG_GameCore_TeamType =
            parse_il2cpp_enum(skill_owner._Team()?)?;

        match skill_owner_team_value {
            RPG_GameCore_TeamType::TeamLight => {
                let skill_name =
                    turn_based_ability_component.get_ability_mapped_skill(ability_name)?;

                let character_data_ref = turn_based_ability_component._CharacterDataRef()?;
                let character_config = character_data_ref._JsonConfig_k__BackingField()?;
                let skill_index = character_config.get_skill_index_by_trigger_key(skill_name)?;
                let skill_data = skill_character_component.get_skill_data(skill_index, -1)?;

                if !skill_data.0.is_null() {
                    let entity_value: RPG_GameCore_EntityType =
                        parse_il2cpp_enum(skill_owner._EntityType()?)?;

                    match entity_value {
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
                            let avatar_entity = match battle_event_data_comp._SourceCaster_k__BackingField() {
                                Ok(e) if !e.0.is_null() => e,
                                _ => return Ok(()), // SourceCaster not set; skip silently
                            };

                            let e = match get_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_owner_entity(avatar_entity) {
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
    safe_call!(unsafe {
        let light_team = a2.LightTeam()?;
        let extra_team = a2.ExtraTeam()?;

        // Now process avatars
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
        for character in extra_team.to_vec::<RPG_GameCore_LineUpCharacter>() {
            let avatar_id = character.CharacterID()?;
            match helpers::get_avatar_from_id((*avatar_id).into()) {
                Ok(avatar) => avatars.push(avatar),
                Err(e) => errors.push(e),
            }
        }
 
        crate::ui::helpers::clear_avatar_buffers();
        crate::ui::helpers::clear_monster_buffers();
        crate::ui::helpers::clear_property_buffers();

        let avatar_ids: Vec<u32> = avatars.iter().map(|a| a.id).collect();
        crate::ui::helpers::populate_avatar_buffers(&avatar_ids);

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
    let res = ON_BATTLE_END_Detour.call(instance);
    BattleContext::handle_event(Ok(Event::OnBattleEnd));
    res
}

#[named]
fn on_turn_begin(instance: RPG_GameCore_TurnBasedGameMode) {
    // Update AV first
    let res = ON_TURN_BEGIN_Detour.call(instance);

    safe_call!(unsafe {
        let turn_owner = instance._CurrentTurnActionEntity()?;

        let entity_value: RPG_GameCore_EntityType = parse_il2cpp_enum(turn_owner._EntityType()?)?;

        match entity_value {
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
    let cycle = ON_UPDATE_CYCLE_Detour.call(instance);
    BattleContext::handle_event(Ok(Event::OnUpdateCycle(OnUpdateCycleEvent { cycle })));
    cycle
}

#[named]
fn handle_hp_change(turn_based_ability_component: RPG_GameCore_TurnBasedAbilityComponent) {
    use std::string::ToString;
    safe_call!(unsafe {
        let property_kind = RPG_GameCore_AbilityProperty::CurrentHP.to_string();
        let property_value = fixpoint_to_raw(
            &turn_based_ability_component.get_property(RPG_GameCore_AbilityProperty::CurrentHP)?
        );

        let entity = turn_based_ability_component.as_base()._OwnerRef()?;
        let entity_value: RPG_GameCore_EntityType = parse_il2cpp_enum(entity._EntityType()?)?;

        match entity_value {
            RPG_GameCore_EntityType::Avatar => {
                let e = match helpers::get_avatar_from_entity(entity) {
                    Ok(avatar) => Ok(Event::OnStatChange(OnStatChangeEvent {
                        entity: Entity {
                            uid: avatar.id,
                            team: Team::Player,
                        },
                        property: Property {
                            r#type: property_kind,
                            value: property_value,
                        },
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
                    property: Property {
                        r#type: property_kind,
                        value: property_value,
                    },
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
    a3: *const c_void,
) {
    let res = ON_DIRECT_CHANGE_HP_Detour.call(instance, a1, a2, a3);
    handle_hp_change(instance);
    res
}

#[named]
pub fn on_direct_damage_hp(
    instance: RPG_GameCore_TurnBasedAbilityComponent,
    a1: RPG_GameCore_FixPoint,
    a2: i32,
    a3: *const c_void,
    a4: RPG_GameCore_FixPoint,
    a5: *const c_void,
) {

    let hp_before_raw = unsafe { instance.get_property(RPG_GameCore_AbilityProperty::CurrentHP) }
        .ok()
        .map(|fp| fixpoint_to_raw(&fp))
        .unwrap_or(0.0);

    let res = ON_DIRECT_DAMAGE_HP_Detour.call(instance, a1, a2, a3, a4, a5);

    let hp_after_raw = unsafe { instance.get_property(RPG_GameCore_AbilityProperty::CurrentHP) }
        .ok()
        .map(|fp| fixpoint_to_raw(&fp))
        .unwrap_or(0.0);

    safe_call!(unsafe {
        let defender = instance.as_base()._OwnerRef()?;
        let defender_uid: u32 = (*defender._RuntimeID_k__BackingField()?).into();

        let pending = pending_damage_contexts()
            .lock()
            .ok()
            .and_then(|mut map| map.get_mut(&defender_uid).and_then(|q| q.pop_front()));

        if let Some(ctx) = pending {
            let hp_damage = (hp_before_raw - hp_after_raw).max(0.0);
            // Only emit finalized damage from observed post-mutation HP delta.
            // Falling back to raw getter here can duplicate hits when DirectDamageHP
            // is invoked multiple times for the same queued record.
            if hp_damage > 0.0 {
                let overkill_damage = if hp_after_raw <= 0.0 {
                    (ctx.raw_damage.max(hp_damage) - ctx.hp_before_raw).max(0.0)
                } else {
                    0.0
                };

                BattleContext::handle_event(Ok(Event::OnDamage(OnDamageEvent {
                    attacker: ctx.attacker,
                    damage: hp_damage,
                    overkill_damage,
                    r#type: ctx.r#type,
                })));
            }
        }

        Ok(())
    });

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
    let res = ON_STAT_CHANGE_Detour.call(instance, property, a2, new_stat, a4);
    safe_call!(unsafe {
        let entity = instance
            .as_base()
            ._OwnerRef()
            .context("on_stat_change: failed to read owner entity")?;
        let property_kind = property.to_string();
        let property_value = fixpoint_to_raw(&new_stat);
        let entity_value: RPG_GameCore_EntityType = parse_il2cpp_enum(
            entity
                ._EntityType()
                .context("on_stat_change: failed to read entity type box")?,
        )
        .context("on_stat_change: failed to parse entity type")?;

        match entity_value {
            RPG_GameCore_EntityType::Avatar => {
                let e = match helpers::get_avatar_from_entity(entity)
                    .context("on_stat_change: failed to resolve avatar from entity")
                {
                    Ok(avatar) => Ok(Event::OnStatChange(OnStatChangeEvent {
                        entity: Entity {
                            uid: avatar.id,
                            team: Team::Player,
                        },
                        property: Property {
                            r#type: property_kind.clone(),
                            value: property_value,
                        },
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
                    property: Property {
                        r#type: property_kind,
                        value: property_value,
                    },
                })));
            }
            _ => {}
        }
        Ok(())
    });
    res
}

use anyhow::Context;
use std::io::Cursor;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Threading::GetCurrentProcess;

static DAMAGE_GETTER_VA: OnceLock<usize> = OnceLock::new();
static DAMAGE_FIELD_OFFSET: OnceLock<Option<usize>> = OnceLock::new();

unsafe fn get_damage_getter(class: Il2CppClass) -> Result<usize> {
    if let Some(va) = DAMAGE_GETTER_VA.get() {
        return Ok(*va);
    }
    let getter = class
        .find_method("GBOAGIMFJCK", vec![])
        .context("Failed to find GBOAGIMFJCK damage getter")?;
    let va = getter.va() as usize;
    let _ = DAMAGE_GETTER_VA.set(va);
    DAMAGE_GETTER_VA
        .get()
        .copied()
        .ok_or_else(|| anyhow!("Failed to cache damage getter VA"))
}

unsafe fn read_damage_fixpoint(damage_info: *const c_void) -> Result<RPG_GameCore_FixPoint> {
    if damage_info.is_null() {
        return Err(anyhow!("damage_info pointer was null"));
    }
    let class_ptr = unsafe { *(damage_info as *const *const c_void) };
    if class_ptr.is_null() {
        return Err(anyhow!("damage_info class pointer was null"));
    }
    let getter_va = unsafe { get_damage_getter(Il2CppClass(class_ptr))? };
    let getter: unsafe extern "fastcall" fn(*const c_void) -> RPG_GameCore_FixPoint =
        unsafe { std::mem::transmute(getter_va as *const c_void) };
    Ok(unsafe { getter(damage_info) })
}

unsafe fn resolve_damage_field_offset() -> Result<usize> {
    let mut on_damage_method = None;
    for (key, class) in il2cpp_runtime::get_type_table()? {
        if is_obfuscated_name(key) {
            if let Ok(method) = class.find_method(
                "*",
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
                    "*",
                ],
            ) {
                on_damage_method = Some(method);
                break;
            }
        }
    }

    let target_fn = on_damage_method
        .ok_or_else(|| anyhow!("Failed to find on_damage method for damage offset extraction"))?
        .va();

    let buffer = vec![0u8; 0x300];
    let mut bytes_read = 0usize;
    let process_handle = unsafe { GetCurrentProcess() };
    unsafe {
        ReadProcessMemory(
            process_handle,
            target_fn,
            buffer.as_ptr() as _,
            buffer.len(),
            Some(&mut bytes_read),
        )
    }
    .context("Failed to read on_damage method bytes")?;

    static DAMAGE_PATTERN: &str = "48 8B 83 ? ? ? ? 48 89 84 24";
    let pattern_tokens = DAMAGE_PATTERN.split_whitespace().collect::<Vec<_>>();
    let disp_index = pattern_tokens
        .windows(4)
        .position(|w| w.iter().all(|token| *token == "?"))
        .ok_or_else(|| anyhow!("Damage pattern does not contain a 4-byte wildcard displacement"))?;

    let locs = patternscan::scan(Cursor::new(buffer), &DAMAGE_PATTERN)
        .context("Failed to scan for damage offset pattern")?;
    let addr = locs
        .first()
        .ok_or_else(|| anyhow!("Damage offset pattern not found in on_damage method"))?;

    let disp_ptr = unsafe { target_fn.byte_offset((addr + disp_index) as isize) } as *const u32;
    Ok(u32::from_le(unsafe { disp_ptr.read_unaligned() }) as usize)
}

unsafe fn get_damage_field_offset() -> Option<usize> {
    if let Some(offset) = DAMAGE_FIELD_OFFSET.get() {
        return *offset;
    }

    let offset = unsafe { resolve_damage_field_offset().ok() };
    let _ = DAMAGE_FIELD_OFFSET.set(offset);
    DAMAGE_FIELD_OFFSET.get().copied().flatten()
}

#[derive(Clone, Copy)]
struct EntityDefeatedOffsets {
    defeated_entity: usize,
    killer_entity: usize,
}

static ENTITY_DEFEATED_OFFSETS: OnceLock<EntityDefeatedOffsets> = OnceLock::new();

unsafe fn resolve_defeated_entity_offset() -> Result<EntityDefeatedOffsets> {
    // This should be enough
    let buffer = vec![0u8; 0x9A];
    let mut bytes_read = 0usize;
    let process_handle = unsafe { GetCurrentProcess() };
    let target_fn = RPG_GameCore_TurnBasedGameMode::get_class_static()?
        .find_method("_MakeLimboEntityDie", vec!["*"])?;
    // .va();

    unsafe {
        ReadProcessMemory(
            process_handle,
            target_fn.va(),
            buffer.as_ptr() as _,
            buffer.len(),
            Some(&mut bytes_read),
        )
    }
    .context("Failed to read module memory")?;

    // mov     rdx, [r15+??h]
    // mov     rcx, r14
    // call    RPG::GameCore::TurnBasedGameMode::CheckLimboEntityCanDie
    // let _method = RPG_GameCore_TurnBasedGameMode::get_class_static()?
    //     .find_method("_CheckLimboEntityCanDie", vec!["RPG.GameCore.GameEntity"])?;

    static PATTERN: &str = "49 8B 57 ? 4C 89 F1 E8 ? ? ? ?";
    let pattern_tokens = PATTERN.split_whitespace().collect::<Vec<_>>();
    let _call_opcode_index = pattern_tokens
        .iter()
        .position(|token| *token == "E8")
        .context("Pattern does not contain E8 call opcode")?;

    let locs =
        patternscan::scan(Cursor::new(buffer), &PATTERN).context("Failed to scan for pattern")?;
    let addr = locs
        .first()
        // .iter().map(|x| unsafe { target_fn.byte_offset(*x as _) })
        // // .find(|x| unsafe {
        // //     // Get call relative address
        // //     let rel_addr_ptr = (*x).byte_offset((call_opcode_index + 1) as isize) as *const i32;
        // //     let rel_addr = i32::from_le(rel_addr_ptr.read_unaligned()) as isize;
        // //     let opcode_addr = (*x).byte_offset(call_opcode_index as isize);
        // //     let call_addr = opcode_addr.byte_offset(rel_addr);
        // //     if call_addr == method.va() {
        // //         true
        // //     } else {
        // //         false
        // //     }
        // // })
        // .get
        .context("Could not resolve defeated entity offset")?;
    let addr = unsafe { target_fn.va().byte_offset(*addr as isize) };

    // Field offset is at ?? in the pattern, which is the address of the defeated entity.
    let defeated_entity_offset = unsafe { *((addr.wrapping_add(3)) as *const u8) as usize };

    let class = get_cached_class(target_fn.arg(0).name())?;
    let mut has_matching_offset = false;
    let mut alternate_entity_type_offset = None;

    let field_iter: *const c_void = null();
    loop {
        let field = il2cpp_class_get_fields(class, &field_iter);
        if field.0.is_null() {
            break;
        }

        let field_offset = il2cpp_field_get_offset(field) as usize;
        if field_offset == defeated_entity_offset {
            has_matching_offset = true;
        }

        let field_type = il2cpp_field_get_type(field);
        if field_type.name() == "RPG.GameCore.GameEntity" && field_offset != defeated_entity_offset
        {
            alternate_entity_type_offset = Some(field_offset);
        }
    }

    if !has_matching_offset {
        return Err(anyhow!(
            "Failed to match defeated entity field offset {:#x} against {} fields",
            defeated_entity_offset,
            class.name()
        ));
    }

    Ok(EntityDefeatedOffsets {
        defeated_entity: defeated_entity_offset,
        killer_entity: alternate_entity_type_offset
            .context("Failed to find alternate RPG.GameCore.EntityType field offset")?,
    })
}

unsafe fn get_entity_defeated_offsets() -> Result<EntityDefeatedOffsets> {
    ENTITY_DEFEATED_OFFSETS
        .get()
        .copied()
        .ok_or_else(|| anyhow!("Entity defeated offsets are not initialized"))
}

#[named]
pub fn on_entity_defeated(instance: RPG_GameCore_TurnBasedGameMode, a2: *const c_void) -> bool {
    let res = ON_ENTITY_DEFEATED_Detour.call(instance, a2);

    safe_call!(unsafe {
        let offsets = get_entity_defeated_offsets()?;

        let defeated_entity =
            *(a2.byte_offset(offsets.defeated_entity as isize) as *const RPG_GameCore_GameEntity);
        let killer_entity =
            *(a2.byte_offset(offsets.killer_entity as isize) as *const RPG_GameCore_GameEntity);

        let killer_entity_value: RPG_GameCore_EntityType =
            parse_il2cpp_enum(killer_entity._EntityType()?)?;

        let defeated_entity_alive_state_value: RPG_GameCore_AliveState =
            parse_il2cpp_enum(defeated_entity._AliveState()?)?;

        if res && defeated_entity_alive_state_value == RPG_GameCore_AliveState::Dying {
            if killer_entity_value == RPG_GameCore_EntityType::Avatar {
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
    let res = ON_UPDATE_TEAM_FORMATION_Detour.call(instance);
    safe_call!({
        let team_value: RPG_GameCore_TeamType = parse_il2cpp_enum(instance._Team()?)?;

        if team_value == RPG_GameCore_TeamType::TeamDark {
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
    let res = ON_INITIALIZE_ENEMY_Detour.call(instance, turn_based_ability_component);
    safe_call!({
        let row_data = instance._MonsterRowData()?;
        let row = row_data._Row()?;
        let monster_template_id = unsafe { instance.get_monster_template_id()? };
        let mut base_stats = BattleStats {
            properties: HashMap::new(),
        };
        base_stats.set_value(RPG_GameCore_AbilityProperty::Level.to_string(), unsafe { row_data.get_Level()? } as f64);
        base_stats.set_value(RPG_GameCore_AbilityProperty::MaxHP.to_string(), fixpoint_to_raw(&*instance._DefaultMaxHP()?));
        base_stats.set_value(RPG_GameCore_AbilityProperty::CurrentHP.to_string(), fixpoint_to_raw(&*instance._DefaultMaxHP()?));

        let name_id = row.MonsterName()?;
        let monster_name = get_textmap_content(&name_id)?;
        let entity = instance._OwnerRef()?;

        
        BattleContext::handle_event(Ok(Event::OnInitializeEnemy(OnInitializeEnemyEvent {
            enemy: Enemy {
                id: monster_template_id,
                uid: (*entity._RuntimeID_k__BackingField().unwrap()).into(),
                name: (*monster_name).to_string(),
                base_stats,
            },
        })));
        crate::ui::helpers::cache_monster_buffer(monster_template_id);
        Ok(())
    });
    res
}

retour::static_detour! {
    static ON_DAMAGE_Detour: fn(*const c_void, *const c_void, *const c_void, RPG_GameCore_TurnBasedAbilityComponent, RPG_GameCore_TurnBasedAbilityComponent, RPG_GameCore_GameEntity, RPG_GameCore_GameEntity, RPG_GameCore_GameEntity, bool, *const c_void) -> bool;
    static ON_COMBO_Detour: fn(*const c_void, RPG_GameCore_TurnBasedGameMode);
    static ON_USE_SKILL_Detour: fn(RPG_GameCore_SkillCharacterComponent, i32, *const c_void, bool, *const c_void, *const c_void, i32) -> bool;
    static ON_SET_LINEUP_Detour: fn(RPG_GameCore_BattleInstance, *const c_void, RPG_GameCore_BattleLineupData, i32, u32, bool);
    static ON_BATTLE_BEGIN_Detour: fn(RPG_GameCore_TurnBasedGameMode);
    static ON_BATTLE_END_Detour: fn(RPG_GameCore_TurnBasedGameMode);
    static ON_TURN_BEGIN_Detour: fn(RPG_GameCore_TurnBasedGameMode);
    static ON_TURN_END_Detour: fn(RPG_GameCore_TurnBasedAbilityComponent, i32);
    static ON_UPDATE_WAVE_Detour: fn(RPG_GameCore_TurnBasedGameMode);
    static ON_UPDATE_CYCLE_Detour: fn(RPG_GameCore_TurnBasedGameMode) -> u32;
    static ON_DIRECT_CHANGE_HP_Detour: fn(RPG_GameCore_TurnBasedAbilityComponent, i32, RPG_GameCore_FixPoint, *const c_void);
    static ON_DIRECT_DAMAGE_HP_Detour: fn(RPG_GameCore_TurnBasedAbilityComponent, RPG_GameCore_FixPoint, i32, *const c_void, RPG_GameCore_FixPoint, *const c_void);
    static ON_STAT_CHANGE_Detour: fn(RPG_GameCore_TurnBasedAbilityComponent, RPG_GameCore_AbilityProperty, i32, RPG_GameCore_FixPoint, *const c_void) -> bool;
    static ON_ENTITY_DEFEATED_Detour: fn(RPG_GameCore_TurnBasedGameMode, *const c_void) -> bool;
    static ON_UPDATE_TEAM_FORMATION_Detour: fn(RPG_GameCore_TeamFormationComponent);
    static ON_INITIALIZE_ENEMY_Detour: fn(RPG_GameCore_MonsterDataComponent, RPG_GameCore_TurnBasedAbilityComponent);
}

pub fn subscribe() -> Result<()> {
    unsafe {
        // Resolve and hook old on_damage path so all damage categories are captured.
        let mut on_damage_method = None;
        for (key, class) in il2cpp_runtime::get_type_table()? {
            if is_obfuscated_name(key) {
                if let Ok(method) = class.find_method(
                    "*",
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
                        "*",
                    ],
                ) {
                    on_damage_method = Some(method);
                    break;
                }
            }
        }

        let on_damage_method =
            on_damage_method.ok_or_else(|| anyhow!("Failed to find on_damage method"))?;

        // Prewarm damage class metadata from arg 2 (the damage info struct)
        let damage_info_class = on_damage_method.arg(2).class();
        get_attack_type_offset(damage_info_class)
            .context("Failed to resolve AttackType field offset")?;
        get_damage_getter(damage_info_class)
            .context("Failed to resolve damage getter (GBOAGIMFJCK)")?;
        let _ = get_damage_field_offset();

        subscribe_function!(ON_DAMAGE_Detour, on_damage_method.va(), on_damage)
            .context("Failed to initialize on_damage detour")?;

        // Resolve on_combo
        let mut combo_instance_class = None;
        let mut on_combo_method = None;
        let field_iter: *const c_void = null();
        loop {
            let field = il2cpp_class_get_fields(
                get_cached_class("RPG.GameCore.LevelSingleInsertAbilityFinishOrAbort")?,
                &field_iter,
            );
            if field.0.is_null() {
                break;
            }
            let field_name = il2cpp_runtime::utils::cstr_to_str(il2cpp_field_get_name(field));
            if field_name == "<TurnInsertAbilityInstance>k__BackingField" {
                let field_type = il2cpp_field_get_type(field);
                if let Ok(method) = field_type
                    .class()
                    .find_method("*", vec!["RPG.GameCore.TurnBasedGameMode"])
                {
                    combo_instance_class = Some(field_type.class());
                    on_combo_method = Some(method);
                    break;
                }
            }
        }

        if let Some(method) = on_combo_method {
            subscribe_function!(ON_COMBO_Detour, method.va(), on_combo)
                .context("Failed to initialize on_combo detour")?;
        } else {
            return Err(anyhow!("Failed to find on_combo method"));
        }

        if let Some(class) = combo_instance_class {
            get_combo_field_offsets(class).context("Failed to resolve on_combo field offsets")?;
        }

        let defeated_offsets =
            resolve_defeated_entity_offset().context("Failed to resolve entity-defeated offsets")?;
        let _ = ENTITY_DEFEATED_OFFSETS.set(defeated_offsets);
        get_entity_defeated_offsets().context("Failed to cache entity-defeated offsets")?;

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
                        "*"
                    ],
                )?
                .va(),
            on_direct_change_hp
        )?;
        subscribe_function!(
            ON_DIRECT_DAMAGE_HP_Detour,
            RPG_GameCore_TurnBasedAbilityComponent::get_class_static()?
                .find_method(
                    "DirectDamageHP",
                    vec![
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

