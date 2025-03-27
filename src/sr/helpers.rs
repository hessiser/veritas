use super::{functions::rpg::client::AvatarModule_GetAvatar, globals::GlobalVars, types::rpg::{client::{AvatarData, ModuleManager}, gamecore::FixPoint}};

#[inline]
pub fn get_avatar_data_by_id(avatar_id: u32) -> *const AvatarData {
    let s_module_manager = GlobalVars::get_global_var(GlobalVars::s_ModuleManager) as *const ModuleManager;
    let avatar_module = unsafe { (*s_module_manager).AvatarModule };
    AvatarModule_GetAvatar(avatar_module, avatar_id)
}

pub fn fixpoint_to_raw(fixpoint: &FixPoint) -> f32 {
    let hi = ((fixpoint.m_rawValue as u64 & 0xFFFFFFFF00000000) >> 32) as u32 as f32;
    let lo =  ((fixpoint.m_rawValue as u64 & 0x00000000FFFFFFFF)) as u32 as f32;
    hi + lo * (1f32/(2f32.powf(32f32)))
}