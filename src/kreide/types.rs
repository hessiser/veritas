#![allow(
    non_camel_case_types,
    dead_code,
    non_snake_case,
    clippy::upper_case_acronyms
)]

use std::ffi::c_void;

use il2cpp_runtime::{il2cpp_enum_type, il2cpp_getter_property, il2cpp_value_type};
use il2cpp_runtime::prelude::*;


#[il2cpp_value_type("RPG.Client.TextID")]
pub struct RPG_Client_TextID {
    pub hash: i32,
    pub hash64: u64,
}

impl RPG_Client_TextID__Boxed {
    #[il2cpp_field(name = "hash")]
    pub fn hash(&self) -> System_Int32__Boxed {}
}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_TeamType {
    TeamUnknow,
    TeamLight,
    TeamDark,
    TeamNeutral,
    TeamNPC,
    Count
}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_AliveState {
    Unknown,
    Alive,
    Limbo,
    LimboRevivable,
    Deathrattle,
    Dying,
    Died,
    WillBeDestroyed,
    Destroyed,
}

#[il2cpp_ref_type("RPG.GameCore.BattleLineupData")]
pub struct RPG_GameCore_BattleLineupData;
impl RPG_GameCore_BattleLineupData {
    #[il2cpp_field(name = "LightTeam")]
    pub fn LightTeam(&self) -> Il2CppArray {}

    #[il2cpp_field(name = "ExtraTeam")]
    pub fn ExtraTeam(&self) -> Il2CppArray {}
}


#[il2cpp_ref_type("RPG.GameCore.TurnBasedGameMode")]
pub struct RPG_GameCore_TurnBasedGameMode;
impl RPG_GameCore_TurnBasedGameMode {

    #[il2cpp_field(name = "<OwnerBattleInstanceRef>k__BackingField")]
    pub fn _OwnerBattleInstanceRef_k__BackingField(&self) -> RPG_GameCore_BattleInstance {}

    #[il2cpp_field(name = "_CurrentTurnActionEntity")]
    pub fn _CurrentTurnActionEntity(&self) -> RPG_GameCore_GameEntity {}


    #[il2cpp_field(name = "_WaveMonsterCurrentCount")]
    pub fn _WaveMonsterCurrentCount(&self) -> System_Int32__Boxed {}

    #[il2cpp_field(name = "<ElapsedActionDelay>k__BackingField")]
    pub fn _ElapsedActionDelay_k__BackingField(&self) -> RPG_GameCore_FixPoint__Boxed {}

    #[il2cpp_field(name = "<WaveMonsterMaxCount>k__BackingField")]
    pub fn _WaveMonsterMaxCount_k__BackingField(&self) -> System_Int32__Boxed {}

    #[il2cpp_field(name = "<ChallengeTurnLimit>k__BackingField")]
    pub fn _ChallengeTurnLimit_k__BackingField(&self) -> System_UInt32__Boxed {}

    #[il2cpp_field(name = "<CurrentWaveStageID>k__BackingField")]
    pub fn _CurrentWaveStageID_k__BackingField(&self) -> System_UInt32__Boxed {}
}


#[il2cpp_ref_type("RPG.GameCore.TurnBasedAbilityComponent", base(RPG_GameCore_GameComponentBase))]
pub struct RPG_GameCore_TurnBasedAbilityComponent;

impl RPG_GameCore_TurnBasedAbilityComponent {
    #[il2cpp_method(name = "GetAbilityMappedSkill", args = ["string"])]
    pub fn get_ability_mapped_skill(&self, ability_name: Il2CppString) -> Il2CppString {}

    #[il2cpp_method(name = "GetProperty", args = ["RPG.GameCore.AbilityProperty"])]
    pub fn get_property(&self, property: RPG_GameCore_AbilityProperty) -> RPG_GameCore_FixPoint {}

    #[il2cpp_method(name = "TryCheckLimboWaitHeal", args = ["RPG.GameCore.GameEntity"])]
    pub fn try_check_limbo_wait_heal(&self, attacker: RPG_GameCore_GameEntity) -> bool {}

    // HJFKBBCMCCI[]
    #[il2cpp_field(name = "_AbilityProperties")]
    pub fn _AbilityProperties(&self) -> Il2CppArray {}


    #[il2cpp_field(name = "_KillerEntity")]
    pub fn _KillerEntity(&self) -> RPG_GameCore_GameEntity {}

    #[il2cpp_field(name = "_CharacterDataRef")]
    pub fn _CharacterDataRef(&self) -> RPG_GameCore_CharacterDataComponent {}   
}

#[il2cpp_ref_type("RPG.GameCore.CharacterConfig")]
pub struct RPG_GameCore_CharacterConfig;
impl RPG_GameCore_CharacterConfig {
    #[il2cpp_method(name = "GetSkillIndexByTriggerKey", args = ["string"])]
    pub fn get_skill_index_by_trigger_key(&self, skill_name: Il2CppString) -> i32 {}
}

#[il2cpp_enum_type(i32)]
pub enum UnityEngine_ProBuilder_EntityType {
    Detail,
    Occluder,
    Trigger,
    Collider,
    Mover,
}

