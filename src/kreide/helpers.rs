use std::{collections::HashMap, ptr::null, sync::{LazyLock, Mutex, OnceLock}};
use il2cpp_runtime::api::il2cpp_method_get_return_type;

use crate::{
    kreide::types::{
        RPG_Client_CachedAssetLoader, RPG_Client_UIGameEntityUtils, RPG_GameCore_AvatarExcelTable, RPG_GameCore_AvatarPropertyExcelTable, RPG_GameCore_AvatarPropertyType__Boxed, RPG_GameCore_BattleEventDataComponent, RPG_GameCore_EntityType, RPG_GameCore_GameComponentBase, RPG_GameCore_MonsterDataComponent, RPG_GameCore_MonsterTemplateExcelTable, RPG_GameCore_ServantDataComponent, UnityEngine_Graphics, UnityEngine_ImageConversion, UnityEngine_Rect, UnityEngine_RenderTexture, UnityEngine_Sprite, UnityEngine_Texture2D
    },
    models::types::{Avatar, Skill},
};
use anyhow::{Context, Result, anyhow};
use function_name::named;
use il2cpp_runtime::{
    Il2CppObject, System_RuntimeType, get_cached_class,
    types::{Il2CppString, System_Enum, System_Int32__Boxed, System_Type},
};

use super::types::{
    RPG_Client_TextID, RPG_Client_TextmapStatic,
    RPG_GameCore_BattleInstance, RPG_GameCore_FixPoint, RPG_GameCore_FixPoint__Boxed, RPG_GameCore_GameEntity,
    RPG_GameCore_SkillData,
};

fn sanitize_entity_name<S: AsRef<str>>(name: S) -> String {
    let name = name.as_ref();
    if !name.contains("<ub>") && !name.contains("</ub>") {
        return name.to_string();
    }

    name.replace("<ub>", "").replace("</ub>", "")
}

static AVATAR_CACHE: LazyLock<Mutex<HashMap<u32, Avatar>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

unsafe fn get_component_by_name(
    entity: RPG_GameCore_GameEntity,
    type_name: &str,
) -> Result<RPG_GameCore_GameComponentBase> {
    let class = get_cached_class(type_name)?;
    Ok(unsafe { entity.get_component_by_type_handle(class.byval_arg())? })
}

pub fn get_textmap_content(hash: &RPG_Client_TextID) -> Result<String> {
    Ok(unsafe { RPG_Client_TextmapStatic::get_text(hash, null()) }.map(|s| s.to_string())?)
}

#[named]
pub unsafe fn get_avatar_from_id(avatar_id: u32) -> Result<Avatar> {
    log::debug!(function_name!());

    if let Some(avatar) = AVATAR_CACHE
        .lock()
        .ok()
        .and_then(|cache| cache.get(&avatar_id).cloned())
    {
        return Ok(avatar);
    }

    let data = unsafe { RPG_GameCore_AvatarExcelTable::GetData(avatar_id)? };
    let avatar_name = get_textmap_content(&*data.AvatarName()?)?;

    let avatar = Avatar {
        id: avatar_id,
        name: sanitize_entity_name(avatar_name),
    };

    if let Ok(mut cache) = AVATAR_CACHE.lock() {
        cache.insert(avatar_id, avatar.clone());
    }

    Ok(avatar)
}

#[named]
pub unsafe fn get_skill_from_skilldata(skill_data: RPG_GameCore_SkillData) -> Result<Skill> {
    log::debug!(function_name!());

    if skill_data.0.is_null() {
        return Err(anyhow!("SkillData was null"));
    }

    let row_data = skill_data.RowData()?;

    let text_id = unsafe { row_data.get_SkillName()? };

    let skill_type = unsafe {
        row_data.get_AttackType()?.to_string()
    };

    Ok(Skill {
        name: get_textmap_content(&text_id)?,
        skill_type,
        skill_config_id: isize::try_from(*skill_data.SkillConfigID()?)?,
    })
}

#[named]
pub unsafe fn get_avatar_from_entity(entity: RPG_GameCore_GameEntity) -> Result<Avatar> {
    log::debug!(function_name!());

    if entity.0.is_null() {
        return Err(anyhow!("Avatar entity was null"));
    }

    let id = unsafe { RPG_Client_UIGameEntityUtils::get_avatar_id(entity) }
        .context("Failed to get AvatarID from GameEntity")?;
    unsafe { get_avatar_from_id(id) }
}

