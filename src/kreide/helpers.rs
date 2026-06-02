use std::{ptr::null, sync::LazyLock};

use crate::{
    kreide::types::{
        RPG_Client_AvatarHelper, RPG_Client_CachedAssetLoader, RPG_Client_UIGameEntityUtils, RPG_GameCore_AttackType__Boxed, RPG_GameCore_AvatarExcelTable, RPG_GameCore_AvatarPropertyExcelTable, RPG_GameCore_AvatarPropertyType__Boxed, RPG_GameCore_AvatarRow, RPG_GameCore_MonsterDataComponent, RPG_GameCore_MonsterTemplateExcelTable, RPG_GameCore_ServantDataComponent, UnityEngine_Graphics, UnityEngine_ImageConversion, UnityEngine_Rect, UnityEngine_RenderTexture, UnityEngine_Sprite, UnityEngine_Texture2D
    },
    models::misc::{Avatar, Skill},
};
use anyhow::{Context, Result, anyhow};
use function_name::named;
use il2cpp_runtime::{
    Il2CppObject, System_RuntimeType, get_cached_class,
    types::{Il2CppString, System_Enum, System_Int32__Boxed, System_Type},
};

use super::types::{
    RPG_Client_TextID, RPG_Client_TextmapStatic,
    RPG_GameCore_BattleInstance, RPG_GameCore_GameEntity,
    RPG_GameCore_SkillData,
};

fn sanitize_entity_name<S: AsRef<str>>(name: S) -> String {
    let name = name.as_ref();
    if !name.contains("<ub>") && !name.contains("</ub>") {
        return name.to_string();
    }

    name.replace("<ub>", "").replace("</ub>", "")
}

pub fn get_textmap_content(hash: &RPG_Client_TextID) -> Result<String> {
    Ok(unsafe { RPG_Client_TextmapStatic::get_text(hash, null()) }.map(|s| s.to_string())?)
}


#[named]
pub fn get_avatar_data_from_id(avatar_id: u32) -> Result<RPG_GameCore_AvatarRow> {
    log::debug!(function_name!());
    Ok(unsafe { RPG_GameCore_AvatarExcelTable::GetData(avatar_id)? })
}

#[named]
pub unsafe fn get_avatar_from_id(avatar_id: u32) -> Result<Avatar> {
    log::debug!(function_name!());

    let avatar_data = get_avatar_data_from_id(avatar_id)
        .context(format!("AvatarData with id {avatar_id} was null"))?;

    let avatar_name = unsafe { RPG_Client_AvatarHelper::GetAvatarName(avatar_id)?.to_string() };

    Ok(Avatar {
        id: avatar_id,
        name: sanitize_entity_name(avatar_name),
    })
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
        let boxed = RPG_GameCore_AttackType__Boxed(System_Enum::to_object_from_int(
            get_type_handle("RPG.GameCore.AttackType")?,
            row_data.get_AttackType()? as i32,
        )?);
        System_Enum::get_name(get_type_handle("RPG.GameCore.AttackType")?, boxed.0)?.to_string()
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

    let avatar_data =
        get_avatar_data_from_id(id).context(format!("AvatarData with id {id} was null"))?;

    let name = unsafe {
        RPG_Client_AvatarHelper::GetAvatarName(id)?.to_string()
    };

    Ok(Avatar {
        id,
        name: sanitize_entity_name(name),
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
    let avatar_entity = unsafe { entity_manager.get_entity_summoner(entity)? };
    unsafe { get_avatar_from_entity(avatar_entity) }
}

#[named]
pub unsafe fn get_monster_from_entity(entity: RPG_GameCore_GameEntity) -> Result<Avatar> {
    log::debug!(function_name!());
    let monster_data_comp = RPG_GameCore_MonsterDataComponent(
        unsafe {
            entity.get_component(System_RuntimeType::from_name(
                "RPG.GameCore.MonsterDataComponent",
            )?)?
        }
        .0,
    );

    if monster_data_comp.0.is_null() {
        return Err(anyhow!("entity does not have MonsterDataComponent!"));
    }

    let monster_name = monster_data_comp._MonsterRowData()?._Row()?.MonsterName()?;

    let monster_id = unsafe { monster_data_comp.get_monster_id()? };

    Ok(Avatar {
        id: monster_id,
        name: sanitize_entity_name(get_textmap_content(&*monster_name)?),
    })
}

#[named]
pub unsafe fn get_servant_from_entity(entity: RPG_GameCore_GameEntity) -> Result<Avatar> {
    log::debug!(function_name!());
    let servant_data_comp = RPG_GameCore_ServantDataComponent(
        unsafe {
            entity.get_component(System_RuntimeType::from_name(
                "RPG.GameCore.ServantDataComponent",
            )?)?
        }
        .0,
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

/// Extract render texture formats for texture-to-PNG conversion
unsafe fn get_render_texture_formats() -> Result<(i32, i32)> {
    unsafe {
        let default_format = {
            let value = System_Int32__Boxed(System_Enum::parse(
                get_type_handle("UnityEngine.RenderTextureFormat")?,
                Il2CppString::new("Default")?,
            )?);
            (*value).0
        };

        let rw_format = {
            let value = System_Int32__Boxed(System_Enum::parse(
                get_type_handle("UnityEngine.RenderTextureReadWrite")?,
                Il2CppString::new("Linear")?,
            )?);
            (*value).0
        };

        Ok((default_format, rw_format))
    }
}

/// Common texture rendering pipeline: texture → render target → readable texture → PNG bytes
unsafe fn render_texture_to_png_bytes(tex: UnityEngine_Texture2D) -> Result<Vec<u8>> {
    unsafe {
        let (default_format, rw_format) = get_render_texture_formats()?;

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

pub fn get_monster_png_bytes(monster_id: u32) -> Result<Vec<u8>> {
    unsafe {
        let monster_row = RPG_GameCore_MonsterTemplateExcelTable::GetData(monster_id)?;
        let type_handle = get_type_handle(UnityEngine_Sprite::ffi_name())?;

        let sprite = RPG_Client_CachedAssetLoader::SyncLoadAsset(
            monster_row.RoundIconPath()?,
            type_handle,
            false,
        )?;
        let sprite = UnityEngine_Sprite(sprite.0);
        let tex = sprite.get_texture()?;

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

        let type_handle = get_type_handle(UnityEngine_Sprite::ffi_name())?;

        let sprite = RPG_Client_CachedAssetLoader::SyncLoadAsset(
            avatar_row.AvatarSideIconPath()?,
            type_handle,
            false,
        )?;
        let sprite = UnityEngine_Sprite(sprite.0);
        let tex = sprite.get_texture()?;

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

        let type_handle = get_type_handle(UnityEngine_Sprite::ffi_name())?;
        
        let sprite = RPG_Client_CachedAssetLoader::SyncLoadAsset(
            icon_path,
            type_handle,
            false,
        )?;
        let sprite = UnityEngine_Sprite(sprite.0);
        let tex = sprite.get_texture()?;

        render_texture_to_png_bytes(tex)
    }
}

pub fn dump_avatar_png_bytes(avatar_id: u32, png_bytes: &[u8]) -> Result<()> {
    use std::fs;
    let out_dir = std::env::current_dir()?.join("avatar_png_dumps");
    fs::create_dir_all(&out_dir)?;
    let out_path = out_dir.join(format!("{}.png", avatar_id));
    fs::write(&out_path, png_bytes)?;

    log::info!("Saved avatar PNG dump: {}", out_path.display());
    Ok(())
}