#[il2cpp_ref_type("RPG.GameCore.TeamFormationComponent", base(RPG_GameCore_GameComponentBase))]
pub struct RPG_GameCore_TeamFormationComponent;
impl RPG_GameCore_TeamFormationComponent {
    #[il2cpp_field(name = "_TeamFormationDatas")]
    pub fn _TeamFormationDatas(&self) -> List {}

    #[il2cpp_field(name = "_Team")]
    pub fn _Team(&self) -> RPG_GameCore_TeamType__Boxed {}
}


#[il2cpp_ref_type("RPG.GameCore.MonsterRowData")]
pub struct RPG_GameCore_MonsterRowData;
impl RPG_GameCore_MonsterRowData {
    #[il2cpp_method(name = "get_Level", args = [])]
    pub fn get_Level(&self) -> u32 {}

    #[il2cpp_field(name = "_Row")]
    pub fn _Row(&self) -> RPG_GameCore_MonsterRow {}
}

#[il2cpp_ref_type("RPG.Client.AvatarData")]
pub struct RPG_Client_AvatarData;
impl RPG_Client_AvatarData {

    #[il2cpp_getter_property(property = "AvatarName")]
    pub fn AvatarName(&self) -> Il2CppString {}
}


#[il2cpp_ref_type("RPG.GameCore.MonsterRow")]
pub struct RPG_GameCore_MonsterRow;
impl RPG_GameCore_MonsterRow {
    #[il2cpp_field(name = "MonsterName")]
    pub fn MonsterName(&self) -> RPG_Client_TextID__Boxed {}
}


#[il2cpp_ref_type("RPG.GameCore.MonsterDataComponent", base(RPG_GameCore_GameComponentBase))]
pub struct RPG_GameCore_MonsterDataComponent;
impl RPG_GameCore_MonsterDataComponent {
    #[il2cpp_method(name = "GetMonsterID", args = [])]
    pub fn get_monster_id(&self) -> u32 {}

    #[il2cpp_method(name = "GetMonsterTemplateID", args = [])]
    pub fn get_monster_template_id(&self) -> u32 {}

    #[il2cpp_field(name = "_OwnerRef")]
    pub fn _OwnerRef(&self) -> RPG_GameCore_GameEntity {}

    #[il2cpp_field(name = "_MonsterRowData")]
    pub fn _MonsterRowData(&self) -> RPG_GameCore_MonsterRowData {}

    #[il2cpp_field(name = "_DefaultMaxHP")]
    pub fn _DefaultMaxHP(&self) -> RPG_GameCore_FixPoint__Boxed {}
}

#[il2cpp_ref_type("RPG.GameCore.GameComponentBase")]
pub struct RPG_GameCore_GameComponentBase;
impl RPG_GameCore_GameComponentBase {

    #[il2cpp_field(name = "_OwnerRef")]
    pub fn _OwnerRef(&self) -> RPG_GameCore_GameEntity {}
}

#[il2cpp_ref_type("RPG.GameCore.AvatarPropertyRow")]
pub struct RPG_GameCore_AvatarPropertyRow;
impl RPG_GameCore_AvatarPropertyRow {
    #[il2cpp_field(name = "IconPath")]
    pub fn IconPath(&self) -> Il2CppString {}
}

