#![allow(non_camel_case_types, non_snake_case)]

use il2cpp_runtime::prelude::*;

use super::types::{
    RPG_GameCore_AttackType__Boxed, RPG_GameCore_FixPoint__Boxed, RPG_GameCore_GameEntity,
    RPG_GameCore_SkillCharacterComponent, RPG_GameCore_TurnBasedAbilityComponent,
};

#[il2cpp_value_type("EIFLGPGKPNB")]
pub struct EIFLGPGKPNB;

impl EIFLGPGKPNB__Boxed {
    #[il2cpp_field(name = "AANENKIIIMF")]
    pub fn AANENKIIIMF(&self) -> Il2CppString {}
}

#[il2cpp_ref_type("FGFFLOAEKKA")]
pub struct FGFFLOAEKKA;

impl FGFFLOAEKKA {
    #[il2cpp_field(name = "LFGAFLLHGCO")]
    pub fn LFGAFLLHGCO(&self) -> RPG_GameCore_GameEntity {}

    #[il2cpp_field(name = "GHCPGPKNBGF")]
    pub fn GHCPGPKNBGF(&self) -> RPG_GameCore_GameEntity {}
}

#[il2cpp_ref_type("OHFGNONJNIG")]
pub struct OHFGNONJNIG;

impl OHFGNONJNIG {
    #[il2cpp_field(name = "KJLBAGPFBDC")]
    pub fn KJLBAGPFBDC(&self) -> RPG_GameCore_FixPoint__Boxed {}

    #[il2cpp_field(name = "DOODKEMMAPK")]
    pub fn DOODKEMMAPK(&self) -> RPG_GameCore_AttackType__Boxed {}
}

#[il2cpp_ref_type("FHPFLNJLDHP")]
pub struct FHPFLNJLDHP;

impl FHPFLNJLDHP {
    #[il2cpp_field(name = "BIACLKKBFMM")]
    pub fn BIACLKKBFMM(&self) -> EIFLGPGKPNB__Boxed {}

    #[il2cpp_field(name = "HHOKFHMEFFF")]
    pub fn HHOKFHMEFFF(&self) -> RPG_GameCore_SkillCharacterComponent {}

    #[il2cpp_field(name = "MMALDILNGNJ")]
    pub fn MMALDILNGNJ(&self) -> RPG_GameCore_TurnBasedAbilityComponent {}
}

#[il2cpp_ref_type("EBKLINDPMKM")]
pub struct EBKLINDPMKM;