#[named]
pub unsafe fn get_avatar_from_owner_entity(entity: RPG_GameCore_GameEntity) -> Result<Avatar> {
    log::debug!(function_name!());

    if entity.0.is_null() {
        return Err(anyhow!("Owner entity was null"));
    }

    match *entity._EntityType()? {
        RPG_GameCore_EntityType::Avatar => unsafe { get_avatar_from_entity(entity) },
        RPG_GameCore_EntityType::Servant | RPG_GameCore_EntityType::Snapshot => {
            unsafe { get_avatar_from_servant_entity(entity) }
        }
        RPG_GameCore_EntityType::BattleEvent => {
            let battle_event_data_comp = RPG_GameCore_BattleEventDataComponent(
                unsafe { get_component_by_name(entity, "RPG.GameCore.BattleEventDataComponent")? }.0,
            );

            if battle_event_data_comp.0.is_null() {
                return Err(anyhow!("entity does not have BattleEventDataComponent!"));
            }

            let source_caster = battle_event_data_comp._SourceCaster_k__BackingField()?;
            if source_caster.0.is_null() || source_caster.0 == entity.0 {
                return Err(anyhow!("BattleEvent source caster was null"));
            }

            unsafe { get_avatar_from_owner_entity(source_caster) }
        }
        _ => unsafe { get_avatar_from_entity(entity) },
    }
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
    let avatar_entity = unsafe { entity_manager.get_entity_summoner(entity)? };
    unsafe { get_avatar_from_entity(avatar_entity) }
}

#[named]
pub unsafe fn get_monster_from_entity(entity: RPG_GameCore_GameEntity) -> Result<Avatar> {
    log::debug!(function_name!());
    let monster_data_comp = RPG_GameCore_MonsterDataComponent(
        unsafe { get_component_by_name(entity, "RPG.GameCore.MonsterDataComponent")? }.0,
    );

    if monster_data_comp.0.is_null() {
        return Err(anyhow!("entity does not have MonsterDataComponent!"));
    }

    let monster_name = monster_data_comp._MonsterRowData()?._Row()?.MonsterName()?;

    let monster_template_id = unsafe { monster_data_comp.get_monster_template_id()? };

    Ok(Avatar {
        id: monster_template_id,
        name: sanitize_entity_name(get_textmap_content(&*monster_name)?),
    })
}