#[il2cpp_ref_type("RPG.GameCore.AvatarPropertyExcelTable")]
pub struct RPG_GameCore_AvatarPropertyExcelTable;
impl RPG_GameCore_AvatarPropertyExcelTable {
    #[il2cpp_method(name = "GetData", args = ["RPG.GameCore.AvatarPropertyType"])]
    pub fn GetData(property_type: RPG_GameCore_AvatarPropertyType) -> RPG_GameCore_AvatarPropertyRow {}
}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_AvatarPropertyType {
	Unknown,
	MaxHP,
	Attack,
	Defence,
	Speed,
	CriticalChance,
	CriticalDamage,
	HealRatio,
	StanceBreakAddedRatio,
	SPRatio,
	StatusProbability,
	StatusResistance,
	PhysicalAddedRatio,
	PhysicalResistance,
	FireAddedRatio,
	FireResistance,
	IceAddedRatio,
	IceResistance,
	ThunderAddedRatio,
	ThunderResistance,
	WindAddedRatio,
	WindResistance,
	QuantumAddedRatio,
	QuantumResistance,
	ImaginaryAddedRatio,
	ImaginaryResistance,
	BaseHP,
	HPDelta,
	BaseAttack,
	AttackDelta,
	BaseDefence,
	DefenceDelta,
	HPAddedRatio,
	AttackAddedRatio,
	DefenceAddedRatio,
	BaseSpeed,
	HealTakenRatio,
	PhysicalResistanceDelta,
	FireResistanceDelta,
	IceResistanceDelta,
	ThunderResistanceDelta,
	WindResistanceDelta,
	QuantumResistanceDelta,
	ImaginaryResistanceDelta,
	AllDamageReduce,
	RelicValueExtraAdditionRatio,
	EquipValueExtraAdditionRatio,
	EquipExtraRank,
	AvatarExtraRank,
	AllDamageTypeAddedRatio,
	SpeedAddedRatio,
	SpeedDelta,
	CriticalChanceBase,
	CriticalDamageBase,
	SPRatioBase,
	HealRatioBase,
	StatusProbabilityBase,
	StatusResistanceBase,
	BreakDamageAddedRatio,
	BreakDamageAddedRatioBase,
	MaxSP,
	SpecialMaxSP,
	PhysicalPenetrate,
	FirePenetrate,
	IcePenetrate,
	ThunderPenetrate,
	WindPenetrate,
	QuantumPenetrate,
	ImaginaryPenetrate,
	AllDamageTypePenetrate,
	BreakDamageExtraAddedRatio,
	ElationDamageAddedRatio,
	ElationDamageAddedRatioBase,
	ExtraAttackAddedRatio1,
	ExtraAttackAddedRatio2,
	ExtraAttackAddedRatio3,
	ExtraAttackAddedRatio4,
	ExtraDefenceAddedRatio1,
	ExtraDefenceAddedRatio2,
	ExtraDefenceAddedRatio3,
	ExtraDefenceAddedRatio4,
	ExtraHPAddedRatio1,
	ExtraHPAddedRatio2,
	ExtraHPAddedRatio3,
	ExtraHPAddedRatio4,
	ExtraHealAddedRatio,
	ExtraAllDamageTypeAddedRatio1,
	ExtraAllDamageTypeAddedRatio2,
	ExtraAllDamageTypeAddedRatio3,
	ExtraAllDamageTypeAddedRatio4,
	ExtraAllDamageReduce,
	ExtraShieldAddedRatio,
	ExtraSpeedAddedRatio1,
	ExtraSpeedAddedRatio2,
	ExtraSpeedAddedRatio3,
	ExtraSpeedAddedRatio4,
	ExtraLuckChance,
	ExtraLuckDamage,
	ExtraFrontPowerBase,
	ExtraFrontPowerAddedRatio1,
	ExtraFrontPowerAddedRatio2,
	ExtraBackPowerBase,
	ExtraBackPowerAddedRatio1,
	ExtraBackPowerAddedRatio2,
	ExtraUltraDamageAddedRatio1,
	ExtraSkillDamageAddedRatio1,
	ExtraNormalDamageAddedRatio1,
	ExtraInsertDamageAddedRatio1,
	ExtraTotalFrontPower,
	ExtraTotalBackPower,
	ExtraDOTDamageAddedRatio1,
	ExtraHealBase,
	ExtraShieldBase,
	ExtraTotalShieldPower,
	ExtraTotalHealPower,
	ExtraTotalSpeedAddedRatio,
	ExtraEnergyBar,
	ExtraInitSP,
	ExtraElementDamageAddedRatio1,
	ExtraTotalLuckChance,
	ExtraLuckChanceBase,
	ExtraTotalLuckDamage,
	ExtraLuckDamageBase,
	ExtraDamageAddedRatio1,
	ExtraQuantumResonance,
	ExtraFrontPowerConvert,
	ExtraBackPowerConvert,
	ExtraLuckDamageConvert,
	ExtraLuckChanceConvert,
	ExtraHealConvert,
	ExtraShieldConvert,
	ExtraAllDamageReduceConvert,
	ExtraTotalAllDamageReduce,
	ExtraAllDamageTypeAddedRatio5
}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_AbilityProperty {
	Unknow,
	MaxHP,
	BaseHP,
	HPAddedRatio,
	HPDelta,
	HPConvert,
	DirtyHPDelta,
	DirtyHPRatio,
	RallyHP,
	NegativeHP,
	CurrentHP,
	MaxSP,
	CurrentSP,
	MaxSpecialSP,
	CurrentSpecialSP,
	MaxExtraSpecialSP,
	CurExtraSpecialSP,
	AdditionalBP,
	Attack,
	BaseAttack,
	AttackAddedRatio,
	AttackDelta,
	AttackConvert,
	Defence,
	BaseDefence,
	DefenceAddedRatio,
	DefenceDelta,
	DefenceConvert,
	DefenceOverride,
	Level,
	Promotion,
	Rank,
	Speed,
	BaseSpeed,
	SpeedAddedRatio,
	SpeedDelta,
	SpeedConvert,
	SpeedOverride,
	ActionDelay,
	ActionDelayAddedRatio,
	ActionDelayAddAttenuation,
	MaxStance,
	CurrentStance,
	Level_FinalDamageAddedRatio,
	AllDamageTypeAddedRatio,
	AllDamageReduce,
	BaseDamageMultiRatio,
	DotDamageAddedRatio,
	FatigueRatio,
	CriticalChance,
	CriticalChanceBase,
	CriticalChanceConvert,
	CriticalDamage,
	CriticalDamageBase,
	CriticalDamageConvert,
	CriticalResistance,
	PhysicalAddedRatio,
	FireAddedRatio,
	IceAddedRatio,
	ThunderAddedRatio,
	QuantumAddedRatio,
	ImaginaryAddedRatio,
	WindAddedRatio,
	PhysicalResistance,
	FireResistance,
	IceResistance,
	ThunderResistance,
	QuantumResistance,
	ImaginaryResistance,
	WindResistance,
	PhysicalResistanceBase,
	FireResistanceBase,
	IceResistanceBase,
	ThunderResistanceBase,
	QuantumResistanceBase,
	ImaginaryResistanceBase,
	WindResistanceBase,
	AllDamageTypeResistanceOverrideBase,
	PhysicalResistanceDelta,
	FireResistanceDelta,
	IceResistanceDelta,
	ThunderResistanceDelta,
	QuantumResistanceDelta,
	ImaginaryResistanceDelta,
	WindResistanceDelta,
	AllDamageTypeResistance,
	PhysicalPenetrate,
	FirePenetrate,
	IcePenetrate,
	ThunderPenetrate,
	QuantumPenetrate,
	ImaginaryPenetrate,
	WindPenetrate,
	AllDamageTypePenetrate,
	PhysicalTakenRatio,
	FireTakenRatio,
	IceTakenRatio,
	ThunderTakenRatio,
	QuantumTakenRatio,
	ImaginaryTakenRatio,
	WindTakenRatio,
	AllDamageTypeTakenRatio,
	Monster_DamageTakenRatio,
	PhysicalAbsorb,
	FireAbsorb,
	IceAbsorb,
	ThunderAbsorb,
	QuantumAbsorb,
	ImaginaryAbsorb,
	WindAbsorb,
	MinimumFatigueRatio,
	ForceStanceBreakRatio,
	StanceBreakAddedRatio,
	StanceBreakResistance,
	StanceBreakTakenRatio,
	PhysicalStanceBreakTakenRatio,
	FireStanceBreakTakenRatio,
	IceStanceBreakTakenRatio,
	ThunderStanceBreakTakenRatio,
	WindStanceBreakTakenRatio,
	QuantumStanceBreakTakenRatio,
	ImaginaryStanceBreakTakenRatio,
	StanceWeakAddedRatio,
	StanceDefaultAddedRatio,
	HealRatio,
	HealRatioBase,
	HealRatioConvert,
	HealTakenRatio,
	Shield,
	MaxShield,
	ShieldAddedRatio,
	ShieldTakenRatio,
	StatusProbability,
	StatusProbabilityBase,
	StatusProbabilityConvert,
	StatusResistance,
	StatusResistanceBase,
	StatusResistanceConvert,
	SPRatio,
	SPRatioBase,
	SPRatioConvert,
	SPRatioOverride,
	BreakDamageAddedRatio,
	BreakDamageAddedRatioBase,
	BreakDamageAddedRatioConvert,
	BreakDamageExtraAddedRatio,
	ElationDamageAddedRatio,
	ElationDamageAddedRatioBase,
	ElationDamageAddedRatioConvert,
	ElationDamageExtraAddedRatio,
	ElationEchoPoint,
	PhysicalStanceBreakResistance,
	FireStanceBreakResistance,
	IceStanceBreakResistance,
	ThunderStanceBreakResistance,
	WindStanceBreakResistance,
	QuantumStanceBreakResistance,
	ImaginaryStanceBreakResistance,
	AggroBase,
	AggroAddedRatio,
	AggroDelta,
	RelicValueExtraAdditionRatio,
	EquipValueExtraAdditionRatio,
	EquipExtraRank,
	AvatarExtraRank,
	Combo,
	NormalBattleCount,
	ExtraAttackAddedRatio1,
	ExtraAttackAddedRatio2,
	ExtraAttackAddedRatio3,
	ExtraAttackAddedRatio4,
	ExtraDefenceAddedRatio1,
	ExtraDefenceAddedRatio2,
	ExtraDefenceAddedRatio3,
	ExtraDefenceAddedRatio4,
	ExtraHPAddedRatio1,
	ExtraHPAddedRatio2,
	ExtraHPAddedRatio3,
	ExtraHPAddedRatio4,
	ExtraHealAddedRatio,
	ExtraAllDamageTypeAddedRatio1,
	ExtraAllDamageTypeAddedRatio2,
	ExtraAllDamageTypeAddedRatio3,
	ExtraAllDamageTypeAddedRatio4,
	ExtraAllDamageTypeAddedRatio5,
	ExtraAllDamageReduce,
	ExtraShieldAddedRatio,
	ExtraSpeedAddedRatio1,
	ExtraSpeedAddedRatio2,
	ExtraSpeedAddedRatio3,
	ExtraSpeedAddedRatio4,
	ExtraLuckChance,
	ExtraLuckDamage,
	ExtraTotalFrontPower,
	ExtraFrontPowerBase,
	ExtraFrontPowerAddedRatio1,
	ExtraFrontPowerAddedRatio2,
	ExtraTotalBackPower,
	ExtraBackPowerBase,
	ExtraBackPowerAddedRatio1,
	ExtraBackPowerAddedRatio2,
	ExtraUltraDamageAddedRatio1,
	ExtraSkillDamageAddedRatio1,
	ExtraNormalDamageAddedRatio1,
	ExtraInsertDamageAddedRatio1,
	ExtraDOTDamageAddedRatio1,
	ExtraElementDamageAddedRatio1,
	ExtraHealBase,
	ExtraShieldBase,
	ExtraTotalShieldPower,
	ExtraTotalHealPower,
	ExtraTotalSpeedAddedRatio,
	ExtraInitSP,
	ExtraTotalLuckDamage,
	ExtraTotalLuckChance,
	ExtraBackPowerConvert,
	ExtraFrontPowerConvert,
	ExtraLuckDamageConvert,
	ExtraLuckChanceConvert,
	ExtraHealConvert,
	ExtraShieldConvert,
	ExtraAllDamageReduceConvert,
	ExtraQuantumResonance,
	ExtraTotalAllDamageReduce,
	Count,
}

