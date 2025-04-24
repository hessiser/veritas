use crate::kreide::client::*;
use crate::kreide::gamecore::*;
use crate::kreide::native_types::*;
use std::{ffi::c_void, sync::LazyLock};
pub mod rpg {
    use crate::kreide::client::*;
    use crate::kreide::gamecore::*;
    use crate::kreide::native_types::*;
    use std::{ffi::c_void, sync::LazyLock};
    pub mod client {
        use crate::kreide::client::*;
        use crate::kreide::gamecore::*;
        use crate::kreide::native_types::*;
        use std::{ffi::c_void, sync::LazyLock};
        pub static AvatarModule_GetAvatar: LazyLock<fn(*const c_void, u32) -> *const AvatarData> =
            lazy_initialize_address!(0xad1eaf0);
        pub static AvatarData_get_AvatarName: LazyLock<
            fn(*const AvatarData) -> *const NativeString,
        > = lazy_initialize_address!(0xad2c080);
        pub static GlobalVars_cctor: LazyLock<fn() -> *const NativeObject> =
            lazy_initialize_address!(0x987e820);
        pub static UIGameEntityUtils_GetAvatarID: LazyLock<fn(*const GameEntity) -> u32> =
            lazy_initialize_address!(0xab0ce60);
        pub static TextmapStatic_GetText: LazyLock<
            fn(*const TextID, *const NativeArray<NativeObject>) -> *const NativeString,
        > = lazy_initialize_address!(0x7e8f730);
    }
    pub mod gamecore {
        use crate::kreide::client::*;
        use crate::kreide::gamecore::*;
        use crate::kreide::native_types::*;
        use std::{ffi::c_void, sync::LazyLock};
        pub static ServantSkillRowData_get_SkillName: LazyLock<
            fn(*const ServantSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x8bf5200);
        pub static ServantSkillRowData_get_AttackType: LazyLock<
            fn(*const ServantSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x8bf5100);
        pub static GamePlayStatic_GetEntityManager: LazyLock<fn() -> *const EntityManager> =
            lazy_initialize_address!(0x9048cb0);
        pub static AvatarSkillRowData_get_SkillName: LazyLock<
            fn(*const AvatarSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x8c007f0);
        pub static AvatarSkillRowData_get_AttackType: LazyLock<
            fn(*const AvatarSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x8c004c0);
        pub static BattleEventSkillRowData_get_SkillName: LazyLock<
            fn(*const BattleEventSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x8c085a0);
        pub static BattleEventSkillRowData_get_AttackType: LazyLock<
            fn(*const BattleEventSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x8c08510);
        pub static CharacterConfig_GetSkillIndexByTriggerKey: LazyLock<
            fn(*const CharacterConfig, *const NativeString) -> i32,
        > = lazy_initialize_address!(0xe31e870);
        pub static SkillCharacterComponent_GetSkillData: LazyLock<
            fn(*const SkillCharacterComponent, i32, i32) -> *const SkillData,
        > = lazy_initialize_address!(0x5aa9970);
        pub static TurnBasedAbilityComponent_GetAbilityMappedSkill: LazyLock<
            fn(*const TurnBasedAbilityComponent, *const NativeString) -> *const NativeString,
        > = lazy_initialize_address!(0x5abcb10);
        pub static EntityManager__GetEntitySummoner: LazyLock<
            fn(*const EntityManager, *const GameEntity) -> *const GameEntity,
        > = lazy_initialize_address!(0x593e200);
        pub static AbilityStatic_GetActualOwner: LazyLock<
            fn(*const GameEntity) -> *const GameEntity,
        > = lazy_initialize_address!(0x62cff40);
    }
}
pub mod unityengine {
    use crate::kreide::client::*;
    use crate::kreide::gamecore::*;
    use crate::kreide::native_types::*;
    use std::{ffi::c_void, sync::LazyLock};
    pub static Application_set_targetFrameRate: LazyLock<fn(i32)> =
        lazy_initialize_address!(0xe65b340);
}
