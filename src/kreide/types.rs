#![allow(unused_imports)]
use crate::kreide::client::*;
use crate::kreide::gamecore::*;
use crate::kreide::native_types::*;
use std::ffi::c_void;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OLHMAHMMBNN {
    pub JBHFMCDFPPL: *const c_void,                    // 0x0
    pub FKHHOBBFMEH: *const NativeString,              // 0x8
    pub BAICECGKLBG: *const c_void,                    // 0x10
    pub OAAMONICNLE: *const NativeArray<NativeObject>, // 0x18
    pub MOIPJLBAODO: i32,                              // 0x20
    pub NMJEMHAMIHD: i32,                              // 0x24
    pub AHNHNPOCNDJ: bool,                             // 0x28
    pub OBNPIDPHFDE: bool,                             // 0x29
    pub EKFIDPFOILC: bool,                             // 0x2a
    pub NMKBJGEONOJ: bool,                             // 0x2b
    pub EDIDAHIELAG: *const c_void,                    // 0x30
    pub OKHBBILFBND: [u8; 0x2],                        // 0x38
    pub LDJAAEOOOLC: [u8; 0x2],                        // 0x3a
    pub MHFEBJINMBP: bool,                             // 0x3c
    pub AJENNABILJC: bool,                             // 0x3d
    pub GJIMBAPCJLF: bool,                             // 0x3e
    pub ODNBNHFLMCD: *const c_void,                    // 0x40
    pub FGJEHAKCLNL: *const c_void,                    // 0x48
    pub KGKBLOJMDPH: bool,                             // 0x50
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NOPBAAAGGLA {
    pub native_object: NativeObject,
    pub JKCOIOLCMEP: *const TurnBasedAbilityComponent, // 0x10
    pub HKFGOHGKOGK: *const c_void,                    // 0x18
    pub LGGEDDMACDF: *const NativeString,              // 0x20
    pub PBHCGDFPEED: *const c_void,                    // 0x28
    pub KNDJNKNHFFG: *const TurnBasedAbilityComponent, // 0x30
    pub BEAJGANIDLJ: *const c_void,                    // 0x38
    pub FKKDFMPMJHG: *const NativeArray<NativeObject>, // 0x40
    pub AAHMMHBHMFN: [u8; 0x90],                       // 0x48
    pub JODAJBNCCNP: *const NativeArray<NativeObject>, // 0xd8
    pub MDEHKOOKJCK: *const NativeList<NativeObject>,  // 0xe0
    pub ALOGNJIBIPG: FixPoint,                         // 0xe8
    pub BJAEJMLMJCL: FixPoint,                         // 0xf0
    pub LJGPDLDGCEO: FixPoint,                         // 0xf8
    pub PJLPGAGKIDE: FixPoint,                         // 0x100
    pub CMNBOEIDAOD: FixPoint,                         // 0x108
    pub MJMDGNPPILN: FixPoint,                         // 0x110
    pub MNGPDEOEHPE: FixPoint,                         // 0x118
    pub MKMILJKLJON: [u8; 0x58],                       // 0x120
    pub KLMAGCLFBAO: FixPoint,                         // 0x178
    pub MKIMEBNOEGI: FixPoint,                         // 0x180
    pub CAILJEGIDKL: FixPoint,                         // 0x188
    pub DINCHAHPEAC: FixPoint,                         // 0x190
    pub JHOHCEFOJNB: FixPoint,                         // 0x198
    pub BGBOFNMKDNJ: FixPoint,                         // 0x1a0
    pub HNJBAFCNNDD: FixPoint,                         // 0x1a8
    pub OHBMMFAFMDP: FixPoint,                         // 0x1b0
    pub DBBDIMCJIKE: FixPoint,                         // 0x1b8
    pub FFCGIMAMDPP: FixPoint,                         // 0x1c0
    pub GCNOMMHFPOG: FixPoint,                         // 0x1c8
    pub CCLFKIPGJOG: FixPoint,                         // 0x1d0
    pub DGFBMAPFPNH: FixPoint,                         // 0x1d8
    pub GLGFEKEMMJJ: FixPoint,                         // 0x1e0
    pub KPELFJICFDH: FixPoint,                         // 0x1e8
    pub COIDNPMCCFG: FixPoint,                         // 0x1f0
    pub PAIGBKBOKDI: FixPoint,                         // 0x1f8
    pub GBENLNNEIJM: FixPoint,                         // 0x200
    pub FMMBMJKNAHI: FixPoint,                         // 0x208
    pub GNMAKKBFOCH: FixPoint,                         // 0x210
    pub CINNHMENLIJ: FixPoint,                         // 0x218
    pub EFAAJEAENFF: FixPoint,                         // 0x220
    pub PJNEJPNBNMP: FixPoint,                         // 0x228
    pub BBNMJNPDOCP: FixPoint,                         // 0x230
    pub FNDCNMHMCIC: FixPoint,                         // 0x238
    pub BLFCEOMPDKK: FixPoint,                         // 0x240
    pub GAALBDHLFOG: FixPoint,                         // 0x248
    pub MKNDMBOCCBO: FixPoint,                         // 0x250
    pub ODBPMMGBKGA: FixPoint,                         // 0x258
    pub OEPAPFDLMML: FixPoint,                         // 0x260
    pub EBDJIHNKAOC: FixPoint,                         // 0x268
    pub APDDLHNGGIM: AttackType,                       // 0x270
    pub COKMLMJPKLH: u32,                              // 0x274
    pub POLANGDKOKH: FixPoint,                         // 0x278
    pub GCFCCDPIACO: FixPoint,                         // 0x280
    pub MAKENPDPHDN: FixPoint,                         // 0x288
    pub FFFOLNDHIEH: [u8; 0x48],                       // 0x290
    pub IAAJMHADJDG: FixPoint,                         // 0x2d8
    pub NCOHIAPKAED: FixPoint,                         // 0x2e0
    pub CGMHNNNOKAI: FixPoint,                         // 0x2e8
    pub ABIPIIBIIBE: FixPoint,                         // 0x2f0
    pub MNAPDDFFHJF: bool,                             // 0x2f8
    pub EKBHFCODKFO: bool,                             // 0x2f9
    pub IJJHMGEHMHB: bool,                             // 0x2fa
    pub KDCHAHHPPGD: bool,                             // 0x2fb
    pub GCGEEFLGCIG: i32,                              // 0x2fc
    pub ANHNDBECCJD: [u8; 0x40],                       // 0x300
    pub BDGDFKGOLPJ: FixPoint,                         // 0x340
    pub DBNKBGKCMKH: FixPoint,                         // 0x348
    pub FOLCDHNIMMI: FixPoint,                         // 0x350
    pub GLPLDJKMOBE: FixPoint,                         // 0x358
    pub NHHNLMOBEGH: FixPoint,                         // 0x360
    pub PJPKDAKBEJI: FixPoint,                         // 0x368
    pub EBDJHPNOALL: FixPoint,                         // 0x370
    pub DKOIGIHEBCD: FixPoint,                         // 0x378
    pub PNGJIDMHIOE: FixPoint,                         // 0x380
    pub DJCAFPFKOGP: FixPoint,                         // 0x388
    pub MGFECPHDPHB: FixPoint,                         // 0x390
    pub IICNDPJGCFA: i32,                              // 0x398
    pub GFFCEBJGABG: bool,                             // 0x39c
    pub HEMFDDDJOGK: bool,                             // 0x39d
    pub AHPFPMEGEKG: bool,                             // 0x39e
    pub KDJBABPDHEG: FixPoint,                         // 0x3a0
    pub EPJEDLOBFPG: FixPoint,                         // 0x3a8
    pub NAGMKEABGEE: FixPoint,                         // 0x3b0
    pub BEGDMOGLLGM: FixPoint,                         // 0x3b8
    pub AMAJNHHAJIM: FixPoint,                         // 0x3c0
    pub JFMADBFKBDK: FixPoint,                         // 0x3c8
    pub KOEGLFLGADD: FixPoint,                         // 0x3d0
    pub OJGNIBKADHK: u32,                              // 0x3d8
    pub JICCOEHBPJJ: bool,                             // 0x3dc
    pub EJJMIFKCFHP: bool,                             // 0x3dd
    pub FNBALMGFGDM: bool,                             // 0x3de
    pub CAANBNCPACE: bool,                             // 0x3df
    pub ENFFBMJBEDP: FixPoint,                         // 0x3e0
    pub JIINJMJGCOH: FixPoint,                         // 0x3e8
    pub NEPGNKOMAAA: FixPoint,                         // 0x3f0
    pub HHEIPBOKCOH: [u8; 0x40],                       // 0x3f8
    pub ELGMFJLGCPH: FixPoint,                         // 0x438
    pub MLKFKKACBCE: FixPoint,                         // 0x440
    pub HJAEPANAFLN: FixPoint,                         // 0x448
    pub HCGBHCPHDKJ: FixPoint,                         // 0x450
    pub GJNAGCJONAO: FixPoint,                         // 0x458
    pub EFFODBPOOCN: FixPoint,                         // 0x460
    pub ACDFHOGEMCC: [u8; 0x40],                       // 0x468
    pub PGGOANFBJON: FixPoint,                         // 0x4a8
    pub GMBACFCLEGD: FixPoint,                         // 0x4b0
    pub CFBOJBAJCEA: i32,                              // 0x4b8
    pub KBKGNDFAKGD: bool,                             // 0x4bc
    pub HKNLHAMMIIM: bool,                             // 0x4bd
    pub EGINKGPDNPK: bool,                             // 0x4be
    pub DPEJKHJPLAC: bool,                             // 0x4bf
    pub DJHDAOOEJOF: FixPoint,                         // 0x4c0
    pub KOCOLHHLFLD: [u8; 0x40],                       // 0x4c8
    pub MHEBPGAHFCB: FixPoint,                         // 0x508
    pub FGIPOLJPICM: FixPoint,                         // 0x510
    pub KODEDHBLGGH: FixPoint,                         // 0x518
    pub GHBPOPKEGLE: FixPoint,                         // 0x520
    pub FLMEBELNIKK: FixPoint,                         // 0x528
    pub PGOHAIPOCNK: FixPoint,                         // 0x530
    pub APDLLHIMMEM: FixPoint,                         // 0x538
    pub DEOICHHPAIF: FixPoint,                         // 0x540
    pub BDLFBDLDEND: [u8; 0x48],                       // 0x548
    pub PDCMJAMPJNL: FixPoint,                         // 0x590
    pub DPPDEDGCLJJ: FixPoint,                         // 0x598
    pub AHOCGHANMCE: FixPoint,                         // 0x5a0
    pub AHHEDGLMDMG: i32,                              // 0x5a8
    pub JGHJIGOCPNP: i32,                              // 0x5ac
    pub KMIKODLPNGL: i32,                              // 0x5b0
    pub NKIAEHFPJEA: i32,                              // 0x5b4
    pub JCPEINMPKAM: FixPoint,                         // 0x5b8
    pub JNFPCNAKNOP: FixPoint,                         // 0x5c0
    pub GIHPOCDLJOA: FixPoint,                         // 0x5c8
    pub HMMMDOHLFEP: FixPoint,                         // 0x5d0
    pub ILNAKPIOOAK: FixPoint,                         // 0x5d8
    pub DCEBGGFOFAO: FixPoint,                         // 0x5e0
    pub JEHMOKDJDDE: FixPoint,                         // 0x5e8
    pub JFKEEOMKMLI: FixPoint,                         // 0x5f0
    pub BKIFAEKCIHN: FixPoint,                         // 0x5f8
    pub GOHOJAIMDNM: FixPoint,                         // 0x600
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MMNDIEBMDNL {
    pub native_object: NativeObject,
    pub HMCDHMFHABF: OLHMAHMMBNN,                      // 0x10
    pub FIMNOPAAFEP: *const TurnBasedAbilityComponent, // 0x68
    pub HECCDOHIAFD: *const SkillCharacterComponent,   // 0x70
    pub GNBEIGMFGIP: *const c_void,                    // 0x78
    pub MKMMNLODHDD: *const c_void,                    // 0x80
    pub OOIFIGDBNBO: i32,                              // 0x88
    pub NMJEMHAMIHD: i32,                              // 0x8c
    pub DADCNHAIOMI: i32,                              // 0x90
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EDJEDBLFIKE {
    pub _parent_object: GameComponentBase,             // 0x0
    pub MNIJEBNCBKB: *const GameEntity,                // 0x18
    pub ECHFEHNDOJF: *const NativeArray<NativeObject>, // 0x20
    pub CJAJFECDPPJ: *const GameEntity,                // 0x28
    pub PEKEMAPHMPK: *const c_void,                    // 0x30
    pub DAGNFDKMDEF: *const NativeString,              // 0x38
    pub LOHMAMHFIGP: *const GameEntity,                // 0x40
    pub MJEBCDHCGMH: *const EDJEDBLFIKE,               // 0x48
    pub PAPNHNOOPMK: *const c_void,                    // 0x50
    pub BKOFKGCANPN: *const c_void,                    // 0x58
    pub MBDGPOJNGFP: *const c_void,                    // 0x60
    pub HLEGPKNHFFM: *const c_void,                    // 0x68
    pub LJHGLKMAPBE: *const TeamFormationComponent,    // 0x70
    pub PIJCPBGIEOC: *const NativeArray<NativeObject>, // 0x78
    pub PFPDENDBJKL: *const c_void,                    // 0x80
    pub LNILILLBCGC: *const NativeString,              // 0x88
    pub OGBHOPFFGFP: *const GameEntity,                // 0x90
    pub EADOCJKPPBN: *const c_void,                    // 0x98
    pub FLFNAFNNMBN: *const c_void,                    // 0xa0
    pub PGPNDEIFMMF: *const NativeList<EDJEDBLFIKE>,   // 0xa8
    pub GDHEHGIAEFE: bool,                             // 0xb0
    pub NJAKDBCILEM: bool,                             // 0xb1
    pub HFNDIAFABLD: bool,                             // 0xb2
    pub EAIAJIOPLDM: bool,                             // 0xb3
    pub KFKBEGPNBBA: u32,                              // 0xb4
    pub BGPEBFGCIFI: [u8; 0xc],                        // 0xb8
    pub FMKFHCDFJFD: i32,                              // 0xc4
    pub FMJMBKNPLPA: f32,                              // 0xc8
    pub JABPKHKIBHM: i32,                              // 0xcc
    pub ANGNPNFAMKA: [u8; 0xc],                        // 0xd0
    pub ODCPODIECFK: i32,                              // 0xdc
    pub IEFGFPBNLJE: i32,                              // 0xe0
    pub PLAODOKPNKA: i32,                              // 0xe4
    pub MNMBEKKJOPN: i32,                              // 0xe8
    pub BFKAEAEMEIF: f32,                              // 0xec
    pub DMGBPALONGL: f32,                              // 0xf0
    pub ALBCLOBMAAA: i32,                              // 0xf4
    pub DMGGHOFAADK: i32,                              // 0xf8
    pub FOMIFIJPOBA: f32,                              // 0xfc
    pub IOLCGFPHNBM: bool,                             // 0x100
    pub MJFIGBNFCMP: bool,                             // 0x101
    pub FCOAHCGBOPH: bool,                             // 0x102
    pub HBMKPLPDABN: bool,                             // 0x103
    pub FLMDDGMPOIN: [u8; 0xc],                        // 0x104
    pub OJABHABFLKK: TeamType,                         // 0x110
    pub CCKJAGJDNCK: bool,                             // 0x114
    pub IsInit__BackingField: bool,                    // 0x115
    pub LKBINIFLPIK: bool,                             // 0x116
    pub PMAHDELPGFH: bool,                             // 0x117
    pub DEFJLDFMHJA: [u8; 0xc],                        // 0x118
    pub HOMHCNECBND: [u8; 0xc],                        // 0x124
    pub JPACGCDPCEP: f32,                              // 0x130
    pub MLAHBDLNOAA: f32,                              // 0x134
    pub FDNNGFMGPBH: f32,                              // 0x138
    pub MOEFEJMBFHN: f32,                              // 0x13c
}
pub mod rpg {
    use crate::kreide::types::*;
    use std::ffi::c_void;
    pub mod gamecore {
        use crate::kreide::types::*;
        use std::ffi::c_void;
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct CharacterConfig {
            pub _parent_object: NativeObject,                          // 0x0
            pub SomatoType: i32,                                       // 0x10
            pub CharacterBodySize: i32,                                // 0x14
            pub CharacterHUDOffset: [u8; 0xc],                         // 0x18
            pub BuffPanelOffset: [u8; 0xc],                            // 0x24
            pub HitBoxOffset: [u8; 0xc],                               // 0x30
            pub TargetSelectGroup: i32,                                // 0x3c
            pub CameraConfigList: *const NativeArray<NativeObject>,    // 0x40
            pub HitBoxType: i32,                                       // 0x48
            pub HitBoxWidth: f32,                                      // 0x4c
            pub HitBoxLength: f32,                                     // 0x50
            pub HitBoxHeight: f32,                                     // 0x54
            pub HitBoxAttachPoint: *const NativeString,                // 0x58
            pub Resilience: *const c_void,                             // 0x60
            pub Location: *const c_void,                               // 0x68
            pub VisualRadius: f32,                                     // 0x70
            pub LookAtIKEnableRadius: f32,                             // 0x74
            pub AutoFlipModel: bool,                                   // 0x78
            pub SaveModelWhenDead: bool,                               // 0x79
            pub DeadPerform: bool,                                     // 0x7a
            pub PreloadUltraSkill: bool,                               // 0x7b
            pub IsSpecialVisualCharacter: i32,                         // 0x7c
            pub HideInTimeline: bool,                                  // 0x80
            pub AnimEventConfigList: *const NativeArray<NativeString>, // 0x88
            pub SkillList: *const NativeArray<NativeObject>,           // 0x90
            pub AbilityList: *const NativeArray<NativeString>,         // 0x98
            pub SkillAbilityList: *const NativeArray<NativeObject>,    // 0xa0
            pub DynamicValues: *const c_void,                          // 0xa8
            pub CustomValues: *const c_void,                           // 0xb0
            pub WeaponType: i32,                                       // 0xb8
            pub ArmorType: i32,                                        // 0xbc
            pub SkillReadyTransits: *const NativeArray<NativeObject>,  // 0xc0
            pub PhaseAnimConfig: *const c_void,                        // 0xc8
            pub AnimZoneConfigPath: *const NativeString,               // 0xd0
            pub InitAnimStateName: *const NativeString,                // 0xd8
            pub WhitelistSkillStateForInterrupt: *const NativeArray<NativeString>, // 0xe0
            pub ModifierPerformTimeFactor: f32,                        // 0xe8
            pub AsAidAttackTask: *const c_void,                        // 0xf0
            pub AsAidDefenderTask: *const c_void,                      // 0xf8
            pub AsAidProtectorTask: *const c_void,                     // 0x100
            pub DisableAnimEventLayers: *const NativeArray<NativeString>, // 0x108
            pub OnHitEditFootIKModeMap: *const c_void,                 // 0x110
            pub RepeatOccurAnimWhenBeHitNormalizedTime: f32,           // 0x118
            pub CameraNamedDynamicOffset: *const NativeString,         // 0x120
            pub IgnoreDynamicOffsetBySelf: bool,                       // 0x128
            pub OverrideHeightForCameraOffset: f32,                    // 0x12c
            pub MonsterIgnoreGlobalDymanicOffset: bool,                // 0x130
            pub MaxMonsterPhase: u32,                                  // 0x134
            pub PhaseList: *const NativeArray<NativeObject>,           // 0x138
            pub OverrideWaveMonsterPerform: *const NativeString,       // 0x140
            pub OverrideColliderCameraByName: *const NativeString,     // 0x148
            pub EntityColliderConfig: *const c_void,                   // 0x150
            pub EffectAdaptionList: *const NativeArray<NativeObject>,  // 0x158
            pub AttachPointEffectAdaptionList: *const NativeArray<NativeObject>, // 0x160
            pub FieldEffectAdaptionList: *const NativeArray<NativeObject>, // 0x168
            pub EffectAttachPointRedirect: *const c_void,              // 0x170
            pub MonsterConfig: *const c_void,                          // 0x178
            pub ResidentEffectKey: *const NativeString,                // 0x180
            pub ResidentPossessionKey: *const NativeString,            // 0x188
            pub EmotionCharacterID: *const NativeString,               // 0x190
            pub GraphEmotionAsset: *const NativeString,                // 0x198
            pub AITagList: *const c_void,                              // 0x1a0
            pub GlobalAIFactorGroups: *const c_void,                   // 0x1a8
            pub ReplaceEmoConfig: *const c_void,                       // 0x1b0
            pub WillUnstage: bool,                                     // 0x1b8
            pub ViewModeSortPriority: u32,                             // 0x1bc
            pub ReplaceAnimtorControllerPath: *const NativeString,     // 0x1c0
            pub AlwaysCutOnSkillTargetTeamChange: bool,                // 0x1c8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct CharacterDataComponent {
            pub _parent_object: GameComponentBase,                  // 0x0
            pub _OverrideCharacterConfigParam: [u8; 0x48],          // 0x18
            pub _DummpyEntityList: *const NativeList<NativeObject>, // 0x60
            pub _DynamicScaleAdaptEffectPathRule: *const c_void,    // 0x68
            pub JsonConfig__BackingField: *const CharacterConfig,   // 0x70
            pub _DynamicScaleAdaptConfigs: *const NativeArray<NativeObject>, // 0x78
            pub Summoner: *const GameEntity,                        // 0x80
            pub _DynamicScaleAdaptTypes: *const NativeArray<NativeObject>, // 0x88
            pub _RowData: *const c_void,                            // 0x90
            pub _CharacterUICustomValueDict: *const c_void,         // 0x98
            pub HideDisplayInfoSkillNames: *const c_void,           // 0xa0
            pub LineupIndex: i32,                                   // 0xa8
            pub CreateReason: i32,                                  // 0xac
            pub LastActTurnCount__BackingField: u32,                // 0xb0
            pub CharacterID__BackingField: u32,                     // 0xb4
            pub DisableHeadLookAtActionEntityOverride: [u8; 0x2],   // 0xb8
            pub IsVisibleInViewMode__BackingField: bool,            // 0xba
            pub SpawnTurnCount: u32,                                // 0xbc
            pub GridFightTag__BackingField: i32,                    // 0xc0
            pub EnhancedState: i32,                                 // 0xc4
            pub DisableRootYawMapping__BackingField: bool,          // 0xc8
            pub _SaveModelWhenDeadOverride: [u8; 0x2],              // 0xc9
            pub TriggerLimbo: bool,                                 // 0xcb
            pub IsBodyPart: bool,                                   // 0xcc
            pub LocalOffsetAsMoveTarget__BackingField: [u8; 0xc],   // 0xd0
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct EntityManager {
            pub native_object: NativeObject,
            pub LevelEntity__BackingField: *const GameEntity, // 0x10
            pub _EntityUniqueNameDict: *const NativeList<NativeObject>, // 0x18
            pub _PauseEntityTimeSlowIndexDic: *const NativeArray<NativeObject>, // 0x20
            pub DataViewUISelectFadeInFollowEntities__BackingField: *const c_void, // 0x28
            pub DataViewUILeaveSummonerOfUncreatedServant__BackingField: *const GameEntity, // 0x30
            pub DataViewUISelectFadeInEntity__BackingField: *const GameEntity, // 0x38
            pub _AllTeamEntity: *const NativeArray<GameEntity>, // 0x40
            pub DataViewUISelectSummonerOfUncreatedServant__BackingField: *const GameEntity, // 0x48
            pub PlayerGORoot__BackingField: *const c_void,    // 0x50
            pub _GroupEntityIDToEntityDict: *const c_void,    // 0x58
            pub _ProcessEntityTeamChangeDelg: *const c_void,  // 0x60
            pub _UniqueNamedEntityDictionary: *const c_void,  // 0x68
            pub EntityGORoot__BackingField: *const c_void,    // 0x70
            pub _AllEntityDictionary: *const c_void,          // 0x78
            pub DataViewUISelectFadeOutEntity__BackingField: *const GameEntity, // 0x80
            pub _ServerEntityIDToEntityDict: *const c_void,   // 0x88
            pub GroupGORoot__BackingField: *const c_void,     // 0x90
            pub _AllTeamEntityList: *const NativeList<GameEntity>, // 0x98
            pub LittleGameGORoot__BackingField: *const c_void, // 0xa0
            pub _OwnerWorldRef: *const GameWorld,             // 0xa8
            pub DataViewUISelectFadeOutSummonerEntity__BackingField: *const GameEntity, // 0xb0
            pub PerformanceGORoot__BackingField: *const c_void, // 0xb8
            pub _SnapshotEntityMap: *const c_void,            // 0xc0
            pub _UseUniqueSnapshot: bool,                     // 0xc8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct BattleLineupData {
            pub native_object: NativeObject,
            pub TeamBuffIDList: *const NativeArray<u32>, // 0x10
            pub ExtraTeam: *const NativeArray<LineUpCharacter>, // 0x18
            pub _TemplateVariables: *const c_void,       // 0x20
            pub LightTeam: *const NativeArray<LineUpCharacter>, // 0x28
            pub DeferCreateTrialPlayerDic: *const c_void, // 0x30
            pub Context: *const c_void,                  // 0x38
            pub SpecialAvatarLevelAreaConfigs: *const c_void, // 0x40
            pub BattleExtraPropertyAdditionList__BackingField: *const NativeList<NativeObject>, // 0x48
            pub _LevelPath: *const NativeString, // 0x50
            pub MazeBuffAdded: *const NativeList<NativeObject>, // 0x58
            pub AdditionalTemplateVariables: *const c_void, // 0x60
            pub WorldLevel: u32,                 // 0x68
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct SkillData {
            pub native_object: NativeObject,
            pub RowData: *const c_void,                   // 0x10
            pub SkillCom: *const SkillCharacterComponent, // 0x18
            pub OverrideAnimState: *const NativeString,   // 0x20
            pub UsableConditionDatas: *const NativeList<NativeObject>, // 0x28
            pub CustomReadyConfigConditions: *const NativeArray<NativeObject>, // 0x30
            pub AllChildSkillDatas: *const NativeList<SkillData>, // 0x38
            pub SkillTriggerKey: *const NativeString,     // 0x40
            pub ParentSkillData: *const SkillData,        // 0x48
            pub PreshowConditions: *const NativeArray<NativeObject>, // 0x50
            pub _SkillProperties: *const NativeArray<NativeObject>, // 0x58
            pub Config: *const c_void,                    // 0x60
            pub OverrideTargetInfo: *const c_void,        // 0x68
            pub OverrideCameraConfigAdded: *const c_void, // 0x70
            pub InsertCondTask: *const c_void,            // 0x78
            pub DefaultTargetInfo: *const c_void,         // 0x80
            pub VisibleCondTask: *const c_void,           // 0x88
            pub OverrideCameraConfig: *const c_void,      // 0x90
            pub _Slot: *const c_void,                     // 0x98
            pub SkillTypeOverride: *const c_void,         // 0xa0
            pub ChildIndex: i32,                          // 0xa8
            pub CurrentCoolDown: i32,                     // 0xac
            pub CommonActiveSkillID: u32,                 // 0xb0
            pub DefaultCoolDown: i32,                     // 0xb4
            pub AttackDamageTypePreshowAttach: i32,       // 0xb8
            pub MaxCastTimes: i32,                        // 0xbc
            pub SkillConfigID: u32,                       // 0xc0
            pub SkillIndex: i32,                          // 0xc4
            pub LeftCastTimes: i32,                       // 0xc8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct GameEntity {
            pub native_object: NativeObject,
            pub _OwnerWorldRef: *const GameWorld,        // 0x10
            pub _LateUpdateComponentList: *const c_void, // 0x18
            pub TimeScaleStack: *const c_void,           // 0x20
            pub _ComponentArray: *const NativeArray<GameComponentBase>, // 0x28
            pub TickLodTemplate: *const NativeString,    // 0x30
            pub _UnityGO: *const c_void,                 // 0x38
            pub _ComponentArrayRef: *const c_void,       // 0x40
            pub WorldTimeScaleAdpator: *const c_void,    // 0x48
            pub _TickComponentList: *const c_void,       // 0x50
            pub OnTeamChange: *const c_void,             // 0x58
            pub DisposeCallback: *const c_void,          // 0x60
            pub _CurTickListRef: [u8; 0x10],             // 0x68
            pub Name__BackingField: *const NativeString, // 0x78
            pub _ComponentList: *const c_void,           // 0x80
            pub HoyoTagContainer: *const c_void,         // 0x88
            pub _DestroyWaitList: *const c_void,         // 0x90
            pub TagComponentContainer: *const c_void,    // 0x98
            pub _UnstageReasonKey: *const NativeString,  // 0xa0
            pub _TickLodProxy: *const c_void,            // 0xa8
            pub OnStageStateChange: *const c_void,       // 0xb0
            pub NameForGameCore__BackingField: *const NativeString, // 0xb8
            pub _TickDelayFrameCount: u32,               // 0xc0
            pub LastTickTime__BackingField: f32,         // 0xc4
            pub TickLodBoundSize__BackingField: f32,     // 0xc8
            pub _Team: TeamType,                         // 0xcc
            pub _EntityType: EntityType,                 // 0xd0
            pub ObjectFeature__BackingField: i32,        // 0xd4
            pub ForceIgnoreTickLodBistSet: u32,          // 0xd8
            pub _ForceTickLodLowestReason: [u8; 0x8],    // 0xe0
            pub LastTickFrame__BackingField: u64,        // 0xe8
            pub IsLoaded__BackingField: bool,            // 0xf0
            pub _Tickable: bool,                         // 0xf1
            pub Visible__BackingField: bool,             // 0xf2
            pub IsStoryMode__BackingField: bool,         // 0xf3
            pub RuntimeID__BackingField: u32,            // 0xf4
            pub Disposing: bool,                         // 0xf8
            pub KillImmediately: bool,                   // 0xf9
            pub _IsRegisterEnviroChara: bool,            // 0xfa
            pub IsFakeAvatar__BackingField: bool,        // 0xfb
            pub _GroupID: u32,                           // 0xfc
            pub LastTickBucket__BackingField: i32,       // 0x100
            pub _AliveState: i32,                        // 0x104
            pub IsHero__BackingField: bool,              // 0x108
            pub _IsOnStage: bool,                        // 0x109
            pub HasDisposed: bool,                       // 0x10a
            pub _ShouldLateUpdate: bool,                 // 0x10b
            pub _ServerEntityID: u32,                    // 0x10c
            pub CampID__BackingField: i32,               // 0x110
            pub _GroupEntityID: u32,                     // 0x114
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct TeamFormationComponent {
            pub _parent_object: GameComponentBase, // 0x0
            pub _LastRefreshLocationTemplateConfigName: *const NativeString, // 0x18
            pub _DefaultLocationExtCfgs: *const NativeArray<NativeObject>, // 0x20
            pub _CurrentCustomFormationName: *const NativeString, // 0x28
            pub _Random: *const c_void,            // 0x30
            pub _AllTeammateCopy: *const NativeList<GameEntity>, // 0x38
            pub _TeamFormationDatas: *const NativeList<EDJEDBLFIKE>, // 0x40
            pub _FormationSnapshots: *const c_void, // 0x48
            pub WaveOfMonsterRuntimeID__BackingField: *const NativeList<u32>, // 0x50
            pub _RecordFormationIndices: *const c_void, // 0x58
            pub _CustomUpdateFormationDatas: *const NativeList<EDJEDBLFIKE>, // 0x60
            pub _FormationInCameraSpace: *const NativeList<GameEntity>, // 0x68
            pub _UseCustomFormationMembers: *const c_void, // 0x70
            pub _AllTeammate: *const NativeList<GameEntity>, // 0x78
            pub _CurrentLightTeamFightFormationConfig: *const c_void, // 0x80
            pub _multiRowTargetSelector: *const c_void, // 0x88
            pub _CurrentUseTemplateConfigName: *const NativeString, // 0x90
            pub _CurrentDarkTeamFightFormationConfig: *const c_void, // 0x98
            pub _GridFormationConfig: *const c_void, // 0xa0
            pub _TeamRefreshLocationContext: *const c_void, // 0xa8
            pub _FormationDatasNormal: *const NativeList<EDJEDBLFIKE>, // 0xb0
            pub _DefaultFormationCenter: *const EDJEDBLFIKE, // 0xb8
            pub _DefaultLocationExtCfgs2: *const NativeArray<NativeObject>, // 0xc0
            pub ComfortZoneWidthOverride__BackingField: f32, // 0xc8
            pub _gridUnitWidth: f32,               // 0xcc
            pub LocationRootWorldPos__BackingField: [u8; 0xc], // 0xd0
            pub _CurrentFormationType: i32,        // 0xdc
            pub _TeamZOffset: f32,                 // 0xe0
            pub _LastDieTeammatePos: [u8; 0x8],    // 0xe4
            pub _TeamActiveRowAliveStateFilter: [u8; 0x2], // 0xec
            pub _IsInited: bool,                   // 0xee
            pub _CustomFormationCenterDirty: bool, // 0xef
            pub _Team: TeamType,                   // 0xf0
            pub _FormationForceWidthMax: f32,      // 0xf4
            pub _RemoveVersionCounter: u32,        // 0xf8
            pub AliveEntitiesFormationCenterWorldPos__BackingField: [u8; 0xc], // 0xfc
            pub LockFormationRefresh__BackingField: bool, // 0x108
            pub _InverseFlag: bool,                // 0x109
            pub _HasCustomTeamFormation: bool,     // 0x10a
            pub _CustomFormationCenterFromBoundBox: [u8; 0xc], // 0x10c
            pub _TeamDistanceCache: f32,           // 0x118
            pub _dataViewRestoreData: [u8; 0xc],   // 0x11c
            pub _IsTeammateDirty: bool,            // 0x128
            pub _IsLocationDirty: bool,            // 0x129
            pub _AutoBalanceDefaultFormation: bool, // 0x12a
            pub _IsGridFightTeamFormation: bool,   // 0x12b
            pub _FormationForceWidthMin: f32,      // 0x12c
            pub TeamRotation__BackingField: [u8; 0x10], // 0x130
            pub TeamFaceDir__BackingField: f32,    // 0x140
            pub _MaxLocationCount: i32,            // 0x144
            pub FormationCenterWorldPos__BackingField: [u8; 0xc], // 0x148
            pub _ServantFormationState: i32,       // 0x154
            pub FormationCenterWorldPosFromBoundBox__BackingField: [u8; 0xc], // 0x158
            pub _CustomFormationCenter: [u8; 0xc], // 0x164
            pub FormationWidth__BackingField: f32, // 0x170
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct MonsterDataComponent {
            pub _parent_object: CharacterDataComponent,      // 0x0
            pub EnergyBarState: *const c_void,               // 0xe0
            pub _MonsterRowData: *const MonsterRowData,      // 0xe8
            pub _DefaultPhaseConfig: *const c_void,          // 0xf0
            pub _CustomDataRef: *const c_void,               // 0xf8
            pub _CreateParams: *const c_void,                // 0x100
            pub _MultiActionCounter: *const c_void,          // 0x108
            pub _DefaultMaxHP: FixPoint,                     // 0x110
            pub MonsterIndexInWave: i32,                     // 0x118
            pub MonsterWave: i32,                            // 0x11c
            pub _CurrentPhase: u32,                          // 0x120
            pub IsMuteLastKill__BackingField: bool,          // 0x124
            pub _CurrentPhaseHPRecovered: bool,              // 0x125
            pub OverrideRankScore__BackingField: [u8; 0x10], // 0x128
            pub _DefaultMaxStance: FixPoint,                 // 0x138
            pub _PhaseMaxHPStackIndex: i32,                  // 0x140
            pub _PhaseMaxStanceStackIndex: i32,              // 0x144
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct LineUpCharacter {
            pub native_object: NativeObject,
            pub SpiritPassiveList: *const NativeArray<u32>, // 0x10
            pub BattleRelicItemModule: *const c_void,       // 0x18
            pub BattleGridAvatarData: *const c_void,        // 0x20
            pub SkillTreePointList: *const NativeArray<NativeObject>, // 0x28
            pub BattleEquipmentList: *const NativeArray<NativeObject>, // 0x30
            pub ChangedSkillTreePointList: *const NativeArray<NativeObject>, // 0x38
            pub CharacterID: u32,                           // 0x40
            pub WorldLevel: u32,                            // 0x44
            pub SpecialAvatarID: u32,                       // 0x48
            pub CharacterAvatarType: i32,                   // 0x4c
            pub CharacterRowIndex: u32,                     // 0x50
            pub CharacterHPRatio: FixPoint,                 // 0x58
            pub CharacterRank: u32,                         // 0x60
            pub AssistUid: u32,                             // 0x64
            pub SpiritLineupType: i32,                      // 0x68
            pub TotalPower: u32,                            // 0x6c
            pub CharacterSP_Denominator: FixPoint,          // 0x70
            pub EnhancedID: u32,                            // 0x78
            pub Index: u32,                                 // 0x7c
            pub CharacterSP_Numerator: FixPoint,            // 0x80
            pub CharacterPromotion: u32,                    // 0x88
            pub CharacterLevel: u32,                        // 0x8c
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct MonsterRowData {
            pub native_object: NativeObject,
            pub _modelPath: *const NativeString, // 0x10
            pub _OverrideStanceWeakList: *const NativeArray<NativeObject>, // 0x18
            pub _HardLevelRow: *const c_void,    // 0x20
            pub _SkillRowDatas: *const NativeList<NativeObject>, // 0x28
            pub _Row: *const c_void,             // 0x30
            pub _OverrideStrategies: *const NativeArray<TextID>, // 0x38
            pub _OverrideDisplay: [u8; 0x50],    // 0x40
            pub _EliteGroup2Row: *const c_void,  // 0x90
            pub _BaseMonsterRow: *const c_void,  // 0x98
            pub _EliteGroupRow: *const c_void,   // 0xa0
            pub _Json: *const CharacterConfig,   // 0xa8
            pub _TemplateRow: *const c_void,     // 0xb0
            pub _InitStance: [u8; 0x10],         // 0xb8
            pub _MaxHP: FixPoint,                // 0xc8
            pub _InitHP: FixPoint,               // 0xd0
            pub _MaxStance: [u8; 0x8],           // 0xd8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct AbilityConfig {
            pub _parent_object: NativeObject,                       // 0x0
            pub Name: *const NativeString,                          // 0x10
            pub TargetInfo: *const c_void,                          // 0x18
            pub OnAdd: *const NativeArray<NativeObject>,            // 0x20
            pub OnRemove: *const NativeArray<NativeObject>,         // 0x28
            pub OnStart: *const NativeArray<NativeObject>,          // 0x30
            pub DynamicValues: *const c_void,                       // 0x38
            pub TaskListTemplate: *const NativeArray<NativeObject>, // 0x40
            pub _TaskListTemplatesMap: *const c_void,               // 0x48
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct BattleEventSkillRowData {
            pub native_object: NativeObject,
            pub _OverrideData: [u8; 0xf0],        // 0x10
            pub _Config: *const c_void,           // 0x100
            pub _Row: *const c_void,              // 0x108
            pub _DefaultOverrideData: [u8; 0xe8], // 0x110
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct GameComponentBase {
            pub native_object: NativeObject,
            pub _OwnerRef: *const GameEntity, // 0x10
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct BattleInstance {
            pub native_object: NativeObject,
            pub _BattleLineupData: *const BattleLineupData, // 0x10
            pub _GameWorld: *const GameWorld,               // 0x18
            pub BattleLogicRandom__BackingField: *const c_void, // 0x20
            pub LogExport__BackingField: *const c_void,     // 0x28
            pub _TurnBasedGameMode: *const TurnBasedGameMode, // 0x30
            pub _BattleLineupDataPrimitive: *const BattleLineupData, // 0x38
            pub BattleUnifiedAreaID__BackingField: u32,     // 0x40
            pub SnapshotHashTurnCount: i32,                 // 0x44
            pub ClientIFixVersion: u32,                     // 0x48
            pub BattleAreaID__BackingField: u32,            // 0x4c
            pub LogicRandomSeed__BackingField: i32,         // 0x50
            pub AutoBattleAtStart: bool,                    // 0x54
            pub IsBattleDirectKill__BackingField: bool,     // 0x55
            pub EncryptTime__BackingField: bool,            // 0x56
            pub IsBattleServerSimulator__BackingField: bool, // 0x57
            pub IsTeamFormationExpansion__BackingField: bool, // 0x58
            pub ComplexSkillAIConsiderUltra: bool,          // 0x59
            pub DimensionID__BackingField: u32,             // 0x5c
            pub FloorID__BackingField: u32,                 // 0x60
            pub BattleCheckResult: i32,                     // 0x64
            pub BattleAreaGroupID__BackingField: u32,       // 0x68
            pub PlaneID__BackingField: u32,                 // 0x6c
            pub BattleID: u32,                              // 0x70
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct ServantSkillRowData {
            pub native_object: NativeObject,
            pub _Row: *const c_void,              // 0x10
            pub _DefaultOverrideData: [u8; 0xf0], // 0x18
            pub _Config: *const c_void,           // 0x108
            pub _OverrideData: [u8; 0xe8],        // 0x110
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct AvatarSkillRowData {
            pub native_object: NativeObject,
            pub _OverrideData: [u8; 0xf0],        // 0x10
            pub _Row: *const c_void,              // 0x100
            pub _DefaultOverrideData: [u8; 0xf0], // 0x108
            pub _Config: *const c_void,           // 0x1f8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct TurnBasedAbilityComponent {
            pub _parent_object: GameComponentBase, // 0x0
            pub _BuffLockStepSources: *const NativeList<NativeObject>, // 0x18
            pub _OnHitEffectMultipleOverride: *const NativeList<NativeObject>, // 0x20
            pub _ModifierEventProcessors: *const NativeArray<NativeObject>, // 0x28
            pub CustomDataRef__BackingField: *const c_void, // 0x30
            pub ResistModifierBehaviorFlags__BackingField: *const NativeList<NativeObject>, // 0x38
            pub DamageSplitData: *const NativeList<NativeObject>, // 0x40
            pub _TransformRef: *const c_void,      // 0x48
            pub Weakness: *const c_void,           // 0x50
            pub _StatusProbabilityDict: *const c_void, // 0x58
            pub _KillerEntity: *const GameEntity,  // 0x60
            pub RegardAsAttackTypeMap: *const NativeList<NativeObject>, // 0x68
            pub LastStanceBreakEntity__BackingField: *const GameEntity, // 0x70
            pub _AbilityPropertiesInitSnapshot: *const NativeArray<FixPoint>, // 0x78
            pub DamageDefender: *const GameEntity, // 0x80
            pub _SyncPropertyMask: *const c_void,  // 0x88
            pub _DefaultStanceInfo: *const c_void, // 0x90
            pub _CharacterDataRef: *const CharacterDataComponent, // 0x98
            pub _OnHitEffectOverride: *const NativeList<NativeObject>, // 0xa0
            pub DisableActionStateByTask__BackingField: *const c_void, // 0xa8
            pub _DotModifierEventProcessors: *const NativeList<NativeObject>, // 0xb0
            pub CharmSkillName: *const NativeString, // 0xb8
            pub _SelfExtrAbilityList: *const NativeList<NativeString>, // 0xc0
            pub _LockHPList: *const NativeList<NativeObject>, // 0xc8
            pub _DmgChunk: *const c_void,          // 0xd0
            pub _DepartedParams: *const NativeList<NativeObject>, // 0xd8
            pub _JsonConfigRef: *const CharacterConfig, // 0xe0
            pub _CharacterChangeSource: *const GameEntity, // 0xe8
            pub ProjectileTargetAttachPoint: *const NativeString, // 0xf0
            pub _MuteBehaviorFlags: *const c_void, // 0xf8
            pub _DamagedEntityListInAttack: *const NativeList<GameEntity>, // 0x100
            pub _DamageStoreList: *const NativeList<NativeObject>, // 0x108
            pub _DamageAttacker: *const GameEntity, // 0x110
            pub _AttackDamageLog: *const NativeList<NativeObject>, // 0x118
            pub OnAbilityPropertyChanged: *const NativeList<NativeObject>, // 0x120
            pub _CharacterChangeParam: *const c_void, // 0x128
            pub _ModifierDelayParamList: *const c_void, // 0x130
            pub _DebuffLockStepSources: *const NativeList<NativeObject>, // 0x138
            pub _AbilityProperties: *const NativeArray<NativeObject>, // 0x140
            pub RegardAsSkillTypeMap: *const NativeList<NativeObject>, // 0x148
            pub AddModifierBindValueMapping: *const c_void, // 0x150
            pub OverflowStanceDamageAttacker__BackingField: *const GameEntity, // 0x158
            pub KillerSkill__BackingField: *const c_void, // 0x160
            pub _ModifierRecordList: *const c_void, // 0x168
            pub CharmDamageAttackProperty: *const c_void, // 0x170
            pub _DamagedAllEntityIDListInAttack: *const c_void, // 0x178
            pub _DelayModifyActionDelayQueue: *const c_void, // 0x180
            pub _ExtraMaxLayerConfig: *const NativeList<NativeObject>, // 0x188
            pub ModifierOverrideMapping: *const c_void, // 0x190
            pub _EnergyPointEntries: *const NativeList<NativeObject>, // 0x198
            pub _RedStanceInfoList: *const NativeList<NativeObject>, // 0x1a0
            pub _ExtraStanceInfo: *const c_void,   // 0x1a8
            pub CharmDamageTarget: *const GameEntity, // 0x1b0
            pub _LockShieldCounter: *const c_void, // 0x1b8
            pub _AbilityToSkillMapping: *const c_void, // 0x1c0
            pub LockActionDelayChange: *const c_void, // 0x1c8
            pub _StancePreshowConfigs: *const NativeList<NativeObject>, // 0x1d0
            pub _ModifierEventSourceMuteCounter: *const c_void, // 0x1d8
            pub AdditionalAbilityParamList: *const NativeArray<NativeObject>, // 0x1e0
            pub _EnableNegativeHPSourceList: *const NativeList<NativeObject>, // 0x1e8
            pub _RedStanceInfo: *const c_void,     // 0x1f0
            pub _SyncPropertySource: *const TurnBasedAbilityComponent, // 0x1f8
            pub AbilityComponentRef__BackingField: *const c_void, // 0x200
            pub _StatusChanceResistanceDict: *const c_void, // 0x208
            pub VisualFlagValue__BackingField: i32, // 0x210
            pub SpeedVisualFlagValue__BackingField: i32, // 0x214
            pub _ModifierDelayAddCount: i32,       // 0x218
            pub _DebuffLockStep: i32,              // 0x21c
            pub _BreakExtendEventUnsettled: bool,  // 0x220
            pub IsForceActionable__BackingField: bool, // 0x221
            pub LastBreakStanceDamageType__BackingField: i32, // 0x224
            pub _ModifierUIOperationIncr: i32,     // 0x228
            pub _HighestPriorityOnHitEffect: i32,  // 0x22c
            pub DeathSource__BackingField: i32,    // 0x230
            pub _IsProcessingModifierDelayParam: bool, // 0x234
            pub MuteAllTriggerDeath__BackingField: bool, // 0x235
            pub IsInAttack: bool,                  // 0x236
            pub CharmDisableSPAdd: bool,           // 0x237
            pub InsertAbilityCount: i32,           // 0x238
            pub _CurrentAttackPhase: i32,          // 0x23c
            pub LastStanceDamageType__BackingField: i32, // 0x240
            pub ForbidVisualFlagValue__BackingField: i32, // 0x244
            pub IsTriggeringStanceCountDown__BackingField: bool, // 0x248
            pub HasRevived: bool,                  // 0x249
            pub MuteTriggerDeath__BackingField: bool, // 0x24a
            pub ActionDelayChanged__BackingField: [u8; 0x5], // 0x24b
            pub StanceState__BackingField: i32,    // 0x250
            pub BattleTag__BackingField: i32,      // 0x254
            pub bIsInCharmAction: bool,            // 0x258
            pub IsSharedDamageDataTarget: bool,    // 0x259
            pub LockSelfActionDelay: bool,         // 0x25a
            pub CharmDisableBPAdd: bool,           // 0x25b
            pub _ResetStanceVersion: u32,          // 0x25c
            pub ProjectileHitCount: i32,           // 0x260
            pub OverflowStanceDamage__BackingField: FixPoint, // 0x268
            pub UseSpecialSP__BackingField: bool,  // 0x270
            pub _IsBehaviorFlagVisualDirty: bool,  // 0x271
            pub IsSnapshot__BackingField: bool,    // 0x272
            pub BlockModifySp__BackingField: bool, // 0x273
            pub _BuffLockStep: i32,                // 0x274
            pub InheritSPRatio: FixPoint,          // 0x278
            pub StanceType: i32,                   // 0x280
            pub _DeathVersion: u32,                // 0x284
            pub CharmDamageCount: i32,             // 0x288
            pub CurrentAttackType__BackingField: AttackType, // 0x28c
            pub PropertyEnumBoundary__BackingField: AbilityProperty, // 0x290
            pub IsTriggeredBlockDamage: bool,      // 0x294
            pub ForceKillFlag__BackingField: bool, // 0x295
            pub PropertyChangeFlag__BackingField: bool, // 0x296
            pub TriggerBreakExtendLogic: bool,     // 0x297
            pub ActionDelayDistance__BackingField: FixPoint, // 0x298
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum AttackType {
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
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct FixPoint {
            pub m_rawValue: i64, // 0x0
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct BattleEventDataComponent {
            pub _parent_object: CharacterDataComponent,          // 0x0
            pub SourceCaster__BackingField: *const GameEntity,   // 0xe0
            pub BattleEventConfig__BackingField: *const c_void,  // 0xe8
            pub _EnergyBarState: *const c_void,                  // 0xf0
            pub _BattleEventRowData: *const c_void,              // 0xf8
            pub _TBAbilityRef: *const TurnBasedAbilityComponent, // 0x100
            pub CreateParams__BackingField: *const c_void,       // 0x108
            pub BattleEventTotalDamageType: TeamType,            // 0x110
            pub WarningChallengeTurnLeft: u32,                   // 0x114
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum TeamType {
            TeamUnknow = 0,
            TeamLight = 1,
            TeamDark = 2,
            TeamNeutral = 3,
            TeamNPC = 4,
            Count = 5,
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct TurnBasedGameMode {
            pub native_object: NativeObject,
            pub _TurnStateFSM: *const c_void,     // 0x10
            pub _RelationGroupMgr: *const c_void, // 0x18
            pub AssistantAvatarEntity__BackingField: *const GameEntity, // 0x20
            pub _CurModifierPerformSeq: *const c_void, // 0x28
            pub BattleCounter: *const c_void,     // 0x30
            pub _LimboEntities: *const NativeList<NativeObject>, // 0x38
            pub TurnActionDelayCostChangeSource__BackingField: *const GameEntity, // 0x40
            pub MonsterWaveTextInfo: *const c_void, // 0x48
            pub BattleChangeAvatarManager__BackingField: *const c_void, // 0x50
            pub _ReplayData: *const c_void,       // 0x58
            pub _ModifierPerformCamerContext: *const c_void, // 0x60
            pub _SkillAddBuffPerformList: *const NativeList<NativeObject>, // 0x68
            pub _ImmediateActionEntities: *const c_void, // 0x70
            pub ActionBarMgr__BackingField: *const c_void, // 0x78
            pub _ActionDelayRedirectManager: *const c_void, // 0x80
            pub LastSummonMonsterList: *const NativeList<GameEntity>, // 0x88
            pub _LevelLockedFeatureSet: *const c_void, // 0x90
            pub _InsertUltraSkillParamsQueue: *const NativeList<NativeObject>, // 0x98
            pub _LimboEntitiesSkipSettlement: *const NativeList<NativeObject>, // 0xa0
            pub StageBattleEventMgr__BackingField: *const c_void, // 0xa8
            pub _ActionEntityList: *const NativeList<GameEntity>, // 0xb0
            pub CurrentWaveMainMonsterIDPool__BackingField: *const NativeArray<u32>, // 0xb8
            pub CurrentMVPEntity__BackingField: *const GameEntity, // 0xc0
            pub CurrentTurnOwnerEntity__BackingField: *const GameEntity, // 0xc8
            pub LastKillTargetList__BackingField: *const NativeList<GameEntity>, // 0xd0
            pub _EvolveBuildGearMgr: *const c_void, // 0xd8
            pub _CurrentSkillCharacter: *const SkillCharacterComponent, // 0xe0
            pub _EntityModifierPerforms: *const c_void, // 0xe8
            pub _UnselectableParams: *const NativeList<NativeObject>, // 0xf0
            pub BattleAIPublicKnowledge__BackingField: *const c_void, // 0xf8
            pub _MainMonster: *const GameEntity,  // 0x100
            pub _AllTeamCharacters: *const NativeList<GameEntity>, // 0x108
            pub _CachedDynamicSkillTargetSelection: *const GameEntity, // 0x110
            pub _EntityCustomUnselectableDatas: *const NativeList<NativeObject>, // 0x118
            pub _CommonSkillPoolNames: *const c_void, // 0x120
            pub PrepareAbility__BackingField: *const c_void, // 0x128
            pub _CurrentActionDelayModifyGroup: *const NativeList<GameEntity>, // 0x130
            pub _SwordTrainingMgr: *const c_void, // 0x138
            pub _WaitingAbilityList: *const NativeList<NativeObject>, // 0x140
            pub _PhaseModifierList: *const NativeList<NativeObject>, // 0x148
            pub BattleEventInitedData__BackingField: *const c_void, // 0x150
            pub _LimboEntitiesWaitAbilityFinish: *const NativeList<NativeObject>, // 0x158
            pub _LastBreakMonster: *const GameEntity, // 0x160
            pub _allowQuitStates: *const NativeList<NativeObject>, // 0x168
            pub ThisTurnAnimEvents: *const c_void, // 0x170
            pub LastZombie__BackingField: *const GameEntity, // 0x178
            pub _ActionDelayLinkMgr: *const c_void, // 0x180
            pub _performParam: *const c_void,     // 0x188
            pub _RogueInBattleData: *const c_void, // 0x190
            pub _LimboRevivableEntities: *const NativeList<NativeObject>, // 0x198
            pub _AttackingEntityList: *const c_void, // 0x1a0
            pub _FateBattleManager: *const c_void, // 0x1a8
            pub _LinkTeammateList: *const NativeList<GameEntity>, // 0x1b0
            pub _ActionEntityListSnapshot: *const NativeList<GameEntity>, // 0x1b8
            pub _EventProcessor: *const c_void,   // 0x1c0
            pub _ActionDelayChangeStamp: [u8; 0x18], // 0x1c8
            pub LastKillSkill__BackingField: *const c_void, // 0x1e0
            pub _OverrieWaveMonsterPerformDatas: *const NativeList<NativeObject>, // 0x1e8
            pub LastKillCaster__BackingField: *const GameEntity, // 0x1f0
            pub _ActionDelayOrderTrigger: *const c_void, // 0x1f8
            pub DamageQueue__BackingField: *const c_void, // 0x200
            pub PerformDelayExecuteList: *const NativeList<NativeObject>, // 0x208
            pub _VersusBarMgr: *const c_void,     // 0x210
            pub TimeGameStart: *const c_void,     // 0x218
            pub _AidDetail: *const c_void,        // 0x220
            pub _CurrentTurnTargetEntity: *const GameEntity, // 0x228
            pub _SomatoModifierPerforms: *const NativeList<NativeObject>, // 0x230
            pub _AvatarChangeParam: *const c_void, // 0x238
            pub _CurrentTurnActionEntity: *const GameEntity, // 0x240
            pub _InsertAbilityList: *const NativeList<MMNDIEBMDNL>, // 0x248
            pub LastTurnSnapshot: *const c_void,  // 0x250
            pub GridFightMananger__BackingField: *const c_void, // 0x258
            pub SkillUsageLog__BackingField: *const c_void, // 0x260
            pub OwnerBattleInstanceRef__BackingField: *const BattleInstance, // 0x268
            pub _AllOffTeamCharacters: *const NativeList<GameEntity>, // 0x270
            pub LocalWinFlag__BackingField: [u8; 0x2], // 0x278
            pub _IsUseLinkSkill: bool,            // 0x27a
            pub IsActionOrder1UsedTBSkill__BackingField: bool, // 0x27b
            pub _RequireMakeLimboEntitiesDie: bool, // 0x27c
            pub _RecordOperationByLG: [u8; 0x8],  // 0x280
            pub CurrentWaveIndexInStage__BackingField: u32, // 0x288
            pub _DarkTeamTurnCount: u32,          // 0x28c
            pub _ModifierEndingPerformRemainedTime: f32, // 0x290
            pub ShowCutinUIState__BackingField: i32, // 0x294
            pub ChallengeTurnLimit__BackingField: u32, // 0x298
            pub PropagateBeingAttackTeam__BackingField: TeamType, // 0x29c
            pub _HitPerformMinTimer: f32,         // 0x2a0
            pub _CurrentTurnTeam: TeamType,       // 0x2a4
            pub StanceCountDownSPChangeValue__BackingField: f32, // 0x2a8
            pub _HoldFrameForCapture: u32,        // 0x2ac
            pub ThisTurnAnimEventCount: i32,      // 0x2b0
            pub BattleResultState__BackingField: [u8; 0x8], // 0x2b4
            pub _TurnCounter: u32,                // 0x2bc
            pub _PrevTickModeState: i32,          // 0x2c0
            pub WaveMonsterMaxCount__BackingField: i32, // 0x2c4
            pub TurnEndKeep: bool,                // 0x2c8
            pub ClearUltraSkillQueue__BackingField: bool, // 0x2c9
            pub _ActionEntityListInited: bool,    // 0x2ca
            pub ApplyUIOperateOnDisableActionFlagChange: bool, // 0x2cb
            pub _DamageCounter: u32,              // 0x2cc
            pub CurrentModeState__BackingField: i32, // 0x2d0
            pub _NextModifierIndex: i32,          // 0x2d4
            pub _IsModifierPerformCameraSet: bool, // 0x2d8
            pub IsManualExitBattle: bool,         // 0x2d9
            pub AddOpCountOnInsertUltraWaitOrder: bool, // 0x2da
            pub SkipTurnOwnerActionFlag__BackingField: bool, // 0x2db
            pub _GamePauseFlag: bool,             // 0x2dc
            pub ApplyUIOperateOnSkillDisableChange: bool, // 0x2dd
            pub PendingMonsterToWave__BackingField: bool, // 0x2de
            pub _CurrentTurnActionEntitySkipActionFlag: bool, // 0x2df
            pub _CachedDynamicSkillInput: i32,    // 0x2e0
            pub _WaveMonsterCurrentCount: i32,    // 0x2e4
            pub _HoldFrameForCaptureFlag: bool,   // 0x2e8
            pub LockTeamZOffset__BackingField: bool, // 0x2e9
            pub IsUseSkillOneMore: bool,          // 0x2ea
            pub CurrentInsertSkillSkipActionFlag: bool, // 0x2eb
            pub PauseState__BackingField: i32,    // 0x2ec
            pub UseSkillOneMoreDefaultSkill: i32, // 0x2f0
            pub _NextAbilityIndex: i32,           // 0x2f4
            pub BattleResultAsWin: bool,          // 0x2f8
            pub _LastReplayAutoBattle: bool,      // 0x2f9
            pub WinFlag: bool,                    // 0x2fa
            pub ClearUltraSkillEffect: bool,      // 0x2fb
            pub _LightTeamTurnCount: u32,         // 0x2fc
            pub _ModifierPerformTimeScale: f32,   // 0x300
            pub SkipCameraDitherByLastKill: bool, // 0x304
            pub CertainlyLoseInAdvance__BackingField: bool, // 0x305
            pub AutoInsertUltraSkill: bool,       // 0x306
            pub _AutoBattle: bool,                // 0x307
            pub _OverrideAILocked: bool,          // 0x308
            pub PrepareAbilityFinish__BackingField: bool, // 0x309
            pub SkipDeathHandle__BackingField: bool, // 0x30a
            pub TurnOwnerActionPhaseEnd__BackingField: bool, // 0x30b
            pub RealTimeCounter__BackingField: f32, // 0x30c
            pub _ModifierPerformTimeTotal: f32,   // 0x310
            pub ChallengeTurnLimitType__BackingField: i32, // 0x314
            pub ForbidAI: bool,                   // 0x318
            pub IsTeamFormationExpansion__BackingField: bool, // 0x319
            pub _IsReplayBeingSaved: bool,        // 0x31a
            pub CertainlyWinInAdvance__BackingField: bool, // 0x31b
            pub LastKillFinish__BackingField: bool, // 0x31c
            pub IsManualRetryExitBattle: bool,    // 0x31d
            pub MuteLastKillTriggered: bool,      // 0x31e
            pub _IsCreatingNewWave: bool,         // 0x31f
            pub _RecordParamByLG: u32,            // 0x320
            pub TurnActionDelayCostRatio__BackingField: FixPoint, // 0x328
            pub _ModifierPerformTimerTotal: f32,  // 0x330
            pub _ChallengeTurnAcc: u32,           // 0x334
            pub _DeathVersion: u32,               // 0x338
            pub _OperationCounter: u32,           // 0x33c
            pub ElapsedActionDelay__BackingField: FixPoint, // 0x340
            pub _SkillExecutionEventState: i32,   // 0x348
            pub BattleFinishReason: i32,          // 0x34c
            pub CurrentWaveStageID__BackingField: u32, // 0x350
            pub TurnOwnerPrepareAbilityUsed__BackingField: bool, // 0x354
            pub IsLastKillTriggered: bool,        // 0x355
            pub CurrentTurnActionEntityContinuousActionCount__BackingField: u32, // 0x358
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum EntityType {
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
        #[repr(C)]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum AbilityProperty {
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
            ExtraFrontPower = 183,
            ExtraFrontPowerBase = 184,
            ExtraFrontPowerAddedRatio1 = 185,
            ExtraFrontPowerAddedRatio2 = 186,
            ExtraBackPower = 187,
            ExtraBackPowerBase = 188,
            ExtraBackPowerAddedRatio1 = 189,
            ExtraBackPowerAddedRatio2 = 190,
            ExtraUltraDamageAddedRatio1 = 191,
            ExtraSkillDamageAddedRatio1 = 192,
            ExtraNormalDamageAddedRatio1 = 193,
            ExtraInsertDamageAddedRatio1 = 194,
            ExtraDOTDamageAddedRatio1 = 195,
            Count = 196,
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct GameWorld {
            pub native_object: NativeObject,
            pub _PrefabGameObjectMap: *const c_void, // 0x10
            pub _TickedEntityListPerFrame: *const NativeList<GameEntity>, // 0x18
            pub _Modules: *const NativeList<NativeObject>, // 0x20
            pub DamageDataStack__BackingField: *const c_void, // 0x28
            pub EntityBeforeDyingCallback: *const c_void, // 0x30
            pub _EntityTickList: *const c_void,      // 0x38
            pub _EntityManager: *const EntityManager, // 0x40
            pub _EnterDyingEntityList: *const NativeList<GameEntity>, // 0x48
            pub EntityDiedCallback: *const c_void,   // 0x50
            pub _GlobalTimeScaleDatas: *const NativeList<NativeObject>, // 0x58
            pub NewEntityCallback: *const c_void,    // 0x60
            pub BattleModeRef__BackingField: *const c_void, // 0x68
            pub _NeedTickModules: *const NativeList<NativeObject>, // 0x70
            pub _EntityList: *const NativeList<GameEntity>, // 0x78
            pub _NeedLateUpdateModules: *const NativeList<NativeObject>, // 0x80
            pub _EventManager: *const c_void,        // 0x88
            pub LogicRandom: *const c_void,          // 0x90
            pub _DeferDeleteEntityList: *const NativeList<GameEntity>, // 0x98
            pub _DyingEntityList: *const NativeList<NativeObject>, // 0xa0
            pub TimeScaleStack__BackingField: *const c_void, // 0xa8
            pub ParamRegister__BackingField: *const c_void, // 0xb0
            pub EntityReviveCallback: *const c_void, // 0xb8
            pub MonoEffectManagerRef: *const c_void, // 0xc0
            pub BattleInstanceRef__BackingField: *const BattleInstance, // 0xc8
            pub _LevelAreaManager: *const c_void,    // 0xd0
            pub _GPTimelineHierarchyManager: *const c_void, // 0xd8
            pub EntityWillDestroyCallback: *const c_void, // 0xe0
            pub _BalanceTickDurationBucket: i32,     // 0xe8
            pub _IDFactory_Battle: u32,              // 0xec
            pub _IDFactory: u32,                     // 0xf0
            pub UnscaledDeltaTime__BackingField: f32, // 0xf4
            pub _IDFactory_ClientOnly: u32,          // 0xf8
            pub _BalanceTickDurationTime: f32,       // 0xfc
            pub _NextTickDeferDeleteCount: i32,      // 0x100
            pub IsBattleGameWorld__BackingField: bool, // 0x104
            pub _IsDisposing: bool,                  // 0x105
            pub _IsInTick: bool,                     // 0x106
            pub _IsPause: bool,                      // 0x107
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct SkillCharacterComponent {
            pub _parent_object: GameComponentBase,                    // 0x0
            pub SkillActualAttacker__BackingField: *const GameEntity, // 0x18
            pub CurrentSkillTargetList__BackingField: *const NativeList<GameEntity>, // 0x20
            pub _SkillDataList: *const NativeList<SkillData>,         // 0x28
            pub SkillPointEntity__BackingField: *const GameEntity,    // 0x30
            pub _SkillTypeDisableSlots: *const c_void,                // 0x38
            pub TaskContext__BackingField: *const c_void,             // 0x40
            pub AutoUseUltraParams: *const c_void,                    // 0x48
            pub CurrentAimAtMainTargetList: *const NativeList<GameEntity>, // 0x50
            pub _recordAbilityInfo: [u8; 0x30],                       // 0x58
            pub CurrentSkillTargetCharacterId: *const c_void,         // 0x88
            pub _JsonConfigRef: *const CharacterConfig,               // 0x90
            pub CurrentSkillSubTargetList__BackingField: *const NativeList<GameEntity>, // 0x98
            pub CurrentSkillTargetDamageHP: *const c_void,            // 0xa0
            pub _SkillSlots: *const NativeArray<NativeObject>,        // 0xa8
            pub CurrentAimAtTargetList: *const NativeList<GameEntity>, // 0xb0
            pub _CharacterDataRef: *const CharacterDataComponent,     // 0xb8
            pub _TBAbilityRef: *const TurnBasedAbilityComponent,      // 0xc0
            pub _SkillTypeDisableCountArr: *const NativeArray<i32>,   // 0xc8
            pub _SkillTypeOverrideProperty: *const NativeArray<NativeObject>, // 0xd0
            pub CurrentAimAtSubTargetList: *const NativeList<GameEntity>, // 0xd8
            pub OnSkillSetup: *const NativeList<NativeObject>,        // 0xe0
            pub _SkillTargetRedirectEntries: *const NativeList<NativeObject>, // 0xe8
            pub IsNoSpCost__BackingField: bool,                       // 0xf0
            pub PassiveUsed__BackingField: bool,                      // 0xf1
            pub IsNoBpCost__BackingField: bool,                       // 0xf2
            pub CurrentSkillKillAllOrBoss: bool,                      // 0xf3
            pub _actionSkillIndex: i32,                               // 0xf4
            pub _OpIndexInSkill: i32,                                 // 0xf8
            pub _RecordSkillExtraUseParam: i32,                       // 0xfc
            pub SelfWaitActiveSkillIndex: i32,                        // 0x100
            pub CurrentSkillKilledCount: i32,                         // 0x104
            pub _CurrentSkillExtraUseParam: i32,                      // 0x108
            pub _CurrentSkillIndex: i32,                              // 0x10c
            pub _SelfSkillPerformState: i32,                          // 0x110
            pub _RedirectTargetIDIncr: i32,                           // 0x114
            pub CurrentSkillBreakStance: bool,                        // 0x118
            pub _isPassive: bool,                                     // 0x119
            pub CharmAction: bool,                                    // 0x11a
            pub _hasRecordSkill: bool,                                // 0x11b
            pub _AutoStandbyOnCurSkillFinish: bool,                   // 0x11c
            pub _hasOpInSkill: bool,                                  // 0x11d
            pub CurrentSkillHasTriggerEffect: bool,                   // 0x11e
            pub _TargetPerformTimeCounter: f32,                       // 0x120
        }
    }
    pub mod client {
        use crate::kreide::types::*;
        use std::ffi::c_void;
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct AvatarData {
            pub native_object: NativeObject,
            pub LevelUpedBeforeData__BackingField: *const c_void, // 0x10
            pub _TrialEquipment: *const c_void,                   // 0x18
            pub SkinData__BackingField: *const c_void,            // 0x20
            pub RelicsData__BackingField: *const c_void,          // 0x28
            pub Row__BackingField: *const c_void,                 // 0x30
            pub _AvatarRowData: *const c_void,                    // 0x38
            pub UltraSkillConfig__BackingField: *const c_void,    // 0x40
            pub SpecialRow__BackingField: *const c_void,          // 0x48
            pub _AvatarCustomName: *const NativeString,           // 0x50
            pub _SkinIDList: *const NativeList<u32>,              // 0x58
            pub _ExtraPropertyAddition: *const c_void,            // 0x60
            pub GrowUpBeforeData__BackingField: *const c_void,    // 0x68
            pub HasTakenPromotionRewardList__BackingField: *const NativeList<u32>, // 0x70
            pub SkillTreeData: *const c_void,                     // 0x78
            pub _SkillDataMap: *const c_void,                     // 0x80
            pub ServantData__BackingField: *const AvatarServantData, // 0x88
            pub PromotedBeforeData__BackingField: *const c_void,  // 0x90
            pub CombatPowerData__BackingField: *const c_void,     // 0x98
            pub AvatarPropertyData__BackingField: *const c_void,  // 0xa0
            pub IsDisplayOnly__BackingField: bool,                // 0xa8
            pub IsNew__BackingField: bool,                        // 0xa9
            pub IsMarked__BackingField: bool,                     // 0xaa
            pub CurrentExp__BackingField: u32,                    // 0xac
            pub _AdventurePlayerIDOverwrite: u32,                 // 0xb0
            pub EquipmentUID__BackingField: u32,                  // 0xb4
            pub FirstMetTimeStamp: u64,                           // 0xb8
            pub Level__BackingField: u32,                         // 0xc0
            pub Promotion__BackingField: u32,                     // 0xc4
            pub Rank__BackingField: u32,                          // 0xc8
            pub AvatarType__BackingField: i32,                    // 0xcc
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct AvatarServantData {
            pub native_object: NativeObject,
            pub _SkillDataMap: *const c_void,   // 0x10
            pub _AvatarData: *const AvatarData, // 0x18
            pub _Json: *const c_void,           // 0x20
            pub _ServantRowData: *const c_void, // 0x28
            pub _Row: *const c_void,            // 0x30
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct ModuleManager {
            pub native_object: NativeObject,
            pub SwitchHandModule: *const c_void,  // 0x10
            pub PingPongModule: *const c_void,    // 0x18
            pub DialogueModule: *const c_void,    // 0x20
            pub FateSupportModule: *const c_void, // 0x28
            pub GrowthModule: *const c_void,      // 0x30
            pub QuestModule: *const c_void,       // 0x38
            pub AlleyModule: *const c_void,       // 0x40
            pub _ModuleInitRequestList: *const NativeList<NativeObject>, // 0x48
            pub CompanionMissionActivityModule: *const c_void, // 0x50
            pub AnniversaryCollectionModule: *const c_void, // 0x58
            pub ToastQueueModule: *const c_void,  // 0x60
            pub ColonyCollectionPuzzleModule: *const c_void, // 0x68
            pub DrinkMakerModule: *const c_void,  // 0x70
            pub FantasticStoryActivityModule: *const c_void, // 0x78
            pub ActivityFeverTimeModule: *const c_void, // 0x80
            pub RecommendModule: *const c_void,   // 0x88
            pub EntityTimeRewindModule: *const c_void, // 0x90
            pub RelicModule: *const c_void,       // 0x98
            pub MonopolyModule: *const c_void,    // 0xa0
            pub ChallengeModule: *const c_void,   // 0xa8
            pub PerformanceRecallModule: *const c_void, // 0xb0
            pub ActivityBenefitModule: *const c_void, // 0xb8
            pub ActivityPlayerReturnModule: *const c_void, // 0xc0
            pub MaterialSubmissionModule: *const c_void, // 0xc8
            pub MultiFloorConflictModule: *const c_void, // 0xd0
            pub WorldShop4ThModule: *const c_void, // 0xd8
            pub MarbleModule: *const c_void,      // 0xe0
            pub ChatModule: *const c_void,        // 0xe8
            pub TransferModule: *const c_void,    // 0xf0
            pub PunkLordModule: *const c_void,    // 0xf8
            pub CatchGhostModule: *const c_void,  // 0x100
            pub AetherDivideModule: *const c_void, // 0x108
            pub RogueMagicModule: *const c_void,  // 0x110
            pub MultiPathAvatarModule: *const c_void, // 0x118
            pub TeamModule: *const c_void,        // 0x120
            pub MissionChronicleModule: *const c_void, // 0x128
            pub MatchThreeV2Module: *const c_void, // 0x130
            pub ChessRogueModule: *const c_void,  // 0x138
            pub MultipleDropModule: *const c_void, // 0x140
            pub ShareModule: *const c_void,       // 0x148
            pub DirectDeliveryNoticeModule: *const c_void, // 0x150
            pub StoryTokenModule: *const c_void,  // 0x158
            pub ActivityTrackPhotoModule: *const c_void, // 0x160
            pub ActivitySwordTrainingModule: *const c_void, // 0x168
            pub TrainModule: *const c_void,       // 0x170
            pub BattleEventModule: *const c_void, // 0x178
            pub FiveDimModule: *const c_void,     // 0x180
            pub ScheduleModule: *const c_void,    // 0x188
            pub MissionTimelineModule: *const c_void, // 0x190
            pub AchievementModule: *const c_void, // 0x198
            pub ActivityStrongChallengeModule: *const c_void, // 0x1a0
            pub ShopModule: *const c_void,        // 0x1a8
            pub LoadingTipsModule: *const c_void, // 0x1b0
            pub GachaModule: *const c_void,       // 0x1b8
            pub ActivityGuessTheSilhouetteModule: *const c_void, // 0x1c0
            pub RogueTournModule: *const c_void,  // 0x1c8
            pub EvolveBuildModule: *const c_void, // 0x1d0
            pub StoryLineModule: *const c_void,   // 0x1d8
            pub TextJoinModule: *const c_void,    // 0x1e0
            pub OperationModule: *const c_void,   // 0x1e8
            pub ActivityClockParkModule: *const c_void, // 0x1f0
            pub PamModule: *const c_void,         // 0x1f8
            pub ActivityModule: *const c_void,    // 0x200
            pub LobbyModule: *const c_void,       // 0x208
            pub MusicAlbumModule: *const c_void,  // 0x210
            pub SilverWolfModule: *const c_void,  // 0x218
            pub RogueArcadeModule: *const c_void, // 0x220
            pub BoxingClubModule: *const c_void,  // 0x228
            pub DifficultyAdjustModule: *const c_void, // 0x230
            pub PamSkinModule: *const c_void,     // 0x238
            pub FateModule: *const c_void,        // 0x240
            pub FeatureSwitchModule: *const c_void, // 0x248
            pub TalkModule: *const c_void,        // 0x250
            pub AnniversaryAvatarDeliverModule: *const c_void, // 0x258
            pub LuaDataModule: *const c_void,     // 0x260
            pub RogueModule: *const c_void,       // 0x268
            pub PhotoGraphModule: *const c_void,  // 0x270
            pub MovieRacingModule: *const c_void, // 0x278
            pub MuseumModule: *const c_void,      // 0x280
            pub ActivityAetherDivideModule: *const c_void, // 0x288
            pub BattleTipsModule: *const c_void,  // 0x290
            pub RogueAdventureModule: *const c_void, // 0x298
            pub ServerPrefsModule: *const c_void, // 0x2a0
            pub NovelModule: *const c_void,       // 0x2a8
            pub AvatarModule: *const c_void,      // 0x2b0
            pub AdventureModule: *const c_void,   // 0x2b8
            pub FriendModule: *const c_void,      // 0x2c0
            pub MissionModule: *const c_void,     // 0x2c8
            pub PetModule: *const c_void,         // 0x2d0
            pub InventoryModule: *const c_void,   // 0x2d8
            pub AntiAddictionModule: *const c_void, // 0x2e0
            pub TutorialSupportModule: *const c_void, // 0x2e8
            pub CumulativeConsumptionModule: *const c_void, // 0x2f0
            pub PayModule: *const c_void,         // 0x2f8
            pub LoginModule: *const c_void,       // 0x300
            pub BattleModule: *const c_void,      // 0x308
            pub FormationMoveModule: *const c_void, // 0x310
            pub ChimeraModule: *const c_void,     // 0x318
            pub EraFlipperModule: *const c_void,  // 0x320
            pub GamePlayLockModule: *const c_void, // 0x328
            pub ActivitySummonModule: *const c_void, // 0x330
            pub GridFightModule: *const c_void,   // 0x338
            pub StarFightModule: *const c_void,   // 0x340
            pub SpaceZooModule: *const c_void,    // 0x348
            pub SystemOpenModule: *const c_void,  // 0x350
            pub ActivityParkourModule: *const c_void, // 0x358
            pub MapPropOverrideConditionModule: *const c_void, // 0x360
            pub TitanAtlasModule: *const c_void,  // 0x368
            pub EntityScoreModule: *const c_void, // 0x370
            pub HeliobusModule: *const c_void,    // 0x378
            pub BattleCollegeModule: *const c_void, // 0x380
            pub ElfRestaurantModule: *const c_void, // 0x388
            pub OfferingModule: *const c_void,    // 0x390
            pub EarlyAccessModule: *const c_void, // 0x398
            pub MapRotationModule: *const c_void, // 0x3a0
            pub ActivityMusicRhythmModule: *const c_void, // 0x3a8
            pub WolfBroShootingModule: *const c_void, // 0x3b0
            pub TravelBrochureModule: *const c_void, // 0x3b8
            pub ActivityQuestTimeLimitModule: *const c_void, // 0x3c0
            pub FightActivityModule: *const c_void, // 0x3c8
            pub ExpeditionModule: *const c_void,  // 0x3d0
            pub HandbookModule: *const c_void,    // 0x3d8
            pub BattlePassModule: *const c_void,  // 0x3e0
            pub MultiplayerGameModule: *const c_void, // 0x3e8
            pub PersonalizeModule: *const c_void, // 0x3f0
            pub RaidModule: *const c_void,        // 0x3f8
            pub MapConnectivityModule: *const c_void, // 0x400
            pub MessageModule: *const c_void,     // 0x408
            pub PlayerModule: *const c_void,      // 0x410
            pub RaidCollectionModule: *const c_void, // 0x418
            pub MatchThreeModule: *const c_void,  // 0x420
            pub FloorConnectivityModule: *const c_void, // 0x428
            pub PlanetFesModule: *const c_void,   // 0x430
            pub ActivityTelevisionModule: *const c_void, // 0x438
            pub RoleTrialModule: *const c_void,   // 0x440
            pub RollShopModule: *const c_void,    // 0x448
            pub MultiPlayerActivityModule: *const c_void, // 0x450
            pub FindChestModule: *const c_void,   // 0x458
            pub FarmModule: *const c_void,        // 0x460
            pub TreasureDungeonModule: *const c_void, // 0x468
            pub FightFestModule: *const c_void,   // 0x470
            pub TarotBookModule: *const c_void,   // 0x478
            pub BigMapModule: *const c_void,      // 0x480
            pub HeartDialModule: *const c_void,   // 0x488
            pub GameStateServiceModule: *const c_void, // 0x490
            pub TrainPartyModule: *const c_void,  // 0x498
            pub ActivityPhotoExhibitionModule: *const c_void, // 0x4a0
            pub RogueHandbookModule: *const c_void, // 0x4a8
            pub modules: *const NativeList<NativeObject>, // 0x4b0
            pub NavMapModule: *const c_void,      // 0x4b8
            pub ItemComposeModule: *const c_void, // 0x4c0
            pub RechargeShopModule: *const c_void, // 0x4c8
            pub WhiteListInteractUploadModule: *const c_void, // 0x4d0
            pub ArchiveModule: *const c_void,     // 0x4d8
            pub isInited: bool,                   // 0x4e0
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct TextID {
            pub hash: i32,   // 0x0
            pub hash64: u64, // 0x8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct BattleAssetPreload {
            pub native_object: NativeObject,
            pub _AdventurePreloadConfig: *const c_void, // 0x10
            pub _BattleEventAssetPreloadGroups: *const NativeList<NativeObject>, // 0x18
            pub _PetAssetPreloadGroup: *const c_void,   // 0x20
            pub _BattlePreloadConfig: *const c_void,    // 0x28
            pub _ServantAssetPreloadGroups: *const NativeList<NativeObject>, // 0x30
            pub BatttleLineupDataInitCallback: *const c_void, // 0x38
            pub _MonsterAssetPreloadGroups: *const NativeList<NativeObject>, // 0x40
            pub _PreloadAssetDic: *const c_void,        // 0x48
            pub _LineupData: *const BattleLineupData,   // 0x50
            pub _WaitFinishPreloadItems: *const c_void, // 0x58
            pub _PendingPreloadItems: *const c_void,    // 0x60
            pub _InBattleAssetGroup: *const c_void,     // 0x68
            pub _PreBattleAssetGroup: *const c_void,    // 0x70
            pub _monsterLodConfig: *const c_void,       // 0x78
            pub _OnloadFinish: *const c_void,           // 0x80
            pub _InBattlePreloadAIDecisionGroupDic: *const c_void, // 0x88
            pub _AvatarAssetPreloadGroups: *const NativeList<NativeObject>, // 0x90
            pub SummonMonsterList: *const NativeList<u32>, // 0x98
            pub _UIAssetPreloadGroup: *const c_void,    // 0xa0
            pub _AvatarAssetPreloadEnable: bool,        // 0xa8
            pub _IsDirty: bool,                         // 0xa9
            pub _IsAsyncLoadItems: bool,                // 0xaa
        }
    }
}