#[il2cpp_ref_type("RPG.GameCore.SkillCharacterComponent", base(RPG_GameCore_GameComponentBase))]
pub struct RPG_GameCore_SkillCharacterComponent;
impl RPG_GameCore_SkillCharacterComponent {
    #[il2cpp_method(name = "GetSkillData", args = ["int", "int"])]
    pub fn get_skill_data(&self, skill_index: i32, extra_use_param: i32) -> RPG_GameCore_SkillData {}

    #[il2cpp_field(name = "_CharacterDataRef")]
    pub fn _CharacterDataRef(&self) -> RPG_GameCore_CharacterDataComponent {}
}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_EntityType {
	None,
	Avatar,
	Monster,
	LocalPlayer,
	NPC,
	NPCMonster,
	StoryCharacter,
	Prop,
	Mission,
	LevelEntity,
	Neutral,
	AtmoNpc,
	BattleEvent,
	TutorialEntity,
	Team,
	Partner,
	LevelGraph,
	Snapshot,
	TeamFormation,
	Model,
	UICamera,
	District,
	GlobalShield,
	CustomData,
	Simple,
	PuzzleGameObjectProp,
	PerformanceLevelGraph,
	Group,
	RtCharacter,
	RtGrid,
	RtTrigger,
	SummonUnit,
	LittleGameInstance,
	Servant,
	PreviewShow,
	LittleGameContainer,
	LittleGameViewProxy,
	GridFightBackend,
	DummyEntity
}

