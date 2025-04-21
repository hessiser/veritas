use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub mod rpg {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
	pub mod client {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub static UIGameEntityUtils_GetAvatarID: LazyLock<fn(*const GameEntity) -> u32> = lazy_initialize_address!(0xac84910);
pub static GlobalVars_cctor: LazyLock<fn() -> *const NativeObject> = lazy_initialize_address!(0x9eb3ab0);
pub static AvatarData_get_AvatarName: LazyLock<fn(*const AvatarData) -> *const NativeString> = lazy_initialize_address!(0xb02d630);
pub static TextmapStatic_GetText: LazyLock<fn(*const TextID,*const NativeArray<NativeObject>) -> *const NativeString> = lazy_initialize_address!(0x8053330);
pub static AvatarModule_GetAvatar: LazyLock<fn(*const c_void,u32) -> *const AvatarData> = lazy_initialize_address!(0xb01ffe0);
	}
	pub mod gamecore {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub static BattleEventSkillRowData_get_SkillName: LazyLock<fn(*const BattleEventSkillRowData) -> *const TextID> = lazy_initialize_address!(0x8c663d0);
pub static BattleEventSkillRowData_get_AttackType: LazyLock<fn(*const BattleEventSkillRowData) -> AttackType> = lazy_initialize_address!(0x8c66320);
pub static AbilityStatic_GetActualOwner: LazyLock<fn(*const GameEntity) -> *const GameEntity> = lazy_initialize_address!(0x6140970);
pub static GamePlayStatic_GetEntityManager: LazyLock<fn() -> *const EntityManager> = lazy_initialize_address!(0x8dffa80);
pub static AvatarSkillRowData_get_SkillName: LazyLock<fn(*const AvatarSkillRowData) -> *const TextID> = lazy_initialize_address!(0x8c5e6e0);
pub static AvatarSkillRowData_get_AttackType: LazyLock<fn(*const AvatarSkillRowData) -> AttackType> = lazy_initialize_address!(0x8c5e3e0);
pub static ServantSkillRowData_get_SkillName: LazyLock<fn(*const ServantSkillRowData) -> *const TextID> = lazy_initialize_address!(0x951a110);
pub static ServantSkillRowData_get_AttackType: LazyLock<fn(*const ServantSkillRowData) -> AttackType> = lazy_initialize_address!(0x951a060);
pub static SkillCharacterComponent_GetSkillData: LazyLock<fn(*const SkillCharacterComponent,i32,i32) -> *const SkillData> = lazy_initialize_address!(0x5928910);
pub static TurnBasedAbilityComponent_GetAbilityMappedSkill: LazyLock<fn(*const TurnBasedAbilityComponent,*const NativeString) -> *const NativeString> = lazy_initialize_address!(0x593c340);
pub static CharacterConfig_GetSkillIndexByTriggerKey: LazyLock<fn(*const CharacterConfig,*const NativeString) -> i32> = lazy_initialize_address!(0xe321a70);
pub static EntityManager__GetEntitySummoner: LazyLock<fn(*const EntityManager,*const GameEntity) -> *const GameEntity> = lazy_initialize_address!(0x59a05c0);
	}
}
