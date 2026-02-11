#![allow(
    non_camel_case_types,
    dead_code,
    non_snake_case,
    clippy::upper_case_acronyms
)]

use crate::{
    cs_class, cs_field, cs_method, cs_property,
    kreide::il2cpp::native::{Il2CppArray, Il2CppObject, Il2CppString, List, RuntimeType},
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPG_Client_TextID {
    pub hash: i32,
    pub hash64: u64,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RPG_GameCore_TeamType {
    TeamUnknow = 0,
    TeamLight = 1,
    TeamDark = 2,
    TeamNeutral = 3,
    TeamNPC = 4,
    Count = 5,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RPG_GameCore_AliveState {
    Unknown = 0,
    Alive = 1,
    Limbo = 2,
    LimboRevivable = 3,
    Deathrattle = 4,
    Dying = 5,
    Died = 6,
    WillBeDestroyed = 7,
    Destroyed = 8,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// AFBBBCMBEME
pub struct AFBBBCMBEME(pub usize);
impl AFBBBCMBEME {
    cs_class!("AFBBBCMBEME");

    // AFBBBCMBEME -> Type: string | Name: NPKHCIODJAF | Offset: 0x8
    cs_field!(NPKHCIODJAF, "NPKHCIODJAF", self, |v| -> Il2CppString {
        Il2CppString(v.0)
    });
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// AFBBBCMBEME
pub struct HIKJAFBBPIP(pub usize);
impl HIKJAFBBPIP {
    cs_class!("HIKJAFBBPIP");

    cs_field!(
        MKFOEBBAKJA,
        "MKFOEBBAKJA",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );

    cs_field!(
        IENPEBIHOHF,
        "IENPEBIHOHF",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// EDJEDBLFIKE
pub struct EDJEDBLFIKE(pub usize);
impl EDJEDBLFIKE {
    cs_class!("EDJEDBLFIKE");

    // RPG.GameCore.GameComponentBase -> Type: GameEntity | Name: _OwnerRef | Offset: 0x10
    cs_field!(
        _OwnerRef,
        "_OwnerRef",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.BattleLineupData
pub struct RPG_GameCore_BattleLineupData(pub usize);
impl RPG_GameCore_BattleLineupData {
    cs_class!("RPG.GameCore.BattleLineupData");

    // RPG.GameCore.BattleLineupData -> Type: LineUpCharacter[] | Name: LightTeam | Offset: 0x48
    cs_field!(LightTeam, "LightTeam", self, |v| -> Il2CppArray {
        Il2CppArray(v.0)
    });

    // RPG.GameCore.BattleLineupData -> Type: LineUpCharacter[] | Name: ExtraTeam | Offset: 0x58
    cs_field!(ExtraTeam, "ExtraTeam", self, |v| -> Il2CppArray {
        Il2CppArray(v.0)
    });
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.TurnBasedGameMode
pub struct RPG_GameCore_TurnBasedGameMode(pub usize);
impl RPG_GameCore_TurnBasedGameMode {
    cs_class!("RPG.GameCore.TurnBasedGameMode");

    // RPG.GameCore.TurnBasedGameMode -> Type: BattleInstance | Name: <OwnerBattleInstanceRef>k__BackingField | Offset: 0x20
    cs_field!(
        _OwnerBattleInstanceRef_k__BackingField,
        "<OwnerBattleInstanceRef>k__BackingField",
        self,
        |v| -> RPG_GameCore_BattleInstance { RPG_GameCore_BattleInstance(v.0) }
    );

    // RPG.GameCore.TurnBasedGameMode -> Type: GameEntity | Name: _CurrentTurnActionEntity | Offset: 0x40
    cs_field!(
        _CurrentTurnActionEntity,
        "_CurrentTurnActionEntity",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );

    // RPG.GameCore.TurnBasedGameMode -> Type: int | Name: _WaveMonsterCurrentCount | Offset: 0x2C8
    cs_field!(
        _WaveMonsterCurrentCount,
        "_WaveMonsterCurrentCount",
        self,
        |v| -> i32 { v.unbox::<i32>() }
    );

    // RPG.GameCore.TurnBasedGameMode -> Type: FixPoint | Name: <ElapsedActionDelay>k__BackingField | Offset: 0x290
    cs_field!(
        _ElapsedActionDelay_k__BackingField,
        "<ElapsedActionDelay>k__BackingField",
        self,
        |v| -> RPG_GameCore_FixPoint {
            RPG_GameCore_FixPoint {
                m_rawValue: v.unbox::<i64>(),
            }
        }
    );

    // RPG.GameCore.TurnBasedGameMode -> Type: int | Name: <WaveMonsterMaxCount>k__BackingField | Offset: 0x2E4
    cs_field!(
        _WaveMonsterMaxCount_k__BackingField,
        "<WaveMonsterMaxCount>k__BackingField",
        self,
        |v| -> i32 { v.unbox::<i32>() }
    );

    // RPG.GameCore.TurnBasedGameMode -> Type: uint | Name: <ChallengeTurnLimit>k__BackingField | Offset: 0x344
    cs_field!(
        _ChallengeTurnLimit_k__BackingField,
        "<ChallengeTurnLimit>k__BackingField",
        self,
        |v| -> u32 { v.unbox::<u32>() }
    );

    // RPG.GameCore.TurnBasedGameMode -> Type: uint | Name: <CurrentWaveStageID>k__BackingField | Offset: 0x354
    cs_field!(
        _CurrentWaveStageID_k__BackingField,
        "<CurrentWaveStageID>k__BackingField",
        self,
        |v| -> u32 { v.unbox::<u32>() }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.TurnBasedAbilityComponent
pub struct RPG_GameCore_TurnBasedAbilityComponent(pub usize);
impl RPG_GameCore_TurnBasedAbilityComponent {
    cs_class!("RPG.GameCore.TurnBasedAbilityComponent");

    cs_method!(pub get_ability_mapped_skill, "GetAbilityMappedSkill", &["string"], Il2CppString, (ability_name: Il2CppString), self);
    cs_method!(pub get_property, "GetProperty", &["RPG.GameCore.AbilityProperty"], RPG_GameCore_FixPoint, (property: RPG_GameCore_AbilityProperty), self);
    cs_method!(pub try_check_limbo_wait_heal, "TryCheckLimboWaitHeal", &["RPG.GameCore.GameEntity"], bool, (attacker: RPG_GameCore_GameEntity), self);

    // RPG.GameCore.GameComponentBase -> Type: GameEntity | Name: _OwnerRef | Offset: 0x10
    cs_field!(
        _OwnerRef,
        "_OwnerRef",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );

    // RPG.GameCore.TurnBasedAbilityComponent -> Type: LIIAAAMMJIM[] | Name: _AbilityProperties | Offset: 0x30
    cs_field!(
        _AbilityProperties,
        "_AbilityProperties",
        self,
        |v| -> Il2CppArray { Il2CppArray(v.0) }
    );

    cs_field!(
        _KillerEntity,
        "_KillerEntity",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );

    // RPG.GameCore.TurnBasedAbilityComponent -> Type: CharacterDataComponent | Name: _CharacterDataRef | Offset: 0x1C0
    cs_field!(
        _CharacterDataRef,
        "_CharacterDataRef",
        self,
        |v| -> RPG_GameCore_CharacterDataComponent { RPG_GameCore_CharacterDataComponent(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.CharacterConfig
pub struct RPG_GameCore_CharacterConfig(pub usize);
impl RPG_GameCore_CharacterConfig {
    cs_class!("RPG.GameCore.CharacterConfig");

    cs_method!(pub get_skill_index_by_trigger_key, "GetSkillIndexByTriggerKey", &["string"], i32, (skill_name: Il2CppString), self);
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnityEngine_ProBuilder_EntityType {
    Detail = 0,
    Occluder = 1,
    Trigger = 2,
    Collider = 3,
    Mover = 4,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.TeamFormationComponent
pub struct RPG_GameCore_TeamFormationComponent(pub usize);
impl RPG_GameCore_TeamFormationComponent {
    cs_class!("RPG.GameCore.TeamFormationComponent");

    // RPG.GameCore.GameComponentBase -> Type: GameEntity | Name: _OwnerRef | Offset: 0x10
    cs_field!(
        _OwnerRef,
        "_OwnerRef",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );

    // RPG.GameCore.TeamFormationComponent -> Type: List<EDJEDBLFIKE> | Name: _TeamFormationDatas | Offset: 0x98
    cs_field!(
        _TeamFormationDatas,
        "_TeamFormationDatas",
        self,
        |v| -> List { List(v.0) }
    );

    // RPG.GameCore.TeamFormationComponent -> Type: TeamType | Name: _Team | Offset: 0xF4
    cs_field!(_Team, "_Team", self, |v| -> RPG_GameCore_TeamType {
        v.unbox::<RPG_GameCore_TeamType>()
    });
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.Client.AvatarServantData
pub struct RPG_Client_AvatarServantData(pub usize);
impl RPG_Client_AvatarServantData {
    cs_class!("RPG.Client.AvatarServantData");
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.MonsterRowData
pub struct RPG_GameCore_MonsterRowData(pub usize);
impl RPG_GameCore_MonsterRowData {
    cs_class!("RPG.GameCore.MonsterRowData");

    cs_method!(pub get_level, "get_Level", &[], u32, (), self);

    // RPG.GameCore.MonsterRowData -> Type: MonsterRow | Name: _Row | Offset: 0xA8
    cs_field!(_Row, "_Row", self, |v| -> RPG_GameCore_MonsterRow {
        RPG_GameCore_MonsterRow(v.0)
    });
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.BattleEventSkillRowData
pub struct RPG_GameCore_BattleEventSkillRowData(pub usize);
impl RPG_GameCore_BattleEventSkillRowData {
    cs_class!("RPG.GameCore.BattleEventSkillRowData");
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.Client.AvatarData
pub struct RPG_Client_AvatarData(pub usize);
impl RPG_Client_AvatarData {
    cs_class!("RPG.Client.AvatarData");

    cs_property!(pub avatarname, "get_AvatarName", Il2CppString, self);
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.MonsterRow
pub struct RPG_GameCore_MonsterRow(pub usize);
impl RPG_GameCore_MonsterRow {
    cs_class!("RPG.GameCore.MonsterRow");

    // RPG.GameCore.MonsterRow -> Type: TextID | Name: MonsterName | Offset: 0x90
    cs_field!(
        MonsterName,
        "MonsterName",
        self,
        |v| -> RPG_Client_TextID_Boxed { RPG_Client_TextID_Boxed(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.AvatarSkillRowData
pub struct RPG_GameCore_AvatarSkillRowData(pub usize);
impl RPG_GameCore_AvatarSkillRowData {
    cs_class!("RPG.GameCore.AvatarSkillRowData");
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.MonsterDataComponent
pub struct RPG_GameCore_MonsterDataComponent(pub usize);
impl RPG_GameCore_MonsterDataComponent {
    cs_class!("RPG.GameCore.MonsterDataComponent");

    cs_method!(pub get_monster_id, "GetMonsterID", &[], u32, (), self);

    // RPG.GameCore.GameComponentBase -> Type: GameEntity | Name: _OwnerRef | Offset: 0x10
    cs_field!(
        _OwnerRef,
        "_OwnerRef",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );

    // RPG.GameCore.MonsterDataComponent -> Type: MonsterRowData | Name: _MonsterRowData | Offset: 0xE0
    cs_field!(
        _MonsterRowData,
        "_MonsterRowData",
        self,
        |v| -> RPG_GameCore_MonsterRowData { RPG_GameCore_MonsterRowData(v.0) }
    );

    // RPG.GameCore.MonsterDataComponent -> Type: FixPoint | Name: _DefaultMaxHP | Offset: 0x118
    cs_field!(
        _DefaultMaxHP,
        "_DefaultMaxHP",
        self,
        |v| -> RPG_GameCore_FixPoint {
            RPG_GameCore_FixPoint {
                m_rawValue: v.unbox::<i64>(),
            }
        }
    );
}


#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.MonsterDataComponent
pub struct RPG_GameCore_AvatarDataComponent(pub usize);
impl RPG_GameCore_AvatarDataComponent {
    cs_class!("RPG.GameCore.AvatarDataComponent");

    // RPG.GameCore.GameComponentBase -> Type: GameEntity | Name: _OwnerRef | Offset: 0x10
    cs_field!(
        _OwnerRef,
        "_OwnerRef",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.ServantSkillRowData
pub struct RPG_GameCore_ServantSkillRowData(pub usize);
impl RPG_GameCore_ServantSkillRowData {
    cs_class!("RPG.GameCore.ServantSkillRowData");
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.GameComponentBase
pub struct RPG_GameCore_GameComponentBase(pub usize);
impl RPG_GameCore_GameComponentBase {
    cs_class!("RPG.GameCore.GameComponentBase");
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RPG_GameCore_AbilityProperty {
    Unknow = 0,
    MaxHP = 1,
    BaseHP = 2,
    HPAddedRatio = 3,
    HPDelta = 4,
    HPConvert = 5,
    DirtyHPDelta = 6,
    DirtyHPRatio = 7,
    RallyHP = 8,
    NegativeHP = 9,
    CurrentHP = 10,
    MaxSP = 11,
    CurrentSP = 12,
    MaxSpecialSP = 13,
    CurrentSpecialSP = 14,
    AdditionalBP = 15,
    Attack = 16,
    BaseAttack = 17,
    AttackAddedRatio = 18,
    AttackDelta = 19,
    AttackConvert = 20,
    Defence = 21,
    BaseDefence = 22,
    DefenceAddedRatio = 23,
    DefenceDelta = 24,
    DefenceConvert = 25,
    DefenceOverride = 26,
    Level = 27,
    Promotion = 28,
    Rank = 29,
    Speed = 30,
    BaseSpeed = 31,
    SpeedAddedRatio = 32,
    SpeedDelta = 33,
    SpeedConvert = 34,
    SpeedOverride = 35,
    ActionDelay = 36,
    ActionDelayAddedRatio = 37,
    ActionDelayAddAttenuation = 38,
    MaxStance = 39,
    CurrentStance = 40,
    Level_AllDamageAddedRatio = 41,
    AllDamageTypeAddedRatio = 42,
    AllDamageReduce = 43,
    DotDamageAddedRatio = 44,
    FatigueRatio = 45,
    CriticalChance = 46,
    CriticalChanceBase = 47,
    CriticalChanceConvert = 48,
    CriticalDamage = 49,
    CriticalDamageBase = 50,
    CriticalDamageConvert = 51,
    CriticalResistance = 52,
    PhysicalAddedRatio = 53,
    FireAddedRatio = 54,
    IceAddedRatio = 55,
    ThunderAddedRatio = 56,
    QuantumAddedRatio = 57,
    ImaginaryAddedRatio = 58,
    WindAddedRatio = 59,
    PhysicalResistance = 60,
    FireResistance = 61,
    IceResistance = 62,
    ThunderResistance = 63,
    QuantumResistance = 64,
    ImaginaryResistance = 65,
    WindResistance = 66,
    PhysicalResistanceBase = 67,
    FireResistanceBase = 68,
    IceResistanceBase = 69,
    ThunderResistanceBase = 70,
    QuantumResistanceBase = 71,
    ImaginaryResistanceBase = 72,
    WindResistanceBase = 73,
    PhysicalResistanceDelta = 74,
    FireResistanceDelta = 75,
    IceResistanceDelta = 76,
    ThunderResistanceDelta = 77,
    QuantumResistanceDelta = 78,
    ImaginaryResistanceDelta = 79,
    WindResistanceDelta = 80,
    AllDamageTypeResistance = 81,
    PhysicalPenetrate = 82,
    FirePenetrate = 83,
    IcePenetrate = 84,
    ThunderPenetrate = 85,
    QuantumPenetrate = 86,
    ImaginaryPenetrate = 87,
    WindPenetrate = 88,
    AllDamageTypePenetrate = 89,
    PhysicalTakenRatio = 90,
    FireTakenRatio = 91,
    IceTakenRatio = 92,
    ThunderTakenRatio = 93,
    QuantumTakenRatio = 94,
    ImaginaryTakenRatio = 95,
    WindTakenRatio = 96,
    AllDamageTypeTakenRatio = 97,
    Monster_DamageTakenRatio = 98,
    PhysicalAbsorb = 99,
    FireAbsorb = 100,
    IceAbsorb = 101,
    ThunderAbsorb = 102,
    QuantumAbsorb = 103,
    ImaginaryAbsorb = 104,
    WindAbsorb = 105,
    MinimumFatigueRatio = 106,
    ForceStanceBreakRatio = 107,
    StanceBreakAddedRatio = 108,
    StanceBreakResistance = 109,
    StanceBreakTakenRatio = 110,
    PhysicalStanceBreakTakenRatio = 111,
    FireStanceBreakTakenRatio = 112,
    IceStanceBreakTakenRatio = 113,
    ThunderStanceBreakTakenRatio = 114,
    WindStanceBreakTakenRatio = 115,
    QuantumStanceBreakTakenRatio = 116,
    ImaginaryStanceBreakTakenRatio = 117,
    StanceWeakAddedRatio = 118,
    StanceDefaultAddedRatio = 119,
    HealRatio = 120,
    HealRatioBase = 121,
    HealRatioConvert = 122,
    HealTakenRatio = 123,
    Shield = 124,
    MaxShield = 125,
    ShieldAddedRatio = 126,
    ShieldTakenRatio = 127,
    StatusProbability = 128,
    StatusProbabilityBase = 129,
    StatusProbabilityConvert = 130,
    StatusResistance = 131,
    StatusResistanceBase = 132,
    StatusResistanceConvert = 133,
    SPRatio = 134,
    SPRatioBase = 135,
    SPRatioConvert = 136,
    SPRatioOverride = 137,
    BreakDamageAddedRatio = 138,
    BreakDamageAddedRatioBase = 139,
    BreakDamageAddedRatioConvert = 140,
    BreakDamageExtraAddedRatio = 141,
    PhysicalStanceBreakResistance = 142,
    FireStanceBreakResistance = 143,
    IceStanceBreakResistance = 144,
    ThunderStanceBreakResistance = 145,
    WindStanceBreakResistance = 146,
    QuantumStanceBreakResistance = 147,
    ImaginaryStanceBreakResistance = 148,
    AggroBase = 149,
    AggroAddedRatio = 150,
    AggroDelta = 151,
    RelicValueExtraAdditionRatio = 152,
    EquipValueExtraAdditionRatio = 153,
    EquipExtraRank = 154,
    AvatarExtraRank = 155,
    Combo = 156,
    NormalBattleCount = 157,
    ExtraAttackAddedRatio1 = 158,
    ExtraAttackAddedRatio2 = 159,
    ExtraAttackAddedRatio3 = 160,
    ExtraAttackAddedRatio4 = 161,
    ExtraDefenceAddedRatio1 = 162,
    ExtraDefenceAddedRatio2 = 163,
    ExtraDefenceAddedRatio3 = 164,
    ExtraDefenceAddedRatio4 = 165,
    ExtraHPAddedRatio1 = 166,
    ExtraHPAddedRatio2 = 167,
    ExtraHPAddedRatio3 = 168,
    ExtraHPAddedRatio4 = 169,
    ExtraHealAddedRatio = 170,
    ExtraAllDamageTypeAddedRatio1 = 171,
    ExtraAllDamageTypeAddedRatio2 = 172,
    ExtraAllDamageTypeAddedRatio3 = 173,
    ExtraAllDamageTypeAddedRatio4 = 174,
    ExtraAllDamageReduce = 175,
    ExtraShieldAddedRatio = 176,
    ExtraSpeedAddedRatio1 = 177,
    ExtraSpeedAddedRatio2 = 178,
    ExtraSpeedAddedRatio3 = 179,
    ExtraSpeedAddedRatio4 = 180,
    ExtraLuckChance = 181,
    ExtraLuckDamage = 182,
    ExtraFrontPowerBase = 183,
    ExtraFrontPowerAddedRatio1 = 184,
    ExtraFrontPowerAddedRatio2 = 185,
    ExtraBackPowerBase = 186,
    ExtraBackPowerAddedRatio1 = 187,
    ExtraBackPowerAddedRatio2 = 188,
    ExtraUltraDamageAddedRatio1 = 189,
    ExtraSkillDamageAddedRatio1 = 190,
    ExtraNormalDamageAddedRatio1 = 191,
    ExtraInsertDamageAddedRatio1 = 192,
    Count = 193,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.SkillCharacterComponent
pub struct RPG_GameCore_SkillCharacterComponent(pub usize);
impl RPG_GameCore_SkillCharacterComponent {
    cs_class!("RPG.GameCore.SkillCharacterComponent");

    cs_method!(pub get_skill_data, "GetSkillData", &["int", "int"], RPG_GameCore_SkillData, (skill_index: i32, extra_use_param: i32), self);

    // RPG.GameCore.GameComponentBase -> Type: GameEntity | Name: _OwnerRef | Offset: 0x10
    cs_field!(
        _OwnerRef,
        "_OwnerRef",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );

    // RPG.GameCore.SkillCharacterComponent -> Type: CharacterDataComponent | Name: _CharacterDataRef | Offset: 0xE0
    cs_field!(
        _CharacterDataRef,
        "_CharacterDataRef",
        self,
        |v| -> RPG_GameCore_CharacterDataComponent { RPG_GameCore_CharacterDataComponent(v.0) }
    );
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RPG_GameCore_EntityType {
    None = 0,
    Avatar = 1,
    Monster = 2,
    LocalPlayer = 3,
    NPC = 4,
    NPCMonster = 5,
    StoryCharacter = 6,
    Prop = 7,
    Mission = 8,
    LevelEntity = 9,
    Neutral = 10,
    AtmoNpc = 11,
    BattleEvent = 12,
    TutorialEntity = 13,
    Team = 14,
    Partner = 15,
    LevelGraph = 16,
    Snapshot = 17,
    TeamFormation = 18,
    Model = 19,
    UICamera = 20,
    District = 21,
    GlobalShield = 22,
    CustomData = 23,
    Simple = 24,
    PuzzleGameObjectProp = 25,
    PerformanceLevelGraph = 26,
    Group = 27,
    ChessCharacter = 28,
    ChessTerrain = 29,
    SummonUnit = 30,
    LittleGameInstance = 31,
    Servant = 32,
    PreviewShow = 33,
    LittleGameContainer = 34,
    LittleGameViewProxy = 35,
    GridFightBackend = 36,
    DummyEntity = 37,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.BattleEventDataComponent
pub struct RPG_GameCore_BattleEventDataComponent(pub usize);
impl RPG_GameCore_BattleEventDataComponent {
    cs_class!("RPG.GameCore.BattleEventDataComponent");

    // RPG.GameCore.GameComponentBase -> Type: GameEntity | Name: _OwnerRef | Offset: 0x10
    cs_field!(
        _OwnerRef,
        "_OwnerRef",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );

    // RPG.GameCore.BattleEventDataComponent -> Type: GameEntity | Name: <SourceCaster>k__BackingField | Offset: 0xF8
    cs_field!(
        _SourceCaster_k__BackingField,
        "<SourceCaster>k__BackingField",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.SkillData
pub struct RPG_GameCore_SkillData(pub usize);
impl RPG_GameCore_SkillData {
    cs_class!("RPG.GameCore.SkillData");

    // RPG.GameCore.SkillData -> Type: ICharacterSkillRowData | Name: RowData | Offset: 0x20
    cs_field!(
        RowData,
        "RowData",
        self,
        |v| -> RPG_GameCore_ICharacterSkillRowData { RPG_GameCore_ICharacterSkillRowData(v.0) }
    );

    cs_field!(SkillConfigID, "SkillConfigID", self, |v| -> u32 {
        v.unbox::<u32>()
    });
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.AbilityConfig
pub struct RPG_GameCore_AbilityConfig(pub usize);
impl RPG_GameCore_AbilityConfig {
    cs_class!("RPG.GameCore.AbilityConfig");
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.LineUpCharacter
pub struct RPG_GameCore_LineUpCharacter(pub usize);
impl RPG_GameCore_LineUpCharacter {
    cs_class!("RPG.GameCore.LineUpCharacter");

    // RPG.GameCore.LineUpCharacter -> Type: uint | Name: CharacterID | Offset: 0x50
    cs_field!(CharacterID, "CharacterID", self, |v| -> u32 {
        v.unbox::<u32>()
    });
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.Client.TextID
pub struct RPG_Client_TextID_Boxed(pub usize);
impl RPG_Client_TextID_Boxed {
    cs_class!("RPG.Client.TextID");

    // RPG.Client.TextID -> Type: int | Name: hash | Offset: 0x0
    cs_field!(hash, "hash", self, |v| -> i32 { v.unbox::<i32>() });

    pub fn unbox(&self) -> RPG_Client_TextID {
        Il2CppObject(self.0).unbox::<RPG_Client_TextID>()
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.CharacterDataComponent
pub struct RPG_GameCore_CharacterDataComponent(pub usize);
impl RPG_GameCore_CharacterDataComponent {
    cs_class!("RPG.GameCore.CharacterDataComponent");

    // RPG.GameCore.GameComponentBase -> Type: GameEntity | Name: _OwnerRef | Offset: 0x10
    cs_field!(
        _OwnerRef,
        "_OwnerRef",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );

    // RPG.GameCore.CharacterDataComponent -> Type: CharacterConfig | Name: <JsonConfig>k__BackingField | Offset: 0x88
    cs_field!(
        _JsonConfig_k__BackingField,
        "<JsonConfig>k__BackingField",
        self,
        |v| -> RPG_GameCore_CharacterConfig { RPG_GameCore_CharacterConfig(v.0) }
    );

    // RPG.GameCore.CharacterDataComponent -> Type: GameEntity | Name: Summoner | Offset: 0xA0
    cs_field!(Summoner, "Summoner", self, |v| -> RPG_GameCore_GameEntity {
        RPG_GameCore_GameEntity(v.0)
    });
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StageBudgetTool_EntityType {
    Prpp_S = 0,
    Prop_A = 1,
    Prop_B = 2,
    Prop_C = 3,
    Prop_D = 4,
    Monster_1 = 5,
    Monster_2 = 6,
    Monster_3 = 7,
    Monster_4 = 8,
    Monster_5 = 9,
    NPC = 10,
    NPC_Avatar = 11,
    NPC_Special = 12,
    NPC_Monster = 13,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.FixPoint
pub struct RPG_GameCore_FixPoint {
    pub m_rawValue: i64,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// BIHKCLPKJBC
pub struct BIHKCLPKJBC(pub usize);
impl BIHKCLPKJBC {
    cs_class!("BIHKCLPKJBC");

    // BIHKCLPKJBC -> Type: FixPoint | Name: GDMICPLJAKA | Offset: 0x340
    cs_field!(
        GDMICPLJAKA,
        "GDMICPLJAKA",
        self,
        |v| -> RPG_GameCore_FixPoint {
            RPG_GameCore_FixPoint {
                m_rawValue: v.unbox::<i64>(),
            }
        }
    );

    // BIHKCLPKJBC -> Type: AttackType | Name: JDDOEMNAMFJ | Offset: 0x434
    cs_field!(
        JDDOEMNAMFJ,
        "JDDOEMNAMFJ",
        self,
        |v| -> RPG_GameCore_AttackType { v.unbox::<RPG_GameCore_AttackType>() }
    );
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RPG_GameCore_AttackType {
    Unknown = 0,
    Normal = 1,
    BPSkill = 2,
    Ultra = 3,
    QTE = 4,
    DOT = 5,
    Pursued = 6,
    Maze = 7,
    MazeNormal = 8,
    Insert = 9,
    ElementDamage = 10,
    Level = 11,
    Servant = 12,
    TrueDamage = 13,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.BattleInstance
pub struct RPG_GameCore_BattleInstance(pub usize);
impl RPG_GameCore_BattleInstance {
    cs_class!("RPG.GameCore.BattleInstance");

    // RPG.GameCore.BattleInstance -> Type: GameWorld | Name: _GameWorld | Offset: 0x38
    cs_field!(
        _GameWorld,
        "_GameWorld",
        self,
        |v| -> RPG_GameCore_GameWorld { RPG_GameCore_GameWorld(v.0) }
    );

    cs_field!(
        _BattleLineupData,
        "_BattleLineupData",
        self,
        |v| -> RPG_GameCore_BattleLineupData { RPG_GameCore_BattleLineupData(v.0) }
    );

    
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.GameEntity
pub struct RPG_GameCore_GameEntity(pub usize);
impl RPG_GameCore_GameEntity {
    cs_class!("RPG.GameCore.GameEntity");

    cs_method!(pub get_component, "GetComponent", &["System.Type"], RPG_GameCore_GameComponentBase, (ty: RuntimeType), self);

    // RPG.GameCore.GameEntity -> Type: GameEntity.GameComponentList | Name: _ComponentList | Offset: 0x48
    cs_field!(_ComponentList, "_ComponentList", self, |v| -> Il2CppArray {
        Il2CppArray(v.0)
    });

    // RPG.GameCore.GameEntity -> Type: GameEntity.GameComponentList | Name: _ComponentList | Offset: 0x48
    cs_field!(_AliveState, "_AliveState", self, |v| -> RPG_GameCore_AliveState {
        v.unbox::<RPG_GameCore_AliveState>()
    });


    // RPG.GameCore.GameEntity -> Type: TeamType | Name: _Team | Offset: 0xF0
    cs_field!(_Team, "_Team", self, |v| -> RPG_GameCore_TeamType {
        v.unbox::<RPG_GameCore_TeamType>()
    });

    // RPG.GameCore.GameEntity -> Type: uint | Name: <RuntimeID>k__BackingField | Offset: 0xF4
    cs_field!(
        _RuntimeID_k__BackingField,
        "<RuntimeID>k__BackingField",
        self,
        |v| -> u32 { v.unbox::<u32>() }
    );

    // RPG.GameCore.GameEntity -> Type: EntityType | Name: _EntityType | Offset: 0x10C
    cs_field!(
        _EntityType,
        "_EntityType",
        self,
        |v| -> RPG_GameCore_EntityType { v.unbox::<RPG_GameCore_EntityType>() }
    );

    cs_field!(
        _OwnerWorldRef,
        "_OwnerWorldRef",
        self,
        |v| -> RPG_GameCore_GameWorld { RPG_GameCore_GameWorld(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.Client.ModuleManager
pub struct RPG_Client_ModuleManager(pub usize);
impl RPG_Client_ModuleManager {
    cs_class!("RPG.Client.ModuleManager");

    // RPG.Client.ModuleManager -> Type: AvatarModule | Name: AvatarModule | Offset: 0x168
    cs_field!(
        AvatarModule,
        "AvatarModule",
        self,
        |v| -> RPG_Client_AvatarModule { RPG_Client_AvatarModule(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// OHCFAIPOEAH
pub struct OHCFAIPOEAH(pub usize);
impl OHCFAIPOEAH {
    cs_class!("OHCFAIPOEAH");

    // OHCFAIPOEAH -> Type: AFBBBCMBEME | Name: GMJNLMGADNN | Offset: 0x10
    cs_field!(GMJNLMGADNN, "GMJNLMGADNN", self, |v| -> AFBBBCMBEME {
        AFBBBCMBEME(v.0)
    });

    // OHCFAIPOEAH -> Type: SkillCharacterComponent | Name: GMIEIJLDHAM | Offset: 0x78
    cs_field!(
        GMIEIJLDHAM,
        "GMIEIJLDHAM",
        self,
        |v| -> RPG_GameCore_SkillCharacterComponent { RPG_GameCore_SkillCharacterComponent(v.0) }
    );

    // OHCFAIPOEAH -> Type: TurnBasedAbilityComponent | Name: DJJDPEJEHAJ | Offset: 0x80
    cs_field!(
        DJJDPEJEHAJ,
        "DJJDPEJEHAJ",
        self,
        |v| -> RPG_GameCore_TurnBasedAbilityComponent {
            RPG_GameCore_TurnBasedAbilityComponent(v.0)
        }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.ICharacterSkillRowData
pub struct RPG_GameCore_ICharacterSkillRowData(pub usize);
impl RPG_GameCore_ICharacterSkillRowData {
    cs_class!("RPG.GameCore.ICharacterSkillRowData");

    cs_property!(pub skillname, "get_SkillName", RPG_Client_TextID, self);
    cs_property!(pub attacktype, "get_AttackType", enumtype RPG_GameCore_AttackType, self);
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.Client.AvatarModule
pub struct RPG_Client_AvatarModule(pub usize);
impl RPG_Client_AvatarModule {
    cs_class!("RPG.Client.AvatarModule");

    cs_method!(pub get_avatar, "GetAvatar", &["uint"], RPG_Client_AvatarData, (avatar_id: u32), self);
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.GameWorld
pub struct RPG_GameCore_GameWorld(pub usize);
impl RPG_GameCore_GameWorld {
    cs_class!("RPG.GameCore.GameWorld");

    // RPG.GameCore.GameWorld -> Type: EntityManager | Name: _EntityManager | Offset: 0xC8
    cs_field!(
        _EntityManager,
        "_EntityManager",
        self,
        |v| -> RPG_GameCore_EntityManager { RPG_GameCore_EntityManager(v.0) }
    );

    cs_field!(
        _BattleInstanceRef_k__BackingField,
        "<BattleInstanceRef>k__BackingField",
        self,
        |v| -> RPG_GameCore_BattleInstance { RPG_GameCore_BattleInstance(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.EntityManager
pub struct RPG_GameCore_EntityManager(pub usize);
impl RPG_GameCore_EntityManager {
    cs_class!("RPG.GameCore.EntityManager");

    cs_method!(pub get_entity_by_runtime_id, "GetEntityByRuntimeID", &["uint"], RPG_GameCore_GameEntity, (runtime_id: u32), self);
    cs_method!(pub get_entity_summoner, "_GetEntitySummoner", &["RPG.GameCore.GameEntity"], RPG_GameCore_GameEntity, (entity: RPG_GameCore_GameEntity), self);
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.Client.GlobalVars
pub struct RPG_Client_GlobalVars(pub usize);
impl RPG_Client_GlobalVars {
    cs_class!("RPG.Client.GlobalVars");

    // RPG.Client.GlobalVars -> Type: ModuleManager | Name: s_ModuleManager | Offset: 0xEB8
    cs_field!(
        s_ModuleManager,
        "s_ModuleManager",
        |v| -> RPG_Client_ModuleManager { RPG_Client_ModuleManager(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.Client.TextmapStatic
pub struct RPG_Client_TextmapStatic(pub usize);
impl RPG_Client_TextmapStatic {
    cs_class!("RPG.Client.TextmapStatic");

    cs_method!(pub get_text, "GetText", &["RPG.Client.TextID", "System.Object[]"], Il2CppString, (id: RPG_Client_TextID, replace_params: Il2CppObject));
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.Client.UIGameEntityUtils
pub struct RPG_Client_UIGameEntityUtils(pub usize);
impl RPG_Client_UIGameEntityUtils {
    cs_class!("RPG.Client.UIGameEntityUtils");

    cs_method!(pub get_avatar_id, "GetAvatarID", &["RPG.GameCore.GameEntity"], u32, (entity: RPG_GameCore_GameEntity));
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.AbilityStatic
pub struct RPG_GameCore_AbilityStatic(pub usize);
impl RPG_GameCore_AbilityStatic {
    cs_class!("RPG.GameCore.AbilityStatic");

    cs_method!(pub get_actual_owner, "GetActualOwner", &["RPG.GameCore.GameEntity"], RPG_GameCore_GameEntity, (entity: RPG_GameCore_GameEntity));
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// MiHoYo.SDK.Win.MiHoYoSDKDll
pub struct MiHoYo_SDK_Win_MiHoYoSDKDll(pub usize);
impl MiHoYo_SDK_Win_MiHoYoSDKDll {
    cs_class!("MiHoYo.SDK.Win.MiHoYoSDKDll");
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// UnityEngine.Application
pub struct UnityEngine_Application(pub usize);
impl UnityEngine_Application {
    cs_class!("UnityEngine.Application");

    cs_method!(pub set_target_framerate, "set_targetFrameRate", &["int"], (), (fps: i32));
    cs_method!(pub get_target_framerate, "get_targetFrameRate", &[], i32, ());
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// KMCALMJKMII
pub struct KMCALMJKMII(pub usize);
impl KMCALMJKMII {
    cs_class!("KMCALMJKMII");
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.Client.BattleAssetPreload
pub struct RPG_Client_BattleAssetPreload(pub usize);
impl RPG_Client_BattleAssetPreload {
    cs_class!("RPG.Client.BattleAssetPreload");

    // RPG.Client.BattleAssetPreload -> Type: BattleLineupData | Name: _LineupData | Offset: 0x68
    cs_field!(
        _LineupData,
        "_LineupData",
        self,
        |v| -> RPG_GameCore_BattleLineupData { RPG_GameCore_BattleLineupData(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.ServantDataComponent
pub struct RPG_GameCore_ServantDataComponent(pub usize);
impl RPG_GameCore_ServantDataComponent {
    cs_class!("RPG.GameCore.ServantDataComponent");

    // RPG.GameCore.GameComponentBase -> Type: GameEntity | Name: _OwnerRef | Offset: 0x10
    cs_field!(
        _OwnerRef,
        "_OwnerRef",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );

    // RPG.GameCore.ServantDataComponent -> Type: ServantRowData | Name: _ServantRowData | Offset: 0xD8
    cs_field!(
        _ServantRowData,
        "_ServantRowData",
        self,
        |v| -> RPG_GameCore_ServantRowData { RPG_GameCore_ServantRowData(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.ServantRowData
pub struct RPG_GameCore_ServantRowData(pub usize);
impl RPG_GameCore_ServantRowData {
    cs_class!("RPG.GameCore.ServantRowData");

    // RPG.GameCore.ServantRowData -> Type: AvatarServantRow | Name: _Row | Offset: 0x70
    cs_field!(_Row, "_Row", self, |v| -> RPG_GameCore_AvatarServantRow {
        RPG_GameCore_AvatarServantRow(v.0)
    });
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.AvatarServantRow
pub struct RPG_GameCore_AvatarServantRow(pub usize);
impl RPG_GameCore_AvatarServantRow {
    cs_class!("RPG.GameCore.AvatarServantRow");

    // RPG.GameCore.AvatarServantRow -> Type: uint | Name: ServantID | Offset: 0x88
    cs_field!(ServantID, "ServantID", self, |v| -> u32 {
        v.unbox::<u32>()
    });

    // RPG.GameCore.AvatarServantRow -> Type: TextID | Name: ServantName | Offset: 0x98
    cs_field!(
        ServantName,
        "ServantName",
        self,
        |v| -> RPG_Client_TextID_Boxed { RPG_Client_TextID_Boxed(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.AbilityComponent
pub struct RPG_GameCore_AbilityComponent(pub usize);
impl RPG_GameCore_AbilityComponent {
    cs_class!("RPG.GameCore.AbilityComponent");

    // RPG.GameCore.GameComponentBase -> Type: GameEntity | Name: _OwnerRef | Offset: 0x10
    cs_field!(
        _OwnerRef,
        "_OwnerRef",
        self,
        |v| -> RPG_GameCore_GameEntity { RPG_GameCore_GameEntity(v.0) }
    );

    // RPG.GameCore.AbilityComponent -> Type: List<MJMPGBOFFMC> | Name: _ModifierList | Offset: 0x28
    cs_field!(_ModifierList, "_ModifierList", self);
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.StatusExcelTable
pub struct RPG_GameCore_StatusExcelTable(pub usize);
impl RPG_GameCore_StatusExcelTable {
    cs_class!("RPG.GameCore.StatusExcelTable");

    cs_method!(pub get_by_modifier_name, "GetByModifierName",  &["string"], RPG_GameCore_StatusRow, (modifier_name: Il2CppString));
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.StatusRow
pub struct RPG_GameCore_StatusRow(pub usize);
impl RPG_GameCore_StatusRow {
    cs_class!("RPG.GameCore.StatusRow");

    // RPG.GameCore.StatusRow -> Type: TextID | Name: StatusName | Offset: 0x38
    cs_field!(
        StatusName,
        "StatusName",
        self,
        |v| -> RPG_Client_TextID_Boxed { RPG_Client_TextID_Boxed(v.0) }
    );

    // RPG.GameCore.StatusRow -> Type: TextID | Name: StatusDesc | Offset: 0x48
    cs_field!(
        StatusDesc,
        "StatusDesc",
        self,
        |v| -> RPG_Client_TextID_Boxed { RPG_Client_TextID_Boxed(v.0) }
    );
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// RPG.GameCore.TurnBasedModifierInstance
pub struct RPG_GameCore_TurnBasedModifierInstance(pub usize);
impl RPG_GameCore_TurnBasedModifierInstance {
    cs_class!("RPG.GameCore.TurnBasedModifierInstance");

    cs_property!(pub key_for_status_config, "get_KeyForStatusConfig", Il2CppString, self);
}