#[il2cpp_ref_type("RPG.GameCore.BattleEventDataComponent", base(RPG_GameCore_GameComponentBase))]
pub struct RPG_GameCore_BattleEventDataComponent;
impl RPG_GameCore_BattleEventDataComponent {
    #[il2cpp_field(name = "<SourceCaster>k__BackingField")]
    pub fn _SourceCaster_k__BackingField(&self) -> RPG_GameCore_GameEntity {}
}

#[il2cpp_ref_type("RPG.GameCore.SkillData")]
pub struct RPG_GameCore_SkillData;
impl RPG_GameCore_SkillData {

    #[il2cpp_field(name = "RowData")]
    pub fn RowData(&self) -> RPG_GameCore_ICharacterSkillRowData {}

    #[il2cpp_field(name = "SkillConfigID")]
    pub fn SkillConfigID(&self) -> System_UInt32__Boxed {}
}


#[il2cpp_ref_type("RPG.GameCore.LineUpCharacter")]
pub struct RPG_GameCore_LineUpCharacter;
impl RPG_GameCore_LineUpCharacter {
    #[il2cpp_field(name = "CharacterID")]
    pub fn CharacterID(&self) -> System_UInt32__Boxed {}
}

#[il2cpp_ref_type("RPG.GameCore.CharacterDataComponent", base(RPG_GameCore_GameComponentBase))]
pub struct RPG_GameCore_CharacterDataComponent;
impl RPG_GameCore_CharacterDataComponent {
    #[il2cpp_field(name = "<JsonConfig>k__BackingField")]
    pub fn _JsonConfig_k__BackingField(&self) -> RPG_GameCore_CharacterConfig {}

    #[il2cpp_field(name = "Summoner")]
    pub fn Summoner(&self) -> RPG_GameCore_GameEntity {}
}

#[il2cpp_value_type("RPG.GameCore.FixPoint")]
pub struct RPG_GameCore_FixPoint {
    pub m_rawValue: i64,
}

impl RPG_GameCore_FixPoint__Boxed {
	#[il2cpp_method(name = "ToFloat", args = [])]
	pub fn ToFloat(self) -> System_Single__Boxed {}
}


#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_AttackType {
    Unknown,
    Normal,
    BPSkill,
    Ultra,
    QTE,
    DOT,
    Pursued,
    Maze,
    MazeNormal,
    Insert,
    ElementDamage,
    Level,
    Servant,
    TrueDamage,
	ElationDamage
}

impl Ord for RPG_GameCore_AttackType {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		(*self as i32).cmp(&(*other as i32))
	}
}

