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
    pub HKFGOHGKOGK: *const c_void,                    // 0x10
    pub KNDJNKNHFFG: *const TurnBasedAbilityComponent, // 0x18
    pub MDEHKOOKJCK: *const NativeList<NativeObject>,  // 0x20
    pub BEAJGANIDLJ: *const c_void,                    // 0x28
    pub JKCOIOLCMEP: *const TurnBasedAbilityComponent, // 0x30
    pub FKKDFMPMJHG: *const NativeArray<NativeObject>, // 0x38
    pub LGGEDDMACDF: *const NativeString,              // 0x40
    pub PBHCGDFPEED: *const c_void,                    // 0x48
    pub JODAJBNCCNP: *const NativeArray<NativeObject>, // 0x50
    pub AAHMMHBHMFN: [u8; 0x90],                       // 0x58
    pub PJLPGAGKIDE: FixPoint,                         // 0xe8
    pub AMAJNHHAJIM: FixPoint,                         // 0xf0
    pub JCPEINMPKAM: FixPoint,                         // 0xf8
    pub JNFPCNAKNOP: FixPoint,                         // 0x100
    pub BGBOFNMKDNJ: FixPoint,                         // 0x108
    pub HHEIPBOKCOH: [u8; 0x40],                       // 0x110
    pub GHBPOPKEGLE: FixPoint,                         // 0x150
    pub APDLLHIMMEM: FixPoint,                         // 0x158
    pub NCOHIAPKAED: FixPoint,                         // 0x160
    pub PJPKDAKBEJI: FixPoint,                         // 0x168
    pub POLANGDKOKH: FixPoint,                         // 0x170
    pub GCNOMMHFPOG: FixPoint,                         // 0x178
    pub FGIPOLJPICM: FixPoint,                         // 0x180
    pub EBDJHPNOALL: FixPoint,                         // 0x188
    pub MKNDMBOCCBO: FixPoint,                         // 0x190
    pub DBBDIMCJIKE: FixPoint,                         // 0x198
    pub KDJBABPDHEG: FixPoint,                         // 0x1a0
    pub JEHMOKDJDDE: FixPoint,                         // 0x1a8
    pub MHEBPGAHFCB: FixPoint,                         // 0x1b0
    pub NHHNLMOBEGH: FixPoint,                         // 0x1b8
    pub KOCOLHHLFLD: [u8; 0x40],                       // 0x1c0
    pub MAKENPDPHDN: FixPoint,                         // 0x200
    pub DBNKBGKCMKH: FixPoint,                         // 0x208
    pub CCLFKIPGJOG: FixPoint,                         // 0x210
    pub GAALBDHLFOG: FixPoint,                         // 0x218
    pub FOLCDHNIMMI: FixPoint,                         // 0x220
    pub PNGJIDMHIOE: FixPoint,                         // 0x228
    pub HNJBAFCNNDD: FixPoint,                         // 0x230
    pub CAILJEGIDKL: FixPoint,                         // 0x238
    pub CMNBOEIDAOD: FixPoint,                         // 0x240
    pub EPJEDLOBFPG: FixPoint,                         // 0x248
    pub GFFCEBJGABG: bool,                             // 0x250
    pub HKNLHAMMIIM: bool,                             // 0x251
    pub HEMFDDDJOGK: bool,                             // 0x252
    pub EKBHFCODKFO: bool,                             // 0x253
    pub APDDLHNGGIM: AttackType,                       // 0x254
    pub PGGOANFBJON: FixPoint,                         // 0x258
    pub BEGDMOGLLGM: FixPoint,                         // 0x260
    pub CINNHMENLIJ: FixPoint,                         // 0x268
    pub BJAEJMLMJCL: FixPoint,                         // 0x270
    pub EBDJIHNKAOC: FixPoint,                         // 0x278
    pub GMBACFCLEGD: FixPoint,                         // 0x280
    pub EFAAJEAENFF: FixPoint,                         // 0x288
    pub GLPLDJKMOBE: FixPoint,                         // 0x290
    pub HMMMDOHLFEP: FixPoint,                         // 0x298
    pub JFMADBFKBDK: FixPoint,                         // 0x2a0
    pub MGFECPHDPHB: FixPoint,                         // 0x2a8
    pub ILNAKPIOOAK: FixPoint,                         // 0x2b0
    pub PDCMJAMPJNL: FixPoint,                         // 0x2b8
    pub BDLFBDLDEND: [u8; 0x48],                       // 0x2c0
    pub AHOCGHANMCE: FixPoint,                         // 0x308
    pub BBNMJNPDOCP: FixPoint,                         // 0x310
    pub ALOGNJIBIPG: FixPoint,                         // 0x318
    pub PGOHAIPOCNK: FixPoint,                         // 0x320
    pub GIHPOCDLJOA: FixPoint,                         // 0x328
    pub LJGPDLDGCEO: FixPoint,                         // 0x330
    pub GNMAKKBFOCH: FixPoint,                         // 0x338
    pub MKIMEBNOEGI: FixPoint,                         // 0x340
    pub ENFFBMJBEDP: FixPoint,                         // 0x348
    pub MNGPDEOEHPE: FixPoint,                         // 0x350
    pub HJAEPANAFLN: FixPoint,                         // 0x358
    pub KOEGLFLGADD: FixPoint,                         // 0x360
    pub MKMILJKLJON: [u8; 0x58],                       // 0x368
    pub KLMAGCLFBAO: FixPoint,                         // 0x3c0
    pub NKIAEHFPJEA: i32,                              // 0x3c8
    pub EJJMIFKCFHP: bool,                             // 0x3cc
    pub CAANBNCPACE: bool,                             // 0x3cd
    pub KDCHAHHPPGD: bool,                             // 0x3ce
    pub AHPFPMEGEKG: bool,                             // 0x3cf
    pub FFFOLNDHIEH: [u8; 0x48],                       // 0x3d0
    pub GBENLNNEIJM: FixPoint,                         // 0x418
    pub BLFCEOMPDKK: FixPoint,                         // 0x420
    pub DGFBMAPFPNH: FixPoint,                         // 0x428
    pub FLMEBELNIKK: FixPoint,                         // 0x430
    pub DJCAFPFKOGP: FixPoint,                         // 0x438
    pub HCGBHCPHDKJ: FixPoint,                         // 0x440
    pub ANHNDBECCJD: [u8; 0x40],                       // 0x448
    pub JIINJMJGCOH: FixPoint,                         // 0x488
    pub PJNEJPNBNMP: FixPoint,                         // 0x490
    pub DEOICHHPAIF: FixPoint,                         // 0x498
    pub FNDCNMHMCIC: FixPoint,                         // 0x4a0
    pub ODBPMMGBKGA: FixPoint,                         // 0x4a8
    pub ELGMFJLGCPH: FixPoint,                         // 0x4b0
    pub KBKGNDFAKGD: bool,                             // 0x4b8
    pub DPEJKHJPLAC: bool,                             // 0x4b9
    pub MNAPDDFFHJF: bool,                             // 0x4ba
    pub JGHJIGOCPNP: i32,                              // 0x4bc
    pub JFKEEOMKMLI: FixPoint,                         // 0x4c0
    pub CGMHNNNOKAI: FixPoint,                         // 0x4c8
    pub DINCHAHPEAC: FixPoint,                         // 0x4d0
    pub KODEDHBLGGH: FixPoint,                         // 0x4d8
    pub DKOIGIHEBCD: FixPoint,                         // 0x4e0
    pub GOHOJAIMDNM: FixPoint,                         // 0x4e8
    pub KPELFJICFDH: FixPoint,                         // 0x4f0
    pub NEPGNKOMAAA: FixPoint,                         // 0x4f8
    pub COKMLMJPKLH: u32,                              // 0x500
    pub AHHEDGLMDMG: i32,                              // 0x504
    pub EFFODBPOOCN: FixPoint,                         // 0x508
    pub DCEBGGFOFAO: FixPoint,                         // 0x510
    pub OHBMMFAFMDP: FixPoint,                         // 0x518
    pub COIDNPMCCFG: FixPoint,                         // 0x520
    pub IAAJMHADJDG: FixPoint,                         // 0x528
    pub MJMDGNPPILN: FixPoint,                         // 0x530
    pub OEPAPFDLMML: FixPoint,                         // 0x538
    pub PAIGBKBOKDI: FixPoint,                         // 0x540
    pub OJGNIBKADHK: u32,                              // 0x548
    pub CFBOJBAJCEA: i32,                              // 0x54c
    pub NAGMKEABGEE: FixPoint,                         // 0x550
    pub FFCGIMAMDPP: FixPoint,                         // 0x558
    pub GLGFEKEMMJJ: FixPoint,                         // 0x560
    pub ABIPIIBIIBE: FixPoint,                         // 0x568
    pub BDGDFKGOLPJ: FixPoint,                         // 0x570
    pub IICNDPJGCFA: i32,                              // 0x578
    pub KMIKODLPNGL: i32,                              // 0x57c
    pub DJHDAOOEJOF: FixPoint,                         // 0x580
    pub DPPDEDGCLJJ: FixPoint,                         // 0x588
    pub JHOHCEFOJNB: FixPoint,                         // 0x590
    pub GCFCCDPIACO: FixPoint,                         // 0x598
    pub FMMBMJKNAHI: FixPoint,                         // 0x5a0
    pub BKIFAEKCIHN: FixPoint,                         // 0x5a8
    pub GJNAGCJONAO: FixPoint,                         // 0x5b0
    pub ACDFHOGEMCC: [u8; 0x40],                       // 0x5b8
    pub MLKFKKACBCE: FixPoint,                         // 0x5f8
    pub GCGEEFLGCIG: i32,                              // 0x600
    pub IJJHMGEHMHB: bool,                             // 0x604
    pub JICCOEHBPJJ: bool,                             // 0x605
    pub FNBALMGFGDM: bool,                             // 0x606
    pub EGINKGPDNPK: bool,                             // 0x607
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MMNDIEBMDNL {
    pub native_object: NativeObject,
    pub HECCDOHIAFD: *const SkillCharacterComponent, // 0x10
    pub GNBEIGMFGIP: *const c_void,                  // 0x18
    pub MKMMNLODHDD: *const c_void,                  // 0x20
    pub HMCDHMFHABF: OLHMAHMMBNN,                    // 0x28
    pub FIMNOPAAFEP: *const TurnBasedAbilityComponent, // 0x80
    pub OOIFIGDBNBO: i32,                            // 0x88
    pub NMJEMHAMIHD: i32,                            // 0x8c
    pub DADCNHAIOMI: i32,                            // 0x90
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EDJEDBLFIKE {
    pub _parent_object: GameComponentBase,             // 0x0
    pub FLFNAFNNMBN: *const c_void,                    // 0x18
    pub MBDGPOJNGFP: *const c_void,                    // 0x20
    pub LNILILLBCGC: *const NativeString,              // 0x28
    pub CJAJFECDPPJ: *const GameEntity,                // 0x30
    pub LJHGLKMAPBE: *const TeamFormationComponent,    // 0x38
    pub HLEGPKNHFFM: *const c_void,                    // 0x40
    pub PFPDENDBJKL: *const c_void,                    // 0x48
    pub MNIJEBNCBKB: *const GameEntity,                // 0x50
    pub PIJCPBGIEOC: *const NativeArray<NativeObject>, // 0x58
    pub PEKEMAPHMPK: *const c_void,                    // 0x60
    pub OGBHOPFFGFP: *const GameEntity,                // 0x68
    pub LOHMAMHFIGP: *const GameEntity,                // 0x70
    pub MJEBCDHCGMH: *const EDJEDBLFIKE,               // 0x78
    pub EADOCJKPPBN: *const c_void,                    // 0x80
    pub PAPNHNOOPMK: *const c_void,                    // 0x88
    pub BKOFKGCANPN: *const c_void,                    // 0x90
    pub ECHFEHNDOJF: *const NativeArray<NativeObject>, // 0x98
    pub PGPNDEIFMMF: *const NativeList<EDJEDBLFIKE>,   // 0xa0
    pub DAGNFDKMDEF: *const NativeString,              // 0xa8
    pub PLAODOKPNKA: i32,                              // 0xb0
    pub ANGNPNFAMKA: [u8; 0xc],                        // 0xb4
    pub JABPKHKIBHM: i32,                              // 0xc0
    pub DMGBPALONGL: f32,                              // 0xc4
    pub DMGGHOFAADK: i32,                              // 0xc8
    pub FMKFHCDFJFD: i32,                              // 0xcc
    pub OJABHABFLKK: TeamType,                         // 0xd0
    pub GDHEHGIAEFE: bool,                             // 0xd4
    pub PMAHDELPGFH: bool,                             // 0xd5
    pub HBMKPLPDABN: bool,                             // 0xd6
    pub FCOAHCGBOPH: bool,                             // 0xd7
    pub MNMBEKKJOPN: i32,                              // 0xd8
    pub JPACGCDPCEP: f32,                              // 0xdc
    pub FOMIFIJPOBA: f32,                              // 0xe0
    pub EAIAJIOPLDM: bool,                             // 0xe4
    pub LKBINIFLPIK: bool,                             // 0xe5
    pub IsInit__BackingField: bool,                    // 0xe6
    pub CCKJAGJDNCK: bool,                             // 0xe7
    pub IOLCGFPHNBM: bool,                             // 0xe8
    pub HFNDIAFABLD: bool,                             // 0xe9
    pub MJFIGBNFCMP: bool,                             // 0xea
    pub NJAKDBCILEM: bool,                             // 0xeb
    pub DEFJLDFMHJA: [u8; 0xc],                        // 0xec
    pub FDNNGFMGPBH: f32,                              // 0xf8
    pub FLMDDGMPOIN: [u8; 0xc],                        // 0xfc
    pub KFKBEGPNBBA: u32,                              // 0x108
    pub IEFGFPBNLJE: i32,                              // 0x10c
    pub MLAHBDLNOAA: f32,                              // 0x110
    pub HOMHCNECBND: [u8; 0xc],                        // 0x114
    pub ODCPODIECFK: i32,                              // 0x120
    pub MOEFEJMBFHN: f32,                              // 0x124
    pub FMJMBKNPLPA: f32,                              // 0x128
    pub BGPEBFGCIFI: [u8; 0xc],                        // 0x12c
    pub ALBCLOBMAAA: i32,                              // 0x138
    pub BFKAEAEMEIF: f32,                              // 0x13c
}
pub mod rpg {
    use crate::kreide::types::*;
    use std::ffi::c_void;
    pub mod client {
        use crate::kreide::types::*;
        use std::ffi::c_void;
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct AvatarData {
            pub native_object: NativeObject,
            pub CombatPowerData__BackingField: *const c_void, // 0x10
            pub PromotedBeforeData__BackingField: *const c_void, // 0x18
            pub LevelUpedBeforeData__BackingField: *const c_void, // 0x20
            pub _SkillDataMap: *const c_void,                 // 0x28
            pub _SkinIDList: *const NativeList<u32>,          // 0x30
            pub GrowUpBeforeData__BackingField: *const c_void, // 0x38
            pub _AvatarRowData: *const c_void,                // 0x40
            pub _AvatarCustomName: *const NativeString,       // 0x48
            pub SkinData__BackingField: *const c_void,        // 0x50
            pub Row__BackingField: *const c_void,             // 0x58
            pub SpecialRow__BackingField: *const c_void,      // 0x60
            pub RelicsData__BackingField: *const c_void,      // 0x68
            pub SkillTreeData: *const c_void,                 // 0x70
            pub _ExtraPropertyAddition: *const c_void,        // 0x78
            pub HasTakenPromotionRewardList__BackingField: *const NativeList<u32>, // 0x80
            pub ServantData__BackingField: *const AvatarServantData, // 0x88
            pub _TrialEquipment: *const c_void,               // 0x90
            pub UltraSkillConfig__BackingField: *const c_void, // 0x98
            pub AvatarPropertyData__BackingField: *const c_void, // 0xa0
            pub _AdventurePlayerIDOverwrite: u32,             // 0xa8
            pub CurrentExp__BackingField: u32,                // 0xac
            pub IsMarked__BackingField: bool,                 // 0xb0
            pub IsNew__BackingField: bool,                    // 0xb1
            pub IsDisplayOnly__BackingField: bool,            // 0xb2
            pub Rank__BackingField: u32,                      // 0xb4
            pub FirstMetTimeStamp: u64,                       // 0xb8
            pub EquipmentUID__BackingField: u32,              // 0xc0
            pub Promotion__BackingField: u32,                 // 0xc4
            pub AvatarType__BackingField: i32,                // 0xc8
            pub Level__BackingField: u32,                     // 0xcc
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct BattleAssetPreload {
            pub native_object: NativeObject,
            pub _WaitFinishPreloadItems: *const c_void, // 0x10
            pub _BattleEventAssetPreloadGroups: *const NativeList<NativeObject>, // 0x18
            pub _InBattleAssetGroup: *const c_void,     // 0x20
            pub _LineupData: *const BattleLineupData,   // 0x28
            pub _AvatarAssetPreloadGroups: *const NativeList<NativeObject>, // 0x30
            pub _PendingPreloadItems: *const c_void,    // 0x38
            pub _MonsterAssetPreloadGroups: *const NativeList<NativeObject>, // 0x40
            pub _monsterLodConfig: *const c_void,       // 0x48
            pub _BattlePreloadConfig: *const c_void,    // 0x50
            pub BatttleLineupDataInitCallback: *const c_void, // 0x58
            pub SummonMonsterList: *const NativeList<u32>, // 0x60
            pub _UIAssetPreloadGroup: *const c_void,    // 0x68
            pub _PetAssetPreloadGroup: *const c_void,   // 0x70
            pub _PreBattleAssetGroup: *const c_void,    // 0x78
            pub _OnloadFinish: *const c_void,           // 0x80
            pub _PreloadAssetDic: *const c_void,        // 0x88
            pub _InBattlePreloadAIDecisionGroupDic: *const c_void, // 0x90
            pub _AdventurePreloadConfig: *const c_void, // 0x98
            pub _ServantAssetPreloadGroups: *const NativeList<NativeObject>, // 0xa0
            pub _AvatarAssetPreloadEnable: bool,        // 0xa8
            pub _IsDirty: bool,                         // 0xa9
            pub _IsAsyncLoadItems: bool,                // 0xaa
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct ModuleManager {
            pub native_object: NativeObject,
            pub AdventureModule: *const c_void,  // 0x10
            pub StoryTokenModule: *const c_void, // 0x18
            pub MapPropOverrideConditionModule: *const c_void, // 0x20
            pub HandbookModule: *const c_void,   // 0x28
            pub MessageModule: *const c_void,    // 0x30
            pub DrinkMakerModule: *const c_void, // 0x38
            pub ChallengeModule: *const c_void,  // 0x40
            pub PlayerModule: *const c_void,     // 0x48
            pub AlleyModule: *const c_void,      // 0x50
            pub NovelModule: *const c_void,      // 0x58
            pub AetherDivideModule: *const c_void, // 0x60
            pub ActivityClockParkModule: *const c_void, // 0x68
            pub ExpeditionModule: *const c_void, // 0x70
            pub ShareModule: *const c_void,      // 0x78
            pub TextJoinModule: *const c_void,   // 0x80
            pub EraFlipperModule: *const c_void, // 0x88
            pub ActivityGuessTheSilhouetteModule: *const c_void, // 0x90
            pub PhotoGraphModule: *const c_void, // 0x98
            pub WorldShop4ThModule: *const c_void, // 0xa0
            pub FindChestModule: *const c_void,  // 0xa8
            pub GachaModule: *const c_void,      // 0xb0
            pub GameStateServiceModule: *const c_void, // 0xb8
            pub PerformanceRecallModule: *const c_void, // 0xc0
            pub BattlePassModule: *const c_void, // 0xc8
            pub ActivityAetherDivideModule: *const c_void, // 0xd0
            pub BigMapModule: *const c_void,     // 0xd8
            pub ActivitySwordTrainingModule: *const c_void, // 0xe0
            pub BattleCollegeModule: *const c_void, // 0xe8
            pub RogueModule: *const c_void,      // 0xf0
            pub PetModule: *const c_void,        // 0xf8
            pub TutorialSupportModule: *const c_void, // 0x100
            pub MultiFloorConflictModule: *const c_void, // 0x108
            pub MissionChronicleModule: *const c_void, // 0x110
            pub TravelBrochureModule: *const c_void, // 0x118
            pub OperationModule: *const c_void,  // 0x120
            pub LuaDataModule: *const c_void,    // 0x128
            pub SwitchHandModule: *const c_void, // 0x130
            pub ActivityModule: *const c_void,   // 0x138
            pub ChessRogueModule: *const c_void, // 0x140
            pub LobbyModule: *const c_void,      // 0x148
            pub AvatarModule: *const c_void,     // 0x150
            pub MultipleDropModule: *const c_void, // 0x158
            pub MapConnectivityModule: *const c_void, // 0x160
            pub TransferModule: *const c_void,   // 0x168
            pub MatchThreeV2Module: *const c_void, // 0x170
            pub MovieRacingModule: *const c_void, // 0x178
            pub QuestModule: *const c_void,      // 0x180
            pub MapRotationModule: *const c_void, // 0x188
            pub ScheduleModule: *const c_void,   // 0x190
            pub BattleEventModule: *const c_void, // 0x198
            pub FarmModule: *const c_void,       // 0x1a0
            pub ActivityStrongChallengeModule: *const c_void, // 0x1a8
            pub RogueArcadeModule: *const c_void, // 0x1b0
            pub FeatureSwitchModule: *const c_void, // 0x1b8
            pub TeamModule: *const c_void,       // 0x1c0
            pub ToastQueueModule: *const c_void, // 0x1c8
            pub MaterialSubmissionModule: *const c_void, // 0x1d0
            pub WhiteListInteractUploadModule: *const c_void, // 0x1d8
            pub RaidModule: *const c_void,       // 0x1e0
            pub RoleTrialModule: *const c_void,  // 0x1e8
            pub modules: *const NativeList<NativeObject>, // 0x1f0
            pub AnniversaryAvatarDeliverModule: *const c_void, // 0x1f8
            pub ActivityParkourModule: *const c_void, // 0x200
            pub WolfBroShootingModule: *const c_void, // 0x208
            pub StarFightModule: *const c_void,  // 0x210
            pub ColonyCollectionPuzzleModule: *const c_void, // 0x218
            pub FriendModule: *const c_void,     // 0x220
            pub FloorConnectivityModule: *const c_void, // 0x228
            pub MuseumModule: *const c_void,     // 0x230
            pub StoryLineModule: *const c_void,  // 0x238
            pub BattleTipsModule: *const c_void, // 0x240
            pub OfferingModule: *const c_void,   // 0x248
            pub TrainPartyModule: *const c_void, // 0x250
            pub ActivityFeverTimeModule: *const c_void, // 0x258
            pub HeliobusModule: *const c_void,   // 0x260
            pub MultiPathAvatarModule: *const c_void, // 0x268
            pub ActivityPlayerReturnModule: *const c_void, // 0x270
            pub CatchGhostModule: *const c_void, // 0x278
            pub RaidCollectionModule: *const c_void, // 0x280
            pub ActivitySummonModule: *const c_void, // 0x288
            pub ActivityPhotoExhibitionModule: *const c_void, // 0x290
            pub NavMapModule: *const c_void,     // 0x298
            pub FateSupportModule: *const c_void, // 0x2a0
            pub MarbleModule: *const c_void,     // 0x2a8
            pub ArchiveModule: *const c_void,    // 0x2b0
            pub FantasticStoryActivityModule: *const c_void, // 0x2b8
            pub LoginModule: *const c_void,      // 0x2c0
            pub DifficultyAdjustModule: *const c_void, // 0x2c8
            pub MatchThreeModule: *const c_void, // 0x2d0
            pub MultiPlayerActivityModule: *const c_void, // 0x2d8
            pub CumulativeConsumptionModule: *const c_void, // 0x2e0
            pub TitanAtlasModule: *const c_void, // 0x2e8
            pub DirectDeliveryNoticeModule: *const c_void, // 0x2f0
            pub PamModule: *const c_void,        // 0x2f8
            pub PamSkinModule: *const c_void,    // 0x300
            pub EvolveBuildModule: *const c_void, // 0x308
            pub PingPongModule: *const c_void,   // 0x310
            pub GridFightModule: *const c_void,  // 0x318
            pub TalkModule: *const c_void,       // 0x320
            pub TreasureDungeonModule: *const c_void, // 0x328
            pub CompanionMissionActivityModule: *const c_void, // 0x330
            pub ChatModule: *const c_void,       // 0x338
            pub _ModuleInitRequestList: *const NativeList<NativeObject>, // 0x340
            pub PayModule: *const c_void,        // 0x348
            pub SystemOpenModule: *const c_void, // 0x350
            pub FiveDimModule: *const c_void,    // 0x358
            pub RogueTournModule: *const c_void, // 0x360
            pub BoxingClubModule: *const c_void, // 0x368
            pub MissionTimelineModule: *const c_void, // 0x370
            pub FateModule: *const c_void,       // 0x378
            pub TarotBookModule: *const c_void,  // 0x380
            pub SpaceZooModule: *const c_void,   // 0x388
            pub AchievementModule: *const c_void, // 0x390
            pub BattleModule: *const c_void,     // 0x398
            pub ActivityQuestTimeLimitModule: *const c_void, // 0x3a0
            pub AnniversaryCollectionModule: *const c_void, // 0x3a8
            pub MultiplayerGameModule: *const c_void, // 0x3b0
            pub ShopModule: *const c_void,       // 0x3b8
            pub MonopolyModule: *const c_void,   // 0x3c0
            pub DialogueModule: *const c_void,   // 0x3c8
            pub FightActivityModule: *const c_void, // 0x3d0
            pub SilverWolfModule: *const c_void, // 0x3d8
            pub InventoryModule: *const c_void,  // 0x3e0
            pub RogueAdventureModule: *const c_void, // 0x3e8
            pub RecommendModule: *const c_void,  // 0x3f0
            pub EntityTimeRewindModule: *const c_void, // 0x3f8
            pub PersonalizeModule: *const c_void, // 0x400
            pub ServerPrefsModule: *const c_void, // 0x408
            pub GrowthModule: *const c_void,     // 0x410
            pub ActivityMusicRhythmModule: *const c_void, // 0x418
            pub HeartDialModule: *const c_void,  // 0x420
            pub ActivityBenefitModule: *const c_void, // 0x428
            pub RollShopModule: *const c_void,   // 0x430
            pub ActivityTelevisionModule: *const c_void, // 0x438
            pub MissionModule: *const c_void,    // 0x440
            pub ChimeraModule: *const c_void,    // 0x448
            pub AntiAddictionModule: *const c_void, // 0x450
            pub MusicAlbumModule: *const c_void, // 0x458
            pub ActivityTrackPhotoModule: *const c_void, // 0x460
            pub PlanetFesModule: *const c_void,  // 0x468
            pub RechargeShopModule: *const c_void, // 0x470
            pub ElfRestaurantModule: *const c_void, // 0x478
            pub FightFestModule: *const c_void,  // 0x480
            pub TrainModule: *const c_void,      // 0x488
            pub FormationMoveModule: *const c_void, // 0x490
            pub RogueHandbookModule: *const c_void, // 0x498
            pub RogueMagicModule: *const c_void, // 0x4a0
            pub LoadingTipsModule: *const c_void, // 0x4a8
            pub GamePlayLockModule: *const c_void, // 0x4b0
            pub EntityScoreModule: *const c_void, // 0x4b8
            pub RelicModule: *const c_void,      // 0x4c0
            pub PunkLordModule: *const c_void,   // 0x4c8
            pub ItemComposeModule: *const c_void, // 0x4d0
            pub EarlyAccessModule: *const c_void, // 0x4d8
            pub isInited: bool,                  // 0x4e0
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct TextID {
            pub hash: i32,   // 0x0
            pub hash64: u64, // 0x8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct AvatarServantData {
            pub native_object: NativeObject,
            pub _ServantRowData: *const c_void, // 0x10
            pub _AvatarData: *const AvatarData, // 0x18
            pub _Row: *const c_void,            // 0x20
            pub _Json: *const c_void,           // 0x28
            pub _SkillDataMap: *const c_void,   // 0x30
        }
    }
    pub mod gamecore {
        use crate::kreide::types::*;
        use std::ffi::c_void;
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct SkillData {
            pub native_object: NativeObject,
            pub OverrideAnimState: *const NativeString, // 0x10
            pub OverrideTargetInfo: *const c_void,      // 0x18
            pub VisibleCondTask: *const c_void,         // 0x20
            pub OverrideCameraConfig: *const c_void,    // 0x28
            pub ParentSkillData: *const SkillData,      // 0x30
            pub _Slot: *const c_void,                   // 0x38
            pub DefaultTargetInfo: *const c_void,       // 0x40
            pub SkillTypeOverride: *const c_void,       // 0x48
            pub UsableConditionDatas: *const NativeList<NativeObject>, // 0x50
            pub PreshowConditions: *const NativeArray<NativeObject>, // 0x58
            pub SkillTriggerKey: *const NativeString,   // 0x60
            pub CustomReadyConfigConditions: *const NativeArray<NativeObject>, // 0x68
            pub Config: *const c_void,                  // 0x70
            pub AllChildSkillDatas: *const NativeList<SkillData>, // 0x78
            pub _SkillProperties: *const NativeArray<NativeObject>, // 0x80
            pub OverrideCameraConfigAdded: *const c_void, // 0x88
            pub RowData: *const c_void,                 // 0x90
            pub SkillCom: *const SkillCharacterComponent, // 0x98
            pub InsertCondTask: *const c_void,          // 0xa0
            pub CommonActiveSkillID: u32,               // 0xa8
            pub DefaultCoolDown: i32,                   // 0xac
            pub CurrentCoolDown: i32,                   // 0xb0
            pub SkillConfigID: u32,                     // 0xb4
            pub ChildIndex: i32,                        // 0xb8
            pub LeftCastTimes: i32,                     // 0xbc
            pub AttackDamageTypePreshowAttach: i32,     // 0xc0
            pub SkillIndex: i32,                        // 0xc4
            pub MaxCastTimes: i32,                      // 0xc8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct MonsterDataComponent {
            pub _parent_object: CharacterDataComponent,      // 0x0
            pub EnergyBarState: *const c_void,               // 0xe0
            pub _DefaultPhaseConfig: *const c_void,          // 0xe8
            pub _CreateParams: *const c_void,                // 0xf0
            pub _MonsterRowData: *const MonsterRowData,      // 0xf8
            pub _MultiActionCounter: *const c_void,          // 0x100
            pub _CustomDataRef: *const c_void,               // 0x108
            pub _PhaseMaxStanceStackIndex: i32,              // 0x110
            pub IsMuteLastKill__BackingField: bool,          // 0x114
            pub _CurrentPhaseHPRecovered: bool,              // 0x115
            pub _DefaultMaxStance: FixPoint,                 // 0x118
            pub _CurrentPhase: u32,                          // 0x120
            pub MonsterIndexInWave: i32,                     // 0x124
            pub _DefaultMaxHP: FixPoint,                     // 0x128
            pub OverrideRankScore__BackingField: [u8; 0x10], // 0x130
            pub MonsterWave: i32,                            // 0x140
            pub _PhaseMaxHPStackIndex: i32,                  // 0x144
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct SkillCharacterComponent {
            pub _parent_object: GameComponentBase, // 0x0
            pub CurrentAimAtTargetList: *const NativeList<GameEntity>, // 0x18
            pub CurrentAimAtMainTargetList: *const NativeList<GameEntity>, // 0x20
            pub SkillPointEntity__BackingField: *const GameEntity, // 0x28
            pub _SkillDataList: *const NativeList<SkillData>, // 0x30
            pub _TBAbilityRef: *const TurnBasedAbilityComponent, // 0x38
            pub CurrentSkillTargetCharacterId: *const c_void, // 0x40
            pub AutoUseUltraParams: *const c_void, // 0x48
            pub _recordAbilityInfo: [u8; 0x30],    // 0x50
            pub CurrentSkillTargetDamageHP: *const c_void, // 0x80
            pub CurrentSkillSubTargetList__BackingField: *const NativeList<GameEntity>, // 0x88
            pub _SkillTargetRedirectEntries: *const NativeList<NativeObject>, // 0x90
            pub OnSkillSetup: *const NativeList<NativeObject>, // 0x98
            pub SkillActualAttacker__BackingField: *const GameEntity, // 0xa0
            pub _SkillTypeDisableCountArr: *const NativeArray<i32>, // 0xa8
            pub CurrentSkillTargetList__BackingField: *const NativeList<GameEntity>, // 0xb0
            pub _SkillTypeDisableSlots: *const c_void, // 0xb8
            pub _JsonConfigRef: *const CharacterConfig, // 0xc0
            pub _CharacterDataRef: *const CharacterDataComponent, // 0xc8
            pub _SkillTypeOverrideProperty: *const NativeArray<NativeObject>, // 0xd0
            pub _SkillSlots: *const NativeArray<NativeObject>, // 0xd8
            pub CurrentAimAtSubTargetList: *const NativeList<GameEntity>, // 0xe0
            pub TaskContext__BackingField: *const c_void, // 0xe8
            pub _hasRecordSkill: bool,             // 0xf0
            pub _hasOpInSkill: bool,               // 0xf1
            pub _AutoStandbyOnCurSkillFinish: bool, // 0xf2
            pub PassiveUsed__BackingField: bool,   // 0xf3
            pub SelfWaitActiveSkillIndex: i32,     // 0xf4
            pub _SelfSkillPerformState: i32,       // 0xf8
            pub _RedirectTargetIDIncr: i32,        // 0xfc
            pub _TargetPerformTimeCounter: f32,    // 0x100
            pub _CurrentSkillIndex: i32,           // 0x104
            pub CharmAction: bool,                 // 0x108
            pub IsNoBpCost__BackingField: bool,    // 0x109
            pub _isPassive: bool,                  // 0x10a
            pub IsNoSpCost__BackingField: bool,    // 0x10b
            pub _OpIndexInSkill: i32,              // 0x10c
            pub _RecordSkillExtraUseParam: i32,    // 0x110
            pub _actionSkillIndex: i32,            // 0x114
            pub CurrentSkillKillAllOrBoss: bool,   // 0x118
            pub CurrentSkillHasTriggerEffect: bool, // 0x119
            pub CurrentSkillBreakStance: bool,     // 0x11a
            pub CurrentSkillKilledCount: i32,      // 0x11c
            pub _CurrentSkillExtraUseParam: i32,   // 0x120
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct TurnBasedAbilityComponent {
            pub _parent_object: GameComponentBase,          // 0x0
            pub CustomDataRef__BackingField: *const c_void, // 0x18
            pub RegardAsAttackTypeMap: *const NativeList<NativeObject>, // 0x20
            pub _AbilityProperties: *const NativeArray<NativeObject>, // 0x28
            pub _KillerEntity: *const GameEntity,           // 0x30
            pub _SyncPropertySource: *const TurnBasedAbilityComponent, // 0x38
            pub _StatusProbabilityDict: *const c_void,      // 0x40
            pub LockActionDelayChange: *const c_void,       // 0x48
            pub OverflowStanceDamageAttacker__BackingField: *const GameEntity, // 0x50
            pub _ExtraStanceInfo: *const c_void,            // 0x58
            pub _DamageStoreList: *const NativeList<NativeObject>, // 0x60
            pub _ExtraMaxLayerConfig: *const NativeList<NativeObject>, // 0x68
            pub _StancePreshowConfigs: *const NativeList<NativeObject>, // 0x70
            pub CharmDamageAttackProperty: *const c_void,   // 0x78
            pub _LockShieldCounter: *const c_void,          // 0x80
            pub _DepartedParams: *const NativeList<NativeObject>, // 0x88
            pub _EnableNegativeHPSourceList: *const NativeList<NativeObject>, // 0x90
            pub _BuffLockStepSources: *const NativeList<NativeObject>, // 0x98
            pub ProjectileTargetAttachPoint: *const NativeString, // 0xa0
            pub _CharacterChangeParam: *const c_void,       // 0xa8
            pub _ModifierEventSourceMuteCounter: *const c_void, // 0xb0
            pub CharmDamageTarget: *const GameEntity,       // 0xb8
            pub AbilityComponentRef__BackingField: *const c_void, // 0xc0
            pub _TransformRef: *const c_void,               // 0xc8
            pub CharmSkillName: *const NativeString,        // 0xd0
            pub _DelayModifyActionDelayQueue: *const c_void, // 0xd8
            pub _DamagedEntityListInAttack: *const NativeList<GameEntity>, // 0xe0
            pub AddModifierBindValueMapping: *const c_void, // 0xe8
            pub Weakness: *const c_void,                    // 0xf0
            pub OnAbilityPropertyChanged: *const NativeList<NativeObject>, // 0xf8
            pub _DefaultStanceInfo: *const c_void,          // 0x100
            pub _CharacterChangeSource: *const GameEntity,  // 0x108
            pub _AttackDamageLog: *const NativeList<NativeObject>, // 0x110
            pub RegardAsSkillTypeMap: *const NativeList<NativeObject>, // 0x118
            pub AdditionalAbilityParamList: *const NativeArray<NativeObject>, // 0x120
            pub _ModifierDelayParamList: *const c_void,     // 0x128
            pub _LockHPList: *const NativeList<NativeObject>, // 0x130
            pub _StatusChanceResistanceDict: *const c_void, // 0x138
            pub DisableActionStateByTask__BackingField: *const c_void, // 0x140
            pub _AbilityToSkillMapping: *const c_void,      // 0x148
            pub KillerSkill__BackingField: *const c_void,   // 0x150
            pub DamageDefender: *const GameEntity,          // 0x158
            pub _DamagedAllEntityIDListInAttack: *const c_void, // 0x160
            pub _AbilityPropertiesInitSnapshot: *const NativeArray<FixPoint>, // 0x168
            pub _SyncPropertyMask: *const c_void,           // 0x170
            pub _JsonConfigRef: *const CharacterConfig,     // 0x178
            pub _ModifierRecordList: *const c_void,         // 0x180
            pub _MuteBehaviorFlags: *const c_void,          // 0x188
            pub ResistModifierBehaviorFlags__BackingField: *const NativeList<NativeObject>, // 0x190
            pub _DamageAttacker: *const GameEntity,         // 0x198
            pub _DmgChunk: *const c_void,                   // 0x1a0
            pub ModifierOverrideMapping: *const c_void,     // 0x1a8
            pub _DotModifierEventProcessors: *const NativeList<NativeObject>, // 0x1b0
            pub _OnHitEffectOverride: *const NativeList<NativeObject>, // 0x1b8
            pub _OnHitEffectMultipleOverride: *const NativeList<NativeObject>, // 0x1c0
            pub _ModifierEventProcessors: *const NativeArray<NativeObject>, // 0x1c8
            pub _CharacterDataRef: *const CharacterDataComponent, // 0x1d0
            pub _SelfExtrAbilityList: *const NativeList<NativeString>, // 0x1d8
            pub _RedStanceInfoList: *const NativeList<NativeObject>, // 0x1e0
            pub LastStanceBreakEntity__BackingField: *const GameEntity, // 0x1e8
            pub _RedStanceInfo: *const c_void,              // 0x1f0
            pub DamageSplitData: *const NativeList<NativeObject>, // 0x1f8
            pub _EnergyPointEntries: *const NativeList<NativeObject>, // 0x200
            pub _DebuffLockStepSources: *const NativeList<NativeObject>, // 0x208
            pub _BuffLockStep: i32,                         // 0x210
            pub _ModifierUIOperationIncr: i32,              // 0x214
            pub InheritSPRatio: FixPoint,                   // 0x218
            pub OverflowStanceDamage__BackingField: FixPoint, // 0x220
            pub ActionDelayChanged__BackingField: [u8; 0x2], // 0x228
            pub UseSpecialSP__BackingField: bool,           // 0x22a
            pub LockSelfActionDelay: bool,                  // 0x22b
            pub bIsInCharmAction: bool,                     // 0x22c
            pub BattleTag__BackingField: *const c_void,     // 0x230
            pub ActionDelayDistance__BackingField: FixPoint, // 0x238
            pub _HighestPriorityOnHitEffect: i32,           // 0x240
            pub LastStanceDamageType__BackingField: i32,    // 0x244
            pub IsSnapshot__BackingField: bool,             // 0x248
            pub MuteTriggerDeath__BackingField: bool,       // 0x249
            pub _IsProcessingModifierDelayParam: bool,      // 0x24a
            pub HasRevived: bool,                           // 0x24b
            pub TriggerBreakExtendLogic: bool,              // 0x24c
            pub IsTriggeredBlockDamage: bool,               // 0x24d
            pub ForceKillFlag__BackingField: bool,          // 0x24e
            pub _BreakExtendEventUnsettled: bool,           // 0x24f
            pub _CurrentAttackPhase: i32,                   // 0x250
            pub VisualFlagValue__BackingField: i32,         // 0x254
            pub LastBreakStanceDamageType__BackingField: i32, // 0x258
            pub CurrentAttackType__BackingField: AttackType, // 0x25c
            pub StanceState__BackingField: i32,             // 0x260
            pub PropertyChangeFlag__BackingField: bool,     // 0x264
            pub _IsBehaviorFlagVisualDirty: bool,           // 0x265
            pub IsSharedDamageDataTarget: bool,             // 0x266
            pub IsTriggeringStanceCountDown__BackingField: bool, // 0x267
            pub CharmDamageCount: i32,                      // 0x268
            pub StanceType: i32,                            // 0x26c
            pub SpeedVisualFlagValue__BackingField: i32,    // 0x270
            pub _DeathVersion: u32,                         // 0x274
            pub ForbidVisualFlagValue__BackingField: i32,   // 0x278
            pub PropertyEnumBoundary__BackingField: AbilityProperty, // 0x27c
            pub CharmDisableBPAdd: bool,                    // 0x280
            pub IsInAttack: bool,                           // 0x281
            pub IsForceActionable__BackingField: bool,      // 0x282
            pub BlockModifySp__BackingField: bool,          // 0x283
            pub _ResetStanceVersion: u32,                   // 0x284
            pub _ModifierDelayAddCount: i32,                // 0x288
            pub DeathSource__BackingField: i32,             // 0x28c
            pub CharmDisableSPAdd: bool,                    // 0x290
            pub MuteAllTriggerDeath__BackingField: bool,    // 0x291
            pub InsertAbilityCount: i32,                    // 0x294
            pub _DebuffLockStep: i32,                       // 0x298
            pub ProjectileHitCount: i32,                    // 0x29c
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct EntityManager {
            pub native_object: NativeObject,
            pub _SnapshotEntityMap: *const c_void, // 0x10
            pub PlayerGORoot__BackingField: *const c_void, // 0x18
            pub _OwnerWorldRef: *const GameWorld,  // 0x20
            pub _AllEntityDictionary: *const c_void, // 0x28
            pub DataViewUISelectFadeInFollowEntities__BackingField: *const c_void, // 0x30
            pub DataViewUISelectFadeInEntity__BackingField: *const GameEntity, // 0x38
            pub _ServerEntityIDToEntityDict: *const c_void, // 0x40
            pub _UniqueNamedEntityDictionary: *const c_void, // 0x48
            pub GroupGORoot__BackingField: *const c_void, // 0x50
            pub DataViewUILeaveSummonerOfUncreatedServant__BackingField: *const GameEntity, // 0x58
            pub _PauseEntityTimeSlowIndexDic: *const NativeArray<NativeObject>, // 0x60
            pub _ProcessEntityTeamChangeDelg: *const c_void, // 0x68
            pub _AllTeamEntityList: *const NativeList<GameEntity>, // 0x70
            pub DataViewUISelectFadeOutEntity__BackingField: *const GameEntity, // 0x78
            pub _AllTeamEntity: *const NativeArray<GameEntity>, // 0x80
            pub DataViewUISelectSummonerOfUncreatedServant__BackingField: *const GameEntity, // 0x88
            pub LevelEntity__BackingField: *const GameEntity, // 0x90
            pub _EntityUniqueNameDict: *const NativeList<NativeObject>, // 0x98
            pub EntityGORoot__BackingField: *const c_void, // 0xa0
            pub LittleGameGORoot__BackingField: *const c_void, // 0xa8
            pub DataViewUISelectFadeOutSummonerEntity__BackingField: *const GameEntity, // 0xb0
            pub _GroupEntityIDToEntityDict: *const c_void, // 0xb8
            pub PerformanceGORoot__BackingField: *const c_void, // 0xc0
            pub _UseUniqueSnapshot: bool,          // 0xc8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct CharacterDataComponent {
            pub _parent_object: GameComponentBase, // 0x0
            pub Summoner: *const GameEntity,       // 0x18
            pub _DynamicScaleAdaptConfigs: *const NativeArray<NativeObject>, // 0x20
            pub _DummpyEntityList: *const NativeList<NativeObject>, // 0x28
            pub _DynamicScaleAdaptEffectPathRule: *const c_void, // 0x30
            pub _CharacterUICustomValueDict: *const c_void, // 0x38
            pub _OverrideCharacterConfigParam: [u8; 0x48], // 0x40
            pub HideDisplayInfoSkillNames: *const c_void, // 0x88
            pub JsonConfig__BackingField: *const CharacterConfig, // 0x90
            pub _RowData: *const c_void,           // 0x98
            pub _DynamicScaleAdaptTypes: *const NativeArray<NativeObject>, // 0xa0
            pub LocalOffsetAsMoveTarget__BackingField: [u8; 0xc], // 0xa8
            pub LineupIndex: i32,                  // 0xb4
            pub GridFightTag__BackingField: i32,   // 0xb8
            pub LastActTurnCount__BackingField: u32, // 0xbc
            pub SpawnTurnCount: u32,               // 0xc0
            pub CharacterID__BackingField: u32,    // 0xc4
            pub _SaveModelWhenDeadOverride: [u8; 0x2], // 0xc8
            pub ShowSummonerUI__BackingField: bool, // 0xca
            pub IsVisibleInViewMode__BackingField: bool, // 0xcb
            pub ShowSummonedUI__BackingField: bool, // 0xcc
            pub CreateReason: i32,                 // 0xd0
            pub EnhancedState: i32,                // 0xd4
            pub TriggerLimbo: bool,                // 0xd8
            pub IsBodyPart: bool,                  // 0xd9
            pub DisableRootYawMapping__BackingField: bool, // 0xda
            pub DisableHeadLookAtActionEntityOverride: [u8; 0x0], // 0xdb
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct TurnBasedGameMode {
            pub native_object: NativeObject,
            pub _LastBreakMonster: *const GameEntity, // 0x10
            pub _ActionEntityList: *const NativeList<GameEntity>, // 0x18
            pub _EntityModifierPerforms: *const c_void, // 0x20
            pub _CurrentSkillCharacter: *const SkillCharacterComponent, // 0x28
            pub _SomatoModifierPerforms: *const NativeList<NativeObject>, // 0x30
            pub _ActionEntityListSnapshot: *const NativeList<GameEntity>, // 0x38
            pub _OverrieWaveMonsterPerformDatas: *const NativeList<NativeObject>, // 0x40
            pub _LinkTeammateList: *const NativeList<GameEntity>, // 0x48
            pub _LimboEntities: *const NativeList<NativeObject>, // 0x50
            pub _UnselectableParams: *const NativeList<NativeObject>, // 0x58
            pub StageBattleEventMgr__BackingField: *const c_void, // 0x60
            pub _ActionDelayLinkMgr: *const c_void,   // 0x68
            pub PerformDelayExecuteList: *const NativeList<NativeObject>, // 0x70
            pub _RogueInBattleData: *const c_void,    // 0x78
            pub _CurrentTurnTargetEntity: *const GameEntity, // 0x80
            pub _CurrentActionDelayModifyGroup: *const NativeList<GameEntity>, // 0x88
            pub _VersusBarMgr: *const c_void,         // 0x90
            pub AssistantAvatarEntity__BackingField: *const GameEntity, // 0x98
            pub _ActionDelayRedirectManager: *const c_void, // 0xa0
            pub _InsertAbilityList: *const NativeList<MMNDIEBMDNL>, // 0xa8
            pub PrepareAbility__BackingField: *const c_void, // 0xb0
            pub BattleEventInitedData__BackingField: *const c_void, // 0xb8
            pub _LevelLockedFeatureSet: *const c_void, // 0xc0
            pub _InsertUltraSkillParamsQueue: *const NativeList<NativeObject>, // 0xc8
            pub TurnActionDelayCostChangeSource__BackingField: *const GameEntity, // 0xd0
            pub TimeGameStart: *const c_void,         // 0xd8
            pub LastSummonMonsterList: *const NativeList<GameEntity>, // 0xe0
            pub _LimboEntitiesWaitAbilityFinish: *const NativeList<NativeObject>, // 0xe8
            pub _allowQuitStates: *const NativeList<NativeObject>, // 0xf0
            pub _LimboEntitiesSkipSettlement: *const NativeList<NativeObject>, // 0xf8
            pub _CommonSkillPoolNames: *const c_void, // 0x100
            pub GridFightMananger__BackingField: *const c_void, // 0x108
            pub _CurrentTurnActionEntity: *const GameEntity, // 0x110
            pub _EntityCustomUnselectableDatas: *const NativeList<NativeObject>, // 0x118
            pub _SwordTrainingMgr: *const c_void,     // 0x120
            pub _ModifierPerformCamerContext: *const c_void, // 0x128
            pub _performParam: *const c_void,         // 0x130
            pub BattleCounter: *const c_void,         // 0x138
            pub _CurModifierPerformSeq: *const c_void, // 0x140
            pub ThisTurnAnimEvents: *const c_void,    // 0x148
            pub _EventProcessor: *const c_void,       // 0x150
            pub _MainMonster: *const GameEntity,      // 0x158
            pub _SkillAddBuffPerformList: *const NativeList<NativeObject>, // 0x160
            pub _ReplayData: *const c_void,           // 0x168
            pub CurrentMVPEntity__BackingField: *const GameEntity, // 0x170
            pub _CachedDynamicSkillTargetSelection: *const GameEntity, // 0x178
            pub _AttackingEntityList: *const c_void,  // 0x180
            pub DamageQueue__BackingField: *const c_void, // 0x188
            pub LastKillCaster__BackingField: *const GameEntity, // 0x190
            pub _EvolveBuildGearMgr: *const c_void,   // 0x198
            pub _TurnStateFSM: *const c_void,         // 0x1a0
            pub LastTurnSnapshot: *const c_void,      // 0x1a8
            pub CurrentTurnOwnerEntity__BackingField: *const GameEntity, // 0x1b0
            pub _WaitingAbilityList: *const NativeList<NativeObject>, // 0x1b8
            pub _AllOffTeamCharacters: *const NativeList<GameEntity>, // 0x1c0
            pub _PhaseModifierList: *const NativeList<NativeObject>, // 0x1c8
            pub _ActionDelayOrderTrigger: *const c_void, // 0x1d0
            pub BattleChangeAvatarManager__BackingField: *const c_void, // 0x1d8
            pub _FateBattleManager: *const c_void,    // 0x1e0
            pub BattleAIPublicKnowledge__BackingField: *const c_void, // 0x1e8
            pub LastKillSkill__BackingField: *const c_void, // 0x1f0
            pub ActionBarMgr__BackingField: *const c_void, // 0x1f8
            pub CurrentWaveMainMonsterIDPool__BackingField: *const NativeArray<u32>, // 0x200
            pub OwnerBattleInstanceRef__BackingField: *const BattleInstance, // 0x208
            pub _ActionDelayChangeStamp: [u8; 0x18],  // 0x210
            pub _AllTeamCharacters: *const NativeList<GameEntity>, // 0x228
            pub _LimboRevivableEntities: *const NativeList<NativeObject>, // 0x230
            pub _RelationGroupMgr: *const c_void,     // 0x238
            pub LastKillTargetList__BackingField: *const NativeList<GameEntity>, // 0x240
            pub SkillUsageLog__BackingField: *const c_void, // 0x248
            pub _AidDetail: *const c_void,            // 0x250
            pub _AvatarChangeParam: *const c_void,    // 0x258
            pub LastZombie__BackingField: *const GameEntity, // 0x260
            pub _ImmediateActionEntities: *const c_void, // 0x268
            pub MonsterWaveTextInfo: *const c_void,   // 0x270
            pub PrepareAbilityFinish__BackingField: bool, // 0x278
            pub IsManualExitBattle: bool,             // 0x279
            pub LastKillFinish__BackingField: bool,   // 0x27a
            pub _IsReplayBeingSaved: bool,            // 0x27b
            pub BattleResultAsWin: bool,              // 0x27c
            pub CertainlyLoseInAdvance__BackingField: bool, // 0x27d
            pub TurnOwnerPrepareAbilityUsed__BackingField: bool, // 0x27e
            pub SkipDeathHandle__BackingField: bool,  // 0x27f
            pub ThisTurnAnimEventCount: i32,          // 0x280
            pub _ModifierPerformTimeTotal: f32,       // 0x284
            pub TurnActionDelayCostRatio__BackingField: FixPoint, // 0x288
            pub PropagateBeingAttackTeam__BackingField: TeamType, // 0x290
            pub _SkillExecutionEventState: i32,       // 0x294
            pub ChallengeTurnLimitType__BackingField: i32, // 0x298
            pub _CurrentTurnTeam: TeamType,           // 0x29c
            pub BattleFinishReason: i32,              // 0x2a0
            pub StanceCountDownSPChangeValue__BackingField: f32, // 0x2a4
            pub _WaveMonsterCurrentCount: i32,        // 0x2a8
            pub _LightTeamTurnCount: u32,             // 0x2ac
            pub WaveMonsterMaxCount__BackingField: i32, // 0x2b0
            pub _ChallengeTurnAcc: u32,               // 0x2b4
            pub AddOpCountOnInsertUltraWaitOrder: bool, // 0x2b8
            pub IsTeamFormationExpansion__BackingField: bool, // 0x2b9
            pub _IsUseLinkSkill: bool,                // 0x2ba
            pub IsUseSkillOneMore: bool,              // 0x2bb
            pub _ModifierEndingPerformRemainedTime: f32, // 0x2bc
            pub CurrentModeState__BackingField: i32,  // 0x2c0
            pub _HoldFrameForCapture: u32,            // 0x2c4
            pub RealTimeCounter__BackingField: f32,   // 0x2c8
            pub _HoldFrameForCaptureFlag: bool,       // 0x2cc
            pub LockTeamZOffset__BackingField: bool,  // 0x2cd
            pub LocalWinFlag__BackingField: [u8; 0x2], // 0x2ce
            pub IsManualRetryExitBattle: bool,        // 0x2d0
            pub ApplyUIOperateOnDisableActionFlagChange: bool, // 0x2d1
            pub MuteLastKillTriggered: bool,          // 0x2d2
            pub _GamePauseFlag: bool,                 // 0x2d3
            pub _OverrideAILocked: bool,              // 0x2d4
            pub _OperationCounter: u32,               // 0x2d8
            pub CurrentTurnActionEntityContinuousActionCount__BackingField: u32, // 0x2dc
            pub _RecordOperationByLG: [u8; 0x8],      // 0x2e0
            pub _DeathVersion: u32,                   // 0x2e8
            pub _ModifierPerformTimeScale: f32,       // 0x2ec
            pub ForbidAI: bool,                       // 0x2f0
            pub _LastReplayAutoBattle: bool,          // 0x2f1
            pub _AutoBattle: bool,                    // 0x2f2
            pub ClearUltraSkillEffect: bool,          // 0x2f3
            pub PendingMonsterToWave__BackingField: bool, // 0x2f4
            pub _CurrentTurnActionEntitySkipActionFlag: bool, // 0x2f5
            pub WinFlag: bool,                        // 0x2f6
            pub TurnOwnerActionPhaseEnd__BackingField: bool, // 0x2f7
            pub BattleResultState__BackingField: [u8; 0x8], // 0x2f8
            pub _RecordParamByLG: u32,                // 0x300
            pub PauseState__BackingField: i32,        // 0x304
            pub ApplyUIOperateOnSkillDisableChange: bool, // 0x308
            pub IsLastKillTriggered: bool,            // 0x309
            pub CurrentInsertSkillSkipActionFlag: bool, // 0x30a
            pub _ActionEntityListInited: bool,        // 0x30b
            pub _NextAbilityIndex: i32,               // 0x30c
            pub ClearUltraSkillQueue__BackingField: bool, // 0x310
            pub _IsModifierPerformCameraSet: bool,    // 0x311
            pub _DarkTeamTurnCount: u32,              // 0x314
            pub CurrentWaveIndexInStage__BackingField: u32, // 0x318
            pub CurrentWaveStageID__BackingField: u32, // 0x31c
            pub _CachedDynamicSkillInput: i32,        // 0x320
            pub _HitPerformMinTimer: f32,             // 0x324
            pub _ModifierPerformTimerTotal: f32,      // 0x328
            pub _PrevTickModeState: i32,              // 0x32c
            pub _TurnCounter: u32,                    // 0x330
            pub _RequireMakeLimboEntitiesDie: bool,   // 0x334
            pub SkipTurnOwnerActionFlag__BackingField: bool, // 0x335
            pub CertainlyWinInAdvance__BackingField: bool, // 0x336
            pub TurnEndKeep: bool,                    // 0x337
            pub _DamageCounter: u32,                  // 0x338
            pub ShowCutinUIState__BackingField: i32,  // 0x33c
            pub ElapsedActionDelay__BackingField: FixPoint, // 0x340
            pub UseSkillOneMoreDefaultSkill: i32,     // 0x348
            pub ChallengeTurnLimit__BackingField: u32, // 0x34c
            pub AutoInsertUltraSkill: bool,           // 0x350
            pub IsActionOrder1UsedTBSkill__BackingField: bool, // 0x351
            pub SkipCameraDitherByLastKill: bool,     // 0x352
            pub _IsCreatingNewWave: bool,             // 0x353
            pub _NextModifierIndex: i32,              // 0x354
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct LineUpCharacter {
            pub native_object: NativeObject,
            pub ChangedSkillTreePointList: *const NativeArray<NativeObject>, // 0x10
            pub SkillTreePointList: *const NativeArray<NativeObject>,        // 0x18
            pub BattleGridAvatarData: *const c_void,                         // 0x20
            pub BattleEquipmentList: *const NativeArray<NativeObject>,       // 0x28
            pub SpiritPassiveList: *const NativeArray<u32>,                  // 0x30
            pub BattleRelicItemModule: *const c_void,                        // 0x38
            pub CharacterRank: u32,                                          // 0x40
            pub Index: u32,                                                  // 0x44
            pub CharacterLevel: u32,                                         // 0x48
            pub SpiritLineupType: i32,                                       // 0x4c
            pub WorldLevel: u32,                                             // 0x50
            pub CharacterSP_Numerator: FixPoint,                             // 0x58
            pub AssistUid: u32,                                              // 0x60
            pub TotalPower: u32,                                             // 0x64
            pub CharacterSP_Denominator: FixPoint,                           // 0x68
            pub EnhancedID: u32,                                             // 0x70
            pub CharacterPromotion: u32,                                     // 0x74
            pub CharacterAvatarType: i32,                                    // 0x78
            pub SpecialAvatarID: u32,                                        // 0x7c
            pub CharacterHPRatio: FixPoint,                                  // 0x80
            pub CharacterID: u32,                                            // 0x88
            pub CharacterRowIndex: u32,                                      // 0x8c
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct TeamFormationComponent {
            pub _parent_object: GameComponentBase, // 0x0
            pub WaveOfMonsterRuntimeID__BackingField: *const NativeList<u32>, // 0x18
            pub _Random: *const c_void,            // 0x20
            pub _CurrentLightTeamFightFormationConfig: *const c_void, // 0x28
            pub _LastRefreshLocationTemplateConfigName: *const NativeString, // 0x30
            pub _CurrentDarkTeamFightFormationConfig: *const c_void, // 0x38
            pub _multiRowTargetSelector: *const c_void, // 0x40
            pub _AllTeammateCopy: *const NativeList<GameEntity>, // 0x48
            pub _GridFormationConfig: *const c_void, // 0x50
            pub _CurrentUseTemplateConfigName: *const NativeString, // 0x58
            pub _FormationDatasNormal: *const NativeList<EDJEDBLFIKE>, // 0x60
            pub _FormationInCameraSpace: *const NativeList<GameEntity>, // 0x68
            pub _CustomUpdateFormationDatas: *const NativeList<EDJEDBLFIKE>, // 0x70
            pub _AllTeammate: *const NativeList<GameEntity>, // 0x78
            pub _DefaultLocationExtCfgs2: *const NativeArray<NativeObject>, // 0x80
            pub _TeamFormationDatas: *const NativeList<EDJEDBLFIKE>, // 0x88
            pub _RecordFormationIndices: *const c_void, // 0x90
            pub _CurrentCustomFormationName: *const NativeString, // 0x98
            pub _TeamRefreshLocationContext: *const c_void, // 0xa0
            pub _DefaultFormationCenter: *const EDJEDBLFIKE, // 0xa8
            pub _DefaultLocationExtCfgs: *const NativeArray<NativeObject>, // 0xb0
            pub _UseCustomFormationMembers: *const c_void, // 0xb8
            pub _FormationSnapshots: *const c_void, // 0xc0
            pub LocationRootWorldPos__BackingField: [u8; 0xc], // 0xc8
            pub _IsGridFightTeamFormation: bool,   // 0xd4
            pub _HasCustomTeamFormation: bool,     // 0xd5
            pub _AutoBalanceDefaultFormation: bool, // 0xd6
            pub _IsTeammateDirty: bool,            // 0xd7
            pub _Team: TeamType,                   // 0xd8
            pub _TeamDistanceCache: f32,           // 0xdc
            pub _RemoveVersionCounter: u32,        // 0xe0
            pub _MaxLocationCount: i32,            // 0xe4
            pub _TeamZOffset: f32,                 // 0xe8
            pub _CustomFormationCenter: [u8; 0xc], // 0xec
            pub _dataViewRestoreData: [u8; 0xc],   // 0xf8
            pub _FormationForceWidthMin: f32,      // 0x104
            pub _ServantFormationState: i32,       // 0x108
            pub _LastDieTeammatePos: [u8; 0x8],    // 0x10c
            pub ComfortZoneWidthOverride__BackingField: f32, // 0x114
            pub FormationCenterWorldPosFromBoundBox__BackingField: [u8; 0xc], // 0x118
            pub _gridUnitWidth: f32,               // 0x124
            pub FormationCenterWorldPos__BackingField: [u8; 0xc], // 0x128
            pub AliveEntitiesFormationCenterWorldPos__BackingField: [u8; 0xc], // 0x134
            pub FormationWidth__BackingField: f32, // 0x140
            pub _CurrentFormationType: i32,        // 0x144
            pub _CustomFormationCenterFromBoundBox: [u8; 0xc], // 0x148
            pub _CustomFormationCenterDirty: bool, // 0x154
            pub _IsInited: bool,                   // 0x155
            pub _IsLocationDirty: bool,            // 0x156
            pub TeamFaceDir__BackingField: f32,    // 0x158
            pub LockFormationRefresh__BackingField: bool, // 0x15c
            pub _InverseFlag: bool,                // 0x15d
            pub _TeamActiveRowAliveStateFilter: [u8; 0x2], // 0x15e
            pub _FormationForceWidthMax: f32,      // 0x160
            pub TeamRotation__BackingField: [u8; 0x10], // 0x164
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct FixPoint {
            pub m_rawValue: i64, // 0x0
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct GameWorld {
            pub native_object: NativeObject,
            pub BattleModeRef__BackingField: *const c_void, // 0x10
            pub EntityBeforeDyingCallback: *const c_void,   // 0x18
            pub _LevelAreaManager: *const c_void,           // 0x20
            pub _Modules: *const NativeList<NativeObject>,  // 0x28
            pub BattleInstanceRef__BackingField: *const BattleInstance, // 0x30
            pub _EnterDyingEntityList: *const NativeList<GameEntity>, // 0x38
            pub TimeScaleStack__BackingField: *const c_void, // 0x40
            pub _EntityManager: *const EntityManager,       // 0x48
            pub _NeedTickModules: *const NativeList<NativeObject>, // 0x50
            pub _DeferDeleteEntityList: *const NativeList<GameEntity>, // 0x58
            pub _EventManager: *const c_void,               // 0x60
            pub _EntityTickList: *const c_void,             // 0x68
            pub DamageDataStack__BackingField: *const c_void, // 0x70
            pub EntityWillDestroyCallback: *const c_void,   // 0x78
            pub _GlobalTimeScaleDatas: *const NativeList<NativeObject>, // 0x80
            pub _EntityList: *const NativeList<GameEntity>, // 0x88
            pub _TickedEntityListPerFrame: *const NativeList<GameEntity>, // 0x90
            pub _DyingEntityList: *const NativeList<NativeObject>, // 0x98
            pub MonoEffectManagerRef: *const c_void,        // 0xa0
            pub NewEntityCallback: *const c_void,           // 0xa8
            pub LogicRandom: *const c_void,                 // 0xb0
            pub EntityDiedCallback: *const c_void,          // 0xb8
            pub _NeedLateUpdateModules: *const NativeList<NativeObject>, // 0xc0
            pub _PrefabGameObjectMap: *const c_void,        // 0xc8
            pub _GPTimelineHierarchyManager: *const c_void, // 0xd0
            pub EntityReviveCallback: *const c_void,        // 0xd8
            pub ParamRegister__BackingField: *const c_void, // 0xe0
            pub UnscaledDeltaTime__BackingField: f32,       // 0xe8
            pub _IDFactory: u32,                            // 0xec
            pub _IsPause: bool,                             // 0xf0
            pub IsBattleGameWorld__BackingField: bool,      // 0xf1
            pub _IsDisposing: bool,                         // 0xf2
            pub _IsInTick: bool,                            // 0xf3
            pub _IDFactory_ClientOnly: u32,                 // 0xf4
            pub _BalanceTickDurationTime: f32,              // 0xf8
            pub _BalanceTickDurationBucket: i32,            // 0xfc
            pub _IDFactory_Battle: u32,                     // 0x100
            pub _NextTickDeferDeleteCount: i32,             // 0x104
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct ServantSkillRowData {
            pub native_object: NativeObject,
            pub _Row: *const c_void,              // 0x10
            pub _OverrideData: [u8; 0xf0],        // 0x18
            pub _Config: *const c_void,           // 0x108
            pub _DefaultOverrideData: [u8; 0xe8], // 0x110
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct BattleEventSkillRowData {
            pub native_object: NativeObject,
            pub _Row: *const c_void,              // 0x10
            pub _DefaultOverrideData: [u8; 0xf0], // 0x18
            pub _Config: *const c_void,           // 0x108
            pub _OverrideData: [u8; 0xe8],        // 0x110
        }
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
        pub struct BattleInstance {
            pub native_object: NativeObject,
            pub LogExport__BackingField: *const c_void, // 0x10
            pub _GameWorld: *const GameWorld,           // 0x18
            pub _BattleLineupDataPrimitive: *const BattleLineupData, // 0x20
            pub _BattleLineupData: *const BattleLineupData, // 0x28
            pub _TurnBasedGameMode: *const TurnBasedGameMode, // 0x30
            pub BattleLogicRandom__BackingField: *const c_void, // 0x38
            pub SnapshotHashTurnCount: i32,             // 0x40
            pub BattleUnifiedAreaID__BackingField: u32, // 0x44
            pub ClientIFixVersion: u32,                 // 0x48
            pub BattleAreaID__BackingField: u32,        // 0x4c
            pub BattleID: u32,                          // 0x50
            pub DimensionID__BackingField: u32,         // 0x54
            pub IsBattleDirectKill__BackingField: bool, // 0x58
            pub EncryptTime__BackingField: bool,        // 0x59
            pub PlaneID__BackingField: u32,             // 0x5c
            pub LogicRandomSeed__BackingField: i32,     // 0x60
            pub FloorID__BackingField: u32,             // 0x64
            pub BattleCheckResult: i32,                 // 0x68
            pub BattleAreaGroupID__BackingField: u32,   // 0x6c
            pub AutoBattleAtStart: bool,                // 0x70
            pub IsTeamFormationExpansion__BackingField: bool, // 0x71
            pub ComplexSkillAIConsiderUltra: bool,      // 0x72
            pub IsBattleServerSimulator__BackingField: bool, // 0x73
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct MonsterRowData {
            pub native_object: NativeObject,
            pub _BaseMonsterRow: *const MonsterRow, // 0x10
            pub _OverrideDisplay: [u8; 0x50],       // 0x18
            pub _TemplateRow: *const c_void,        // 0x68
            pub _OverrideStanceWeakList: *const NativeArray<NativeObject>, // 0x70
            pub _EliteGroupRow: *const c_void,      // 0x78
            pub _EliteGroup2Row: *const c_void,     // 0x80
            pub _Row: *const MonsterRow,            // 0x88
            pub _SkillRowDatas: *const NativeList<NativeObject>, // 0x90
            pub _HardLevelRow: *const c_void,       // 0x98
            pub _OverrideStrategies: *const NativeArray<TextID>, // 0xa0
            pub _modelPath: *const NativeString,    // 0xa8
            pub _Json: *const CharacterConfig,      // 0xb0
            pub _MaxHP: FixPoint,                   // 0xb8
            pub _InitStance: [u8; 0x10],            // 0xc0
            pub _MaxStance: [u8; 0x10],             // 0xd0
            pub _InitHP: FixPoint,                  // 0xe0
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
        pub struct GameEntity {
            pub native_object: NativeObject,
            pub _CurTickListRef: [u8; 0x10],            // 0x10
            pub _TickLodProxy: *const c_void,           // 0x20
            pub _UnityGO: *const c_void,                // 0x28
            pub TickLodTemplate: *const NativeString,   // 0x30
            pub TagComponentContainer: *const c_void,   // 0x38
            pub _DestroyWaitList: *const c_void,        // 0x40
            pub OnStageStateChange: *const c_void,      // 0x48
            pub _TickComponentList: *const c_void,      // 0x50
            pub _UnstageReasonKey: *const NativeString, // 0x58
            pub _OwnerWorldRef: *const GameWorld,       // 0x60
            pub WorldTimeScaleAdpator: *const c_void,   // 0x68
            pub NameForGameCore__BackingField: *const NativeString, // 0x70
            pub DisposeCallback: *const c_void,         // 0x78
            pub _ComponentList: *const c_void,          // 0x80
            pub _ComponentArrayRef: *const c_void,      // 0x88
            pub Name__BackingField: *const NativeString, // 0x90
            pub TimeScaleStack: *const c_void,          // 0x98
            pub OnTeamChange: *const c_void,            // 0xa0
            pub HoyoTagContainer: *const c_void,        // 0xa8
            pub _LateUpdateComponentList: *const c_void, // 0xb0
            pub _ComponentArray: *const NativeArray<GameComponentBase>, // 0xb8
            pub IsLoaded__BackingField: bool,           // 0xc0
            pub HasDisposed: bool,                      // 0xc1
            pub _IsRegisterEnviroChara: bool,           // 0xc2
            pub Visible__BackingField: bool,            // 0xc3
            pub _TickDelayFrameCount: u32,              // 0xc4
            pub ForceIgnoreTickLodBistSet: u32,         // 0xc8
            pub _EntityType: EntityType,                // 0xcc
            pub _ServerEntityID: u32,                   // 0xd0
            pub Disposing: bool,                        // 0xd4
            pub IsHero__BackingField: bool,             // 0xd5
            pub _Tickable: bool,                        // 0xd6
            pub _IsOnStage: bool,                       // 0xd7
            pub ObjectFeature__BackingField: i32,       // 0xd8
            pub LastTickTime__BackingField: f32,        // 0xdc
            pub CampID__BackingField: i32,              // 0xe0
            pub _AliveState: i32,                       // 0xe4
            pub LastTickBucket__BackingField: i32,      // 0xe8
            pub _GroupEntityID: u32,                    // 0xec
            pub _GroupID: u32,                          // 0xf0
            pub TickLodBoundSize__BackingField: f32,    // 0xf4
            pub IsFakeAvatar__BackingField: bool,       // 0xf8
            pub IsStoryMode__BackingField: bool,        // 0xf9
            pub KillImmediately: bool,                  // 0xfa
            pub _ShouldLateUpdate: bool,                // 0xfb
            pub RuntimeID__BackingField: u32,           // 0xfc
            pub _ForceTickLodLowestReason: [u8; 0x8],   // 0x100
            pub _Team: TeamType,                        // 0x108
            pub LastTickFrame__BackingField: u64,       // 0x110
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct BattleEventDataComponent {
            pub _parent_object: CharacterDataComponent,          // 0x0
            pub _TBAbilityRef: *const TurnBasedAbilityComponent, // 0xe0
            pub BattleEventConfig__BackingField: *const c_void,  // 0xe8
            pub _EnergyBarState: *const c_void,                  // 0xf0
            pub SourceCaster__BackingField: *const GameEntity,   // 0xf8
            pub _BattleEventRowData: *const c_void,              // 0x100
            pub CreateParams__BackingField: *const c_void,       // 0x108
            pub WarningChallengeTurnLeft: u32,                   // 0x110
            pub BattleEventTotalDamageType: TeamType,            // 0x114
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct BattleLineupData {
            pub native_object: NativeObject,
            pub DeferCreateTrialPlayerDic: *const c_void, // 0x10
            pub ExtraTeam: *const NativeArray<LineUpCharacter>, // 0x18
            pub BattleExtraPropertyAdditionList__BackingField: *const NativeList<NativeObject>, // 0x20
            pub _LevelPath: *const NativeString, // 0x28
            pub LightTeam: *const NativeArray<LineUpCharacter>, // 0x30
            pub SpecialAvatarLevelAreaConfigs: *const c_void, // 0x38
            pub _TemplateVariables: *const c_void, // 0x40
            pub TeamBuffIDList: *const NativeArray<u32>, // 0x48
            pub Context: *const c_void,          // 0x50
            pub MazeBuffAdded: *const NativeList<NativeObject>, // 0x58
            pub AdditionalTemplateVariables: *const c_void, // 0x60
            pub WorldLevel: u32,                 // 0x68
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
        pub struct GameComponentBase {
            pub native_object: NativeObject,
            pub _OwnerRef: *const GameEntity, // 0x10
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct MonsterRow {
            pub native_object: NativeObject,
            pub CustomValueTags: *const NativeArray<NativeString>, // 0x10
            pub DebuffResist: *const NativeArray<NativeObject>,    // 0x18
            pub DynamicValues: *const NativeArray<NativeObject>,   // 0x20
            pub SkillList: *const NativeArray<u32>,                // 0x28
            pub AbilityNameList: *const NativeArray<NativeString>, // 0x30
            pub OverrideAIPath: *const NativeString,               // 0x38
            pub _ExpandedDataRowCache: *const c_void,              // 0x40
            pub MonsterStrategy: *const NativeArray<TextID>,       // 0x48
            pub OverrideSkillParams: *const NativeArray<NativeObject>, // 0x50
            pub StanceWeakList: *const NativeArray<NativeObject>,  // 0x58
            pub SummonIDList: *const NativeArray<u32>,             // 0x60
            pub OverrideAISkillSequence: *const NativeArray<NativeObject>, // 0x68
            pub DamageTypeResistance: *const NativeArray<NativeObject>, // 0x70
            pub CustomValues: *const NativeArray<NativeObject>,    // 0x78
            pub MonsterIntroduction: TextID,                       // 0x80
            pub HPModifyRatio: FixPoint,                           // 0x90
            pub MonsterBattleIntroduction: TextID,                 // 0x98
            pub _IsCached: bool,                                   // 0xa8
            pub InheritSummonIDList: bool,                         // 0xa9
            pub InheritAbilityNameList: bool,                      // 0xaa
            pub InheritDynamicValues: bool,                        // 0xab
            pub MonsterID: u32,                                    // 0xac
            pub Level: u32,                                        // 0xb0
            pub SpeedModifyRatio: FixPoint,                        // 0xb8
            pub SpeedModifyValue: FixPoint,                        // 0xc0
            pub EliteGroup: u32,                                   // 0xc8
            pub HardLevelGroup: u32,                               // 0xcc
            pub DefenceModifyRatio: FixPoint,                      // 0xd0
            pub InheritCustomValueTags: bool,                      // 0xd8
            pub InheritCustomValues: bool,                         // 0xd9
            pub StanceCountDelta: i32,                             // 0xdc
            pub HPModifyValue: FixPoint,                           // 0xe0
            pub AttackModifyRatio: FixPoint,                       // 0xe8
            pub StanceModifyRatio: FixPoint,                       // 0xf0
            pub AttackModifyValue: FixPoint,                       // 0xf8
            pub MonsterName: TextID,                               // 0x100
            pub MonsterTemplateID: u32,                            // 0x110
            pub MonsterType: u32,                                  // 0x114
            pub StanceModifyValue: FixPoint,                       // 0x118
            pub DefenceModifyValue: FixPoint,                      // 0x120
        }
    }
}
