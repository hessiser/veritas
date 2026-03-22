use std::{borrow::Cow, collections::HashMap, ptr::null, sync::LazyLock};

use crate::{
    kreide::types::{
        RPG_Client_AvatarData, RPG_Client_GlobalVars, RPG_Client_ModuleManager,
        RPG_Client_UIGameEntityUtils, RPG_GameCore_MonsterDataComponent,
        RPG_GameCore_ServantDataComponent,
    },
    models::misc::{Avatar, Skill},
};
use anyhow::{Context, Result, anyhow};
use function_name::named;
use il2cpp_runtime::System_RuntimeType;

use super::types::{
    RPG_Client_TextID, RPG_Client_TextmapStatic, RPG_GameCore_AbilityProperty,
    RPG_GameCore_BattleInstance, RPG_GameCore_FixPoint, RPG_GameCore_GameEntity,
    RPG_GameCore_SkillData, RPG_GameCore_TurnBasedAbilityComponent,
};

pub fn get_textmap_content(hash: &RPG_Client_TextID) -> Result<String> {
    Ok(RPG_Client_TextmapStatic::get_text(hash, null()).map(|s| s.to_string())?)
}

fn sanitize_entity_name(name: &str) -> Cow<'_, str> {
    if !name.contains("<ub>") && !name.contains("</ub>") {
        return Cow::Borrowed(name);
    }

    Cow::Owned(name.replace("<ub>", "").replace("</ub>", ""))
}

#[named]
pub fn get_module_manager() -> Result<RPG_Client_ModuleManager> {
    log::debug!(function_name!());
    Ok(RPG_Client_GlobalVars::s_ModuleManager()?)
}

#[named]
fn get_avatar_data_from_id(avatar_id: u32) -> Result<RPG_Client_AvatarData> {
    log::debug!(function_name!());
    let s_module_manager = get_module_manager()?;
    let avatar_module = s_module_manager.AvatarModule()?;
    Ok(avatar_module.get_avatar(avatar_id)?)
}

#[named]
pub unsafe fn get_avatar_from_id(avatar_id: u32) -> Result<Avatar> {
    log::debug!(function_name!());

    let avatar_data = get_avatar_data_from_id(avatar_id)
        .context(format!("AvatarData with id {avatar_id} was null"))?;

    let avatar_name = avatar_data
        .AvatarName()
        .map(|name| name.to_string())
        .unwrap_or_default();

    Ok(Avatar {
        id: avatar_id,
        name: sanitize_entity_name(avatar_name.as_ref()).into_owned(),
    })
}

#[named]
pub unsafe fn get_skill_from_skilldata(skill_data: RPG_GameCore_SkillData) -> Result<Skill> {
    log::debug!(function_name!());

    if skill_data.0.is_null() {
        return Err(anyhow!("SkillData was null"));
    }

    let row_data = skill_data.RowData().context("SkillData RowData was null")?;

    let text_id = row_data.get_SkillName().context("Skill name was null")?;

    let skill_type = row_data.get_AttackType()?;

    Ok(Skill {
        name: get_textmap_content(&text_id)?,
        skill_type: skill_type as isize,
        skill_config_id: isize::try_from(*skill_data.SkillConfigID()?)?,
    })
}

#[named]
pub unsafe fn get_avatar_from_entity(entity: RPG_GameCore_GameEntity) -> Result<Avatar> {
    log::debug!(function_name!());

    if entity.0.is_null() {
        return Err(anyhow!("Avatar entity was null"));
    }

    let id = RPG_Client_UIGameEntityUtils::get_avatar_id(entity)
        .context("Failed to get AvatarID from GameEntity")?;

    let avatar_data =
        get_avatar_data_from_id(id).context(format!("AvatarData with id {id} was null"))?;

    let name = avatar_data
        .AvatarName()
        .map(|name| name.to_string())
        .unwrap_or_default();

    Ok(Avatar {
        id,
        name: sanitize_entity_name(name.as_ref()).into_owned(),
    })
}

#[named]
pub unsafe fn get_avatar_from_servant_entity(entity: RPG_GameCore_GameEntity) -> Result<Avatar> {
    log::debug!(function_name!());

    if entity.0.is_null() {
        return Err(anyhow!("Servant Entity was null"));
    }

    let battle_instance = entity
        ._OwnerWorldRef()?
        ._BattleInstanceRef_k__BackingField()?;

    let entity_manager = battle_instance._GameWorld()?._EntityManager()?;
    let avatar_entity = entity_manager.get_entity_summoner(entity)?;
    unsafe { get_avatar_from_entity(avatar_entity) }
}