impl PartialOrd for RPG_GameCore_AttackType {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

#[il2cpp_ref_type("RPG.GameCore.BattleInstance")]
pub struct RPG_GameCore_BattleInstance;
impl RPG_GameCore_BattleInstance {
    #[il2cpp_field(name = "_GameWorld")]
    pub fn _GameWorld(&self) -> RPG_GameCore_GameWorld {}

    #[il2cpp_field(name = "_BattleLineupData")]
    pub fn _BattleLineupData(&self) -> RPG_GameCore_BattleLineupData {}
}

#[il2cpp_ref_type("RPG.GameCore.GameEntity")]
pub struct RPG_GameCore_GameEntity;
impl RPG_GameCore_GameEntity {
    #[il2cpp_method(name = "GetComponent", args = ["System.Type"])]
    pub fn get_component(&self, ty: System_RuntimeType) -> RPG_GameCore_GameComponentBase {}

	#[il2cpp_method(name = "GetComponent", args = ["System.RuntimeTypeHandle"])]
	pub fn get_component_by_type_handle(&self, ty: Il2CppType) -> RPG_GameCore_GameComponentBase {}

    #[il2cpp_field(name = "_ComponentList")]
    pub fn _ComponentList(&self) -> Il2CppArray {}

    #[il2cpp_field(name = "_AliveState")]
    pub fn _AliveState(&self) -> RPG_GameCore_AliveState__Boxed {}

    #[il2cpp_field(name = "_Team")]
    pub fn _Team(&self) -> RPG_GameCore_TeamType__Boxed {}

    #[il2cpp_field(name = "<RuntimeID>k__BackingField")]
    pub fn _RuntimeID_k__BackingField(&self) -> System_UInt32__Boxed {}

    #[il2cpp_field(name = "_EntityType")]
    pub fn _EntityType(&self) -> RPG_GameCore_EntityType__Boxed {}

    #[il2cpp_field(name = "_OwnerWorldRef")]
    pub fn _OwnerWorldRef(&self) -> RPG_GameCore_GameWorld {}
}

#[il2cpp_ref_type("RPG.Client.ModuleManager")]
pub struct RPG_Client_ModuleManager;
impl RPG_Client_ModuleManager {

    #[il2cpp_field(name = "AvatarModule")]
    pub fn AvatarModule(&self) -> RPG_Client_AvatarModule {}
}


#[il2cpp_ref_type("RPG.GameCore.ICharacterSkillRowData")]
pub struct RPG_GameCore_ICharacterSkillRowData;
impl RPG_GameCore_ICharacterSkillRowData {
    #[il2cpp_getter_property(property = "SkillName")]
    pub fn get_SkillName(&self) -> RPG_Client_TextID {}

    #[il2cpp_getter_property(property = "AttackType")]
    pub fn get_AttackType(&self) -> RPG_GameCore_AttackType {}
}

#[il2cpp_ref_type("RPG.Client.AvatarModule")]
pub struct RPG_Client_AvatarModule;
impl RPG_Client_AvatarModule {
    #[il2cpp_method(name = "GetAvatar", args = ["uint"])]
    pub fn get_avatar(&self, avatar_id: u32) -> RPG_Client_AvatarData {}
}

#[il2cpp_ref_type("RPG.GameCore.GameWorld")]
pub struct RPG_GameCore_GameWorld;
impl RPG_GameCore_GameWorld {
    #[il2cpp_field(name = "_EntityManager")]
    pub fn _EntityManager(&self) -> RPG_GameCore_EntityManager {}

    #[il2cpp_field(name = "<BattleInstanceRef>k__BackingField")]
    pub fn _BattleInstanceRef_k__BackingField(&self) -> RPG_GameCore_BattleInstance {}
}

#[il2cpp_ref_type("RPG.GameCore.EntityManager")]
pub struct RPG_GameCore_EntityManager;
impl RPG_GameCore_EntityManager {
    #[il2cpp_method(name = "GetEntityByRuntimeID", args = ["uint"])]
    pub fn get_entity_by_runtime_id(&self, runtime_id: u32) -> RPG_GameCore_GameEntity {}