#[named]
pub unsafe fn get_servant_from_entity(entity: RPG_GameCore_GameEntity) -> Result<Avatar> {
    log::debug!(function_name!());
    let servant_data_comp = RPG_GameCore_ServantDataComponent(
        unsafe { get_component_by_name(entity, "RPG.GameCore.ServantDataComponent")? }.0,
    );

    if servant_data_comp.0.is_null() {
        return Err(anyhow!("entity does not have ServantDataComponent!"));
    }

    let servant_row = servant_data_comp._ServantRowData()?._Row()?;

    Ok(Avatar {
        id: u32::try_from(*servant_row.ServantID()?)?,
        name: sanitize_entity_name(get_textmap_content(&*servant_row.ServantName()?)?),
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

// pub unsafe fn get_entity_ability_properties(
//     entity: RPG_GameCore_GameEntity,
// ) -> Result<HashMap<String, f64>> {
//     let ability_comp = RPG_GameCore_TurnBasedAbilityComponent(
//         unsafe {
//             entity.get_component(System_RuntimeType::from_name(
//                 "RPG.GameCore.TurnBasedAbilityComponent",
//             )?)?
//         }
//         .0,
//     );

//     if ability_comp.0.is_null() {
//         return Err(anyhow!("entity does not have TurnBasedAbilityComponent!"));
//     }

//     Ok((0..=193)
//         .filter_map(|i| {
//             let property_enum =
//                 unsafe { std::mem::transmute::<i32, RPG_GameCore_AbilityProperty>(i) };
//             let value = fixpoint_to_raw(&unsafe { ability_comp.get_property(property_enum).ok()? });

//             (value != 0.0).then_some((format!("{property_enum:?}"), value))
//         })
//         .collect::<HashMap<String, f64>>())
// }

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

static FIXPOINT_TO_DOUBLE_VA: OnceLock<usize> = OnceLock::new();

fn get_fixpoint_op_explicit_double_va() -> Result<usize> {
    if let Some(va) = FIXPOINT_TO_DOUBLE_VA.get() {
        return Ok(*va);
    }
    let method = get_cached_class("RPG.GameCore.FixPoint")?
        .methods()
        .into_iter()
        .find(|m| {
            if m.name() != "op_Explicit" || m.args_cnt() != 1 {
                return false;
            }
            if m.arg_type_formatted(0) != "RPG.GameCore.FixPoint" {
                return false;
            }
            let ret = il2cpp_method_get_return_type(*m).alias_name();
            ret == "double" || ret == "System.Double"
        })
        .ok_or_else(|| anyhow!("op_Explicit(FixPoint)->double not found on RPG.GameCore.FixPoint"))?;

    let va = method.va() as usize;
    let _ = FIXPOINT_TO_DOUBLE_VA.set(va);
    Ok(va)
}
#[named]
pub fn fixpoint_to_raw(fixpoint: &RPG_GameCore_FixPoint) -> f64 {
    log::debug!(function_name!());
    match get_fixpoint_op_explicit_double_va() {
        Ok(va) => {
            let op_explicit: unsafe extern "fastcall" fn(RPG_GameCore_FixPoint) -> f64 =
                unsafe { std::mem::transmute(va as *const ()) };
            unsafe { op_explicit(*fixpoint) }
        }
        Err(e) => {
            log::error!("fixpoint_to_raw: op_Explicit(FixPoint)->double VA unavailable: {e}");
            0.0
        }
    }
}

pub fn is_obfuscated_name<S: AsRef<str>>(name: S) -> bool {
    let name = name.as_ref();
    name.len() == 11 && name.chars().all(|c| c.is_ascii_uppercase())
}

pub fn get_type_handle<S: AsRef<str>>(type_name: S) -> Result<System_Type> {
    let type_name = type_name.as_ref();
    let runtime_type = System_RuntimeType::from_name(type_name)?;
    let ty = runtime_type.get_il2cpp_type();
    Ok(unsafe { System_Type::get_type_from_handle(ty)? })
}

/// Common texture rendering pipeline: texture → render target → readable texture → PNG bytes
unsafe fn render_texture_to_png_bytes(tex: UnityEngine_Texture2D) -> Result<Vec<u8>> {
    unsafe {
        // RenderTextureFormat.Default = 7, RenderTextureReadWrite.Linear = 1
        let default_format: i32 = 7;
        let rw_format: i32 = 1;

        let render_tex = UnityEngine_RenderTexture::GetTemporary(
            tex.as_base().get_width()?,
            tex.as_base().get_height()?,
            0,
            default_format,
            rw_format,
        )?;
        UnityEngine_Graphics::Blit(tex, render_tex)?;
        let prev = UnityEngine_RenderTexture::GetActive()?;
        UnityEngine_RenderTexture::set_active(render_tex)?;

        use il2cpp_runtime::api::il2cpp_object_new;
        let readable_tex = UnityEngine_Texture2D(il2cpp_object_new(get_cached_class(
            UnityEngine_Texture2D::ffi_name(),
        )?));

        readable_tex.new(tex.as_base().get_width()?, tex.as_base().get_height()?)?;
        readable_tex.read_pixels(
            UnityEngine_Rect {
                x: 0.,
                y: 0.,
                width: render_tex.get_width()? as f32,
                height: render_tex.get_height()? as f32,
            },
            0,
            0,
        )?;
        readable_tex.apply()?;
        UnityEngine_RenderTexture::set_active(prev)?;
        UnityEngine_RenderTexture::ReleaseTemporary(render_tex)?;

        let array = UnityEngine_ImageConversion::EncodeToPNG(readable_tex)?;
        Ok(array.to_vec::<u8>())
    }
}

unsafe fn load_texture_from_asset(asset_path: Il2CppString) -> Result<UnityEngine_Texture2D> {
    if let Ok(sprite_type) = get_type_handle(UnityEngine_Sprite::ffi_name()) {
        if let Ok(sprite_obj) =
            unsafe { RPG_Client_CachedAssetLoader::SyncLoadAsset(asset_path, sprite_type, false) }
        {
            if !sprite_obj.0.is_null() {
                let sprite = UnityEngine_Sprite(sprite_obj.0);
                if let Ok(tex) = unsafe { sprite.get_texture() } {
                    if !tex.0.is_null() {
                        return Ok(tex);
                    }
                }
            }
        }
    }

    let texture_type = get_type_handle(UnityEngine_Texture2D::ffi_name())?;
    let texture_obj =
        unsafe { RPG_Client_CachedAssetLoader::SyncLoadAsset(asset_path, texture_type, false) }?;
    if texture_obj.0.is_null() {
        return Err(anyhow!("SyncLoadAsset returned null for both Sprite and Texture2D"));
    }

    Ok(UnityEngine_Texture2D(texture_obj.0))
}

pub fn get_monster_png_bytes(monster_id: u32) -> Result<Vec<u8>> {
    unsafe {
        let monster_row = RPG_GameCore_MonsterTemplateExcelTable::GetData(monster_id)?;
        let tex = load_texture_from_asset(monster_row.RoundIconPath()?)?;

        render_texture_to_png_bytes(tex)
    }
}

pub fn get_avatar_png_bytes(avatar_id: u32) -> Result<Vec<u8>> {
    unsafe {
        let avatar_row = RPG_GameCore_AvatarExcelTable::GetData(avatar_id)?;
        log::info!(
            "Support Avatar: {}, Icon Path: {}",
            avatar_id,
            avatar_row.AvatarSideIconPath()?.to_string()
        );

        let tex = load_texture_from_asset(avatar_row.AvatarSideIconPath()?)?;

        render_texture_to_png_bytes(tex)
    }
}

pub fn get_property_icon_png_bytes(property_name: &str) -> Result<Vec<u8>> {
    unsafe {
        let property_type = RPG_GameCore_AvatarPropertyType__Boxed(System_Enum::parse(
            get_type_handle("RPG.GameCore.AvatarPropertyType")?,
            Il2CppString::new(property_name)?,
        )?);
        
        let row = RPG_GameCore_AvatarPropertyExcelTable::GetData(*property_type)?;
        let icon_path = row.IconPath()?;

        let tex = load_texture_from_asset(icon_path)?;

        render_texture_to_png_bytes(tex)
    }
}