#[named]
pub unsafe fn get_monster_from_entity(entity: RPG_GameCore_GameEntity) -> Result<Avatar> {
    log::debug!(function_name!());
    let monster_data_comp = RPG_GameCore_MonsterDataComponent(
        entity
            .get_component(System_RuntimeType::from_name(
                "RPG.GameCore.MonsterDataComponent",
            )?)?
            .0,
    );

    if monster_data_comp.0.is_null() {
        return Err(anyhow!("entity does not have MonsterDataComponent!"));
    }

    let monster_name = monster_data_comp._MonsterRowData()?._Row()?.MonsterName()?;

    let monster_id = monster_data_comp.get_monster_id()?;

    Ok(Avatar {
        id: monster_id,
        name: sanitize_entity_name(get_textmap_content(&*monster_name)?.as_ref()).into_owned(),
    })
}

#[named]
pub unsafe fn get_servant_from_entity(entity: RPG_GameCore_GameEntity) -> Result<Avatar> {
    log::debug!(function_name!());
    let servant_data_comp = RPG_GameCore_ServantDataComponent(
        entity
            .get_component(System_RuntimeType::from_name(
                "RPG.GameCore.ServantDataComponent",
            )?)?
            .0,
    );

    if servant_data_comp.0.is_null() {
        return Err(anyhow!("entity does not have ServantDataComponent!"));
    }

    let servant_row = servant_data_comp._ServantRowData()?._Row()?;

    Ok(Avatar {
        id: u32::try_from(*servant_row.ServantID()?)?,
    name: sanitize_entity_name(get_textmap_content(&*servant_row.ServantName()?)?.as_ref())
        .into_owned(),
    })
}

// #[named]
// pub unsafe fn get_entity_modifiers(entity: RPG_GameCore_GameEntity) -> Result<Vec<Value>> {
//     log::debug!(function_name!());
//     let ability_comp = RPG_GameCore_AbilityComponent(
//         entity
//             .get_component(System_RuntimeType::from_name("RPG.GameCore.AbilityComponent")?)?
//             .0,
//     );

//     if ability_comp.0.is_null() {
//         return Err(anyhow!("entity does not have AbilityComponent!"));
//     }

//     let modifier_list = List(ability_comp._ModifierList()?.0);
//     let modifier_list_array = modifier_list.to_vec::<RPG_GameCore_TurnBasedModifierInstance>();

//     Ok(modifier_list_array
//         .iter()
//         .filter_map(|obj| {
//             let status_config_key = obj.get_key_for_status_config().ok()?;

//             let status_row =
//                 RPG_GameCore_StatusExcelTable::get_by_modifier_name(status_config_key).ok()?;

//             Some(if status_row.is_null() {
//                 json!({
//                     "key": status_config_key.as_str(),
//                 })
//             } else {
//                 json!({
//                     "key": status_config_key.as_str(),
//                     "desc": get_textmap_content(&status_row.StatusDesc().ok()?),
//                     "name": get_textmap_content(&status_row.StatusName().ok()?),
//                 })
//             })
//         })
//         .collect::<Vec<_>>())
// }

pub unsafe fn get_entity_ability_properties(
    entity: RPG_GameCore_GameEntity,
) -> Result<HashMap<String, f64>> {
    let ability_comp = RPG_GameCore_TurnBasedAbilityComponent(
        entity
            .get_component(System_RuntimeType::from_name(
                "RPG.GameCore.TurnBasedAbilityComponent",
            )?)?
            .0,
    );

    if ability_comp.0.is_null() {
        return Err(anyhow!("entity does not have TurnBasedAbilityComponent!"));
    }

    Ok((0..=193)
        .filter_map(|i| {
            let property_enum =
                unsafe { std::mem::transmute::<i32, RPG_GameCore_AbilityProperty>(i) };
            let value = fixpoint_to_raw(&ability_comp.get_property(property_enum).ok()?);

            (value != 0.0).then_some((format!("{property_enum:?}"), value))
        })
        .collect::<HashMap<String, f64>>())
}

#[named]
pub unsafe fn get_monster_from_runtime_id(
    id: u32,
    battle_instance: RPG_GameCore_BattleInstance,
) -> Result<Avatar> {
    log::debug!(function_name!());
    unsafe {
        get_monster_from_entity(
            battle_instance
                ._GameWorld()?
                ._EntityManager()?
                .get_entity_by_runtime_id(id)?,
        )
    }
}

#[named]
pub fn fixpoint_to_raw(fixpoint: &RPG_GameCore_FixPoint) -> f64 {
    log::debug!(function_name!());
    static FLOAT_CONVERSION_CONSTANT: LazyLock<f64> = LazyLock::new(|| 1f64 / 2f64.powf(32f64));
    let raw_value = fixpoint.m_rawValue;
    let hi = ((raw_value as u64 & 0xFFFFFFFF00000000) >> 32) as u32;
    let lo = (raw_value as u64 & 0x00000000FFFFFFFF) as u32;
    hi as f64 + lo as f64 * *FLOAT_CONVERSION_CONSTANT
}