    #[il2cpp_method(name = "_GetEntitySummoner", args = ["RPG.GameCore.GameEntity"])]
    pub fn get_entity_summoner(&self, entity: RPG_GameCore_GameEntity) -> RPG_GameCore_GameEntity {}
}


#[il2cpp_ref_type("RPG.Client.GlobalVars")]
pub struct RPG_Client_GlobalVars;
impl RPG_Client_GlobalVars {
    #[il2cpp_field(name = "s_ModuleManager")]
    pub fn s_ModuleManager() -> RPG_Client_ModuleManager {}
}

#[il2cpp_ref_type("RPG.Client.TextmapStatic")]
pub struct RPG_Client_TextmapStatic;
impl RPG_Client_TextmapStatic {
    #[il2cpp_method(name = "GetText", args = ["RPG.Client.TextID", "object[]"])]
    pub fn get_text(id: &RPG_Client_TextID, replace_params: *const c_void) -> Il2CppString {}
}

#[il2cpp_ref_type("RPG.Client.UIGameEntityUtils")]
pub struct RPG_Client_UIGameEntityUtils;
impl RPG_Client_UIGameEntityUtils {
    #[il2cpp_method(name = "GetAvatarID", args = ["RPG.GameCore.GameEntity"])]
    pub fn get_avatar_id(entity: RPG_GameCore_GameEntity) -> u32 {}
}

#[il2cpp_ref_type("RPG.GameCore.AbilityStatic")]
pub struct RPG_GameCore_AbilityStatic;
impl RPG_GameCore_AbilityStatic {
    #[il2cpp_method(name = "GetActualOwner", args = ["RPG.GameCore.GameEntity"])]
    pub fn get_actual_owner(entity: RPG_GameCore_GameEntity) -> RPG_GameCore_GameEntity {}
}


#[il2cpp_ref_type("RPG.Client.BattleAssetPreload")]
pub struct RPG_Client_BattleAssetPreload;
impl RPG_Client_BattleAssetPreload {
    #[il2cpp_field(name = "_LineupData")]
    pub fn _LineupData(&self) -> RPG_GameCore_BattleLineupData {}
}

#[il2cpp_ref_type("RPG.GameCore.ServantDataComponent", base(RPG_GameCore_GameComponentBase))]
pub struct RPG_GameCore_ServantDataComponent;
impl RPG_GameCore_ServantDataComponent {
    #[il2cpp_field(name = "_ServantRowData")]
    pub fn _ServantRowData(&self) -> RPG_GameCore_ServantRowData {}
}

#[il2cpp_ref_type("RPG.GameCore.ServantRowData")]
pub struct RPG_GameCore_ServantRowData;
impl RPG_GameCore_ServantRowData {
    #[il2cpp_field(name = "_Row")]
    pub fn _Row(&self) -> RPG_GameCore_AvatarServantRow {}
}

#[il2cpp_ref_type("RPG.GameCore.AvatarServantRow")]
pub struct RPG_GameCore_AvatarServantRow;
impl RPG_GameCore_AvatarServantRow {
    #[il2cpp_field(name = "ServantID")]
    pub fn ServantID(&self) -> System_UInt32__Boxed {}

    #[il2cpp_field(name = "ServantName")]
    pub fn ServantName(&self) -> RPG_Client_TextID__Boxed {}
}

#[il2cpp_ref_type("RPG.GameCore.AbilityComponent", base(RPG_GameCore_GameComponentBase))]
pub struct RPG_GameCore_AbilityComponent;
impl RPG_GameCore_AbilityComponent {
    #[il2cpp_field(name = "_ModifierList")]
    pub fn _ModifierList(&self) -> Il2CppArray {}
}

// #[il2cpp_ref_type("RPG.GameCore.StatusExcelTable")]
// pub struct RPG_GameCore_StatusExcelTable;
// impl RPG_GameCore_StatusExcelTable {
//     #[il2cpp_method(name = "GetByModifierName", args = ["string"])]
//     pub fn get_by_modifier_name(&self, modifier_name: Il2CppString) -> RPG_GameCore_StatusRow {}
// }


#[il2cpp_ref_type("RPG.GameCore.StatusRow")]
pub struct RPG_GameCore_StatusRow;
impl RPG_GameCore_StatusRow {
    #[il2cpp_field(name = "StatusName")]
    pub fn StatusName(&self) -> RPG_Client_TextID__Boxed {}

    #[il2cpp_field(name = "StatusDesc")]
    pub fn StatusDesc(&self) -> RPG_Client_TextID__Boxed {}
}

#[il2cpp_ref_type("RPG.GameCore.TurnBasedModifierInstance")]
pub struct RPG_GameCore_TurnBasedModifierInstance;
impl RPG_GameCore_TurnBasedModifierInstance {
    #[il2cpp_method(name = "get_KeyForStatusConfig", args = [])]
    pub fn get_key_for_status_config(&self) -> Il2CppString {}
}

#[il2cpp_ref_type("RPG.Client.CachedAssetLoader")]
pub struct RPG_Client_CachedAssetLoader;
impl RPG_Client_CachedAssetLoader {
    #[il2cpp_method(name = "SyncLoadAsset", args = ["string", "System.Type", "bool"])]
    pub fn SyncLoadAsset(asset_name: Il2CppString, ty: System_Type, flag: bool) -> UnityEngine_Object {}
}

#[il2cpp_ref_type("UnityEngine.Sprite")]
pub struct UnityEngine_Sprite;
impl UnityEngine_Sprite {
	#[il2cpp_getter_property(property = "texture")]
	pub fn get_texture(&self) -> UnityEngine_Texture2D {}
}

#[il2cpp_ref_type("UnityEngine.Object")]
pub struct UnityEngine_Object;

impl UnityEngine_Object {
	#[il2cpp_getter_property(property = "name")]
	pub fn get_name(&self) -> Il2CppString {}
}

#[il2cpp_value_type("UnityEngine.Rect")]
pub struct UnityEngine_Rect {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}
#[il2cpp_ref_type("UnityEngine.Texture2D", base(UnityEngine_Texture))]
pub struct UnityEngine_Texture2D;
impl UnityEngine_Texture2D {
	// #[il2cpp_getter_property(property = "format")]
	// pub fn get_format(&self) -> i32 {}

	// #[il2cpp_getter_property(property = "isReadable")]
	// pub fn get_is_readable(&self) -> bool {}

	#[il2cpp_method(name = ".ctor", args = ["int", "int"])]
	pub fn new(self, width: i32, height: i32) {}

	#[il2cpp_method(name = "GetPixels32", args = [])]
	pub fn get_pixels32(&self) -> Il2CppArray {}

	#[il2cpp_method(name = "ReadPixels", args = ["UnityEngine.Rect", "int", "int"])]
	pub fn read_pixels(&self, source: UnityEngine_Rect, destX: i32, destY: i32) {}

	#[il2cpp_method(name = "Apply", args = [])]
	pub fn apply(&self) {}
}

#[il2cpp_ref_type("UnityEngine.ImageConversion")]
pub struct UnityEngine_ImageConversion;
impl UnityEngine_ImageConversion {
	#[il2cpp_method(name = "EncodeToPNG", args = ["UnityEngine.Texture2D"])]
	pub fn EncodeToPNG(tex: UnityEngine_Texture2D) -> Il2CppArray {}
}

#[il2cpp_ref_type("UnityEngine.Texture")]
pub struct UnityEngine_Texture;
impl UnityEngine_Texture {
	#[il2cpp_getter_property(property = "width")]
	pub fn get_width(&self) -> i32 {}

	#[il2cpp_getter_property(property = "height")]
	pub fn get_height(&self) -> i32 {}
}

#[il2cpp_value_type("UnityEngine.Color32")]
pub struct UnityEngine_Color32 {
	pub rgba: i32,
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

#[il2cpp_ref_type("RPG.GameCore.AvatarExcelTable")]
pub struct RPG_GameCore_AvatarExcelTable;
impl RPG_GameCore_AvatarExcelTable {
	#[il2cpp_method(name = "GetData", args = ["uint"])]
	pub fn GetData(avatar_id: u32) -> RPG_GameCore_AvatarRow {}
}

#[il2cpp_ref_type("RPG.GameCore.AvatarRow")]
pub struct RPG_GameCore_AvatarRow;
impl RPG_GameCore_AvatarRow {
	#[il2cpp_field(name = "AvatarSideIconPath")]
	pub fn AvatarSideIconPath(&self) -> Il2CppString {}

	#[il2cpp_field(name = "AvatarName")]
	pub fn AvatarName(&self) -> RPG_Client_TextID__Boxed {}
}

#[il2cpp_ref_type("RPG.GameCore.MonsterTemplateExcelTable")]
pub struct RPG_GameCore_MonsterTemplateExcelTable;
impl RPG_GameCore_MonsterTemplateExcelTable {
	#[il2cpp_method(name = "GetData", args = ["uint"])]
	pub fn GetData(template_id: u32) -> RPG_GameCore_MonsterTemplateRow {}
}

#[il2cpp_ref_type("RPG.GameCore.MonsterTemplateRow")]
pub struct RPG_GameCore_MonsterTemplateRow;
impl RPG_GameCore_MonsterTemplateRow {
	#[il2cpp_field(name = "RoundIconPath")]
	pub fn RoundIconPath(&self) -> Il2CppString {}
}

#[il2cpp_ref_type("UnityEngine.RenderTexture")]
pub struct UnityEngine_RenderTexture;
impl UnityEngine_RenderTexture {
	#[il2cpp_method(name = "GetTemporary", args = ["int", "int", "int", "UnityEngine.RenderTextureFormat", "UnityEngine.RenderTextureReadWrite"])]
	pub fn GetTemporary(width: i32, height: i32, depthBuffer: i32, format: i32, readWrite: i32) -> UnityEngine_RenderTexture {}

	#[il2cpp_method(name = "GetActive", args = [])]
	pub fn GetActive() -> UnityEngine_RenderTexture {}

	#[il2cpp_method(name = "set_active", args = ["UnityEngine.RenderTexture"])]
	pub fn set_active(rt: UnityEngine_RenderTexture) {}

	// ReleaseTemporary(UnityEngine.RenderTexture)
	#[il2cpp_method(name = "ReleaseTemporary", args = ["UnityEngine.RenderTexture"])]
	pub fn ReleaseTemporary(rt: UnityEngine_RenderTexture) {}

	#[il2cpp_getter_property(property = "width")]
	pub fn get_width(&self) -> i32 {}

	#[il2cpp_getter_property(property = "height")]
	pub fn get_height(&self) -> i32 {}
}

#[il2cpp_ref_type("UnityEngine.Graphics")]
pub struct UnityEngine_Graphics;
impl UnityEngine_Graphics {
	#[il2cpp_method(name = "Blit", args = ["UnityEngine.Texture", "UnityEngine.RenderTexture"])]
	pub fn Blit(source: UnityEngine_Texture2D, dest: UnityEngine_RenderTexture) {}
}