use std::ffi::c_void;
use rpg::gamecore::{FixPoint, GameEntity};

use super::il2cpp_types::{
    Il2CppArray,
    Il2CppObject,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NOPBAAAGGLA {
    pub il2cpp_object: Il2CppObject,
    pub MDEHKOOKJCK: *const Il2CppArray<Il2CppObject>, // 0x10
    pub HKFGOHGKOGK: *const c_void,                    // 0x18
    pub PBHCGDFPEED: *const c_void,                    // 0x20
    pub FKKDFMPMJHG: *const Il2CppArray<Il2CppObject>, // 0x28
    pub LGGEDDMACDF: *const c_void,                    // 0x30
    pub AAHMMHBHMFN: [u8; 0x90],                       // 0x38
    pub JKCOIOLCMEP: *const c_void,                    // 0xc8
    pub KNDJNKNHFFG: *const c_void,                    // 0xd0
    pub BEAJGANIDLJ: *const c_void,                    // 0xd8
    pub JODAJBNCCNP: *const Il2CppArray<Il2CppObject>, // 0xe0
    pub CMNBOEIDAOD: FixPoint,                         // 0xe8
    pub NHHNLMOBEGH: FixPoint,                         // 0xf0
    pub DKOIGIHEBCD: FixPoint,                         // 0xf8
    pub DEOICHHPAIF: FixPoint,                         // 0x100
    pub POLANGDKOKH: FixPoint,                         // 0x108
    pub FMMBMJKNAHI: FixPoint,                         // 0x110
    pub PJNEJPNBNMP: FixPoint,                         // 0x118
    pub GJNAGCJONAO: FixPoint,                         // 0x120
    pub HHEIPBOKCOH: [u8; 0x40],                       // 0x128
    pub MGFECPHDPHB: FixPoint,                         // 0x168
    pub COIDNPMCCFG: FixPoint,                         // 0x170
    pub GHBPOPKEGLE: FixPoint,                         // 0x178
    pub CCLFKIPGJOG: FixPoint,                         // 0x180
    pub MAKENPDPHDN: FixPoint,                         // 0x188
    pub BLFCEOMPDKK: FixPoint,                         // 0x190
    pub GLPLDJKMOBE: FixPoint,                         // 0x198
    pub FNDCNMHMCIC: FixPoint,                         // 0x1a0
    pub GAALBDHLFOG: FixPoint,                         // 0x1a8
    pub FOLCDHNIMMI: FixPoint,                         // 0x1b0
    pub ANHNDBECCJD: [u8; 0x40],                       // 0x1b8
    pub HMMMDOHLFEP: FixPoint,                         // 0x1f8
    pub NCOHIAPKAED: FixPoint,                         // 0x200
    pub EPJEDLOBFPG: FixPoint,                         // 0x208
    pub DINCHAHPEAC: FixPoint,                         // 0x210
    pub PDCMJAMPJNL: FixPoint,                         // 0x218
    pub DJHDAOOEJOF: FixPoint,                         // 0x220
    pub NEPGNKOMAAA: FixPoint,                         // 0x228
    pub IAAJMHADJDG: FixPoint,                         // 0x230
    pub FGIPOLJPICM: FixPoint,                         // 0x238
    pub GBENLNNEIJM: FixPoint,                         // 0x240
    pub BBNMJNPDOCP: FixPoint,                         // 0x248
    pub CAILJEGIDKL: FixPoint,                         // 0x250
    pub LJGPDLDGCEO: FixPoint,                         // 0x258
    pub JCPEINMPKAM: FixPoint,                         // 0x260
    pub GLGFEKEMMJJ: FixPoint,                         // 0x268
    pub DBNKBGKCMKH: FixPoint,                         // 0x270
    pub ILNAKPIOOAK: FixPoint,                         // 0x278
    pub ENFFBMJBEDP: FixPoint,                         // 0x280
    pub KPELFJICFDH: FixPoint,                         // 0x288
    pub GCFCCDPIACO: FixPoint,                         // 0x290
    pub PAIGBKBOKDI: FixPoint,                         // 0x298
    pub BDLFBDLDEND: [u8; 0x48],                       // 0x2a0
    pub DBBDIMCJIKE: FixPoint,                         // 0x2e8
    pub IICNDPJGCFA: [u8; 0x4],                        // 0x2f0
    pub IJJHMGEHMHB: bool,                             // 0x2f4
    pub DPEJKHJPLAC: bool,                             // 0x2f5
    pub FNBALMGFGDM: bool,                             // 0x2f6
    pub GOHOJAIMDNM: FixPoint,                         // 0x2f8
    pub JEHMOKDJDDE: FixPoint,                         // 0x300
    pub HCGBHCPHDKJ: FixPoint,                         // 0x308
    pub MJMDGNPPILN: FixPoint,                         // 0x310
    pub KDJBABPDHEG: FixPoint,                         // 0x318
    pub KNMEDDOAGEK: [u8; 0x20],                       // 0x320
    pub NAGMKEABGEE: FixPoint,                         // 0x340
    pub ODBPMMGBKGA: FixPoint,                         // 0x348
    pub BDGDFKGOLPJ: FixPoint,                         // 0x350
    pub BGBOFNMKDNJ: FixPoint,                         // 0x358
    pub JHOHCEFOJNB: FixPoint,                         // 0x360
    pub EBDJHPNOALL: FixPoint,                         // 0x368
    pub FLMEBELNIKK: FixPoint,                         // 0x370
    pub JFKEEOMKMLI: FixPoint,                         // 0x378
    pub GNMAKKBFOCH: FixPoint,                         // 0x380
    pub ELGMFJLGCPH: FixPoint,                         // 0x388
    pub OHBMMFAFMDP: FixPoint,                         // 0x390
    pub APDDLHNGGIM: [u8; 0x4],                        // 0x398
    pub KMIKODLPNGL: [u8; 0x4],                        // 0x39c
    pub BEGDMOGLLGM: FixPoint,                         // 0x3a0
    pub AMAJNHHAJIM: FixPoint,                         // 0x3a8
    pub BKIFAEKCIHN: FixPoint,                         // 0x3b0
    pub DPPDEDGCLJJ: FixPoint,                         // 0x3b8
    pub KODEDHBLGGH: FixPoint,                         // 0x3c0
    pub GCNOMMHFPOG: FixPoint,                         // 0x3c8
    pub EJJMIFKCFHP: bool,                             // 0x3d0
    pub HEMFDDDJOGK: bool,                             // 0x3d1
    pub EKBHFCODKFO: bool,                             // 0x3d2
    pub BBDANLEJCIA: bool,                             // 0x3d3
    pub GCGEEFLGCIG: [u8; 0x4],                        // 0x3d4
    pub OEPAPFDLMML: FixPoint,                         // 0x3d8
    pub COKMLMJPKLH: u32,                              // 0x3e0
    pub MKIMEBNOEGI: FixPoint,                         // 0x3e8
    pub AHHEDGLMDMG: [u8; 0x4],                        // 0x3f0
    pub GFFCEBJGABG: bool,                             // 0x3f4
    pub JICCOEHBPJJ: bool,                             // 0x3f5
    pub EGINKGPDNPK: bool,                             // 0x3f6
    pub CAANBNCPACE: bool,                             // 0x3f7
    pub ABIPIIBIIBE: FixPoint,                         // 0x3f8
    pub GMBACFCLEGD: FixPoint,                         // 0x400
    pub JNFPCNAKNOP: FixPoint,                         // 0x408
    pub DCEBGGFOFAO: FixPoint,                         // 0x410
    pub BJAEJMLMJCL: FixPoint,                         // 0x418
    pub DGFBMAPFPNH: FixPoint,                         // 0x420
    pub MLKFKKACBCE: FixPoint,                         // 0x428
    pub MNGPDEOEHPE: FixPoint,                         // 0x430
    pub JGHJIGOCPNP: [u8; 0x4],                        // 0x438
    pub OJGNIBKADHK: u32,                              // 0x43c
    pub PNGJIDMHIOE: FixPoint,                         // 0x440
    pub EFFODBPOOCN: FixPoint,                         // 0x448
    pub CFBOJBAJCEA: [u8; 0x4],                        // 0x450
    pub HKNLHAMMIIM: bool,                             // 0x454
    pub KBKGNDFAKGD: bool,                             // 0x455
    pub KDCHAHHPPGD: bool,                             // 0x456
    pub AHPFPMEGEKG: bool,                             // 0x457
    pub DJCAFPFKOGP: FixPoint,                         // 0x458
    pub MHEBPGAHFCB: FixPoint,                         // 0x460
    pub FFFOLNDHIEH: [u8; 0x48],                       // 0x468
    pub PJLPGAGKIDE: FixPoint,                         // 0x4b0
    pub HNJBAFCNNDD: FixPoint,                         // 0x4b8
    pub KOCOLHHLFLD: [u8; 0x40],                       // 0x4c0
    pub JIINJMJGCOH: FixPoint,                         // 0x500
    pub EFAAJEAENFF: FixPoint,                         // 0x508
    pub FFCGIMAMDPP: FixPoint,                         // 0x510
    pub JFMADBFKBDK: FixPoint,                         // 0x518
    pub CGMHNNNOKAI: FixPoint,                         // 0x520
    pub KOEGLFLGADD: FixPoint,                         // 0x528
    pub KLMAGCLFBAO: FixPoint,                         // 0x530
    pub PGGOANFBJON: FixPoint,                         // 0x538
    pub MKNDMBOCCBO: FixPoint,                         // 0x540
    pub ACDFHOGEMCC: [u8; 0x40],                       // 0x548
    pub EBDJIHNKAOC: FixPoint,                         // 0x588
    pub ALOGNJIBIPG: FixPoint,                         // 0x590
    pub PGOHAIPOCNK: FixPoint,                         // 0x598
    pub GIHPOCDLJOA: FixPoint,                         // 0x5a0
    pub PJPKDAKBEJI: FixPoint,                         // 0x5a8
    pub AHOCGHANMCE: FixPoint,                         // 0x5b0
    pub CINNHMENLIJ: FixPoint,                         // 0x5b8
    pub HJAEPANAFLN: FixPoint,                         // 0x5c0
    pub APDLLHIMMEM: FixPoint,                         // 0x5c8
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HBIAGLPHICO {
    pub il2cpp_object: Il2CppObject,
    pub JKCOIOLCMEP: *const GameEntity, // 0x10
    pub LNIIFEIJANE: *const c_void,     // 0x18
    pub KNDJNKNHFFG: *const GameEntity, // 0x20
    pub KJDOLBOBKJF: *const c_void,     // 0x28
}
pub mod rpg {
    pub mod client {
        use std::ffi::c_void;

        use crate::sr::il2cpp_types::{
            Il2CppArray,
            Il2CppObject,
        };

        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct ModuleManager {
            pub il2cpp_object: Il2CppObject,
            pub ChimeraModule: *const c_void,                             // 0x10
            pub SystemOpenModule: *const c_void,                          // 0x18
            pub TransferModule: *const c_void,                            // 0x20
            pub AntiAddictionModule: *const c_void,                       // 0x28
            pub MonopolyModule: *const c_void,                            // 0x30
            pub RelicModule: *const c_void,                               // 0x38
            pub OfferingModule: *const c_void,                            // 0x40
            pub TrainPartyModule: *const c_void,                          // 0x48
            pub ActivityTelevisionModule: *const c_void,                  // 0x50
            pub AnniversaryCollectionModule: *const c_void,               // 0x58
            pub MultiFloorConflictModule: *const c_void,                  // 0x60
            pub FantasticStoryActivityModule: *const c_void,              // 0x68
            pub WolfBroShootingModule: *const c_void,                     // 0x70
            pub PunkLordModule: *const c_void,                            // 0x78
            pub FeatureSwitchModule: *const c_void,                       // 0x80
            pub GameStateServiceModule: *const c_void,                    // 0x88
            pub WhiteListInteractUploadModule: *const c_void,             // 0x90
            pub ActivityQuestTimeLimitModule: *const c_void,              // 0x98
            pub FightActivityModule: *const c_void,                       // 0xa0
            pub RaidCollectionModule: *const c_void,                      // 0xa8
            pub MissionChronicleModule: *const c_void,                    // 0xb0
            pub BattleTipsModule: *const c_void,                          // 0xb8
            pub ActivityMusicRhythmModule: *const c_void,                 // 0xc0
            pub ActivityPlayerReturnModule: *const c_void,                // 0xc8
            pub ActivitySummonModule: *const c_void,                      // 0xd0
            pub AnniversaryAvatarDeliverModule: *const c_void,            // 0xd8
            pub TextJoinModule: *const c_void,                            // 0xe0
            pub QuestModule: *const c_void,                               // 0xe8
            pub PingPongModule: *const c_void,                            // 0xf0
            pub TitanAtlasModule: *const c_void,                          // 0xf8
            pub TarotBookModule: *const c_void,                           // 0x100
            pub AlleyModule: *const c_void,                               // 0x108
            pub StoryLineModule: *const c_void,                           // 0x110
            pub HandbookModule: *const c_void,                            // 0x118
            pub SilverWolfModule: *const c_void,                          // 0x120
            pub NovelModule: *const c_void,                               // 0x128
            pub modules: *const Il2CppArray<Il2CppObject>,                // 0x130
            pub LoginModule: *const c_void,                               // 0x138
            pub PamModule: *const c_void,                                 // 0x140
            pub MultipleDropModule: *const c_void,                        // 0x148
            pub ActivityBenefitModule: *const c_void,                     // 0x150
            pub _ModuleInitRequestList: *const Il2CppArray<Il2CppObject>, // 0x158
            pub ServerPrefsModule: *const c_void,                         // 0x160
            pub MapConnectivityModule: *const c_void,                     // 0x168
            pub ArchiveModule: *const c_void,                             // 0x170
            pub ActivityStrongChallengeModule: *const c_void,             // 0x178
            pub DifficultyAdjustModule: *const c_void,                    // 0x180
            pub RogueAdventureModule: *const c_void,                      // 0x188
            pub GrowthModule: *const c_void,                              // 0x190
            pub ActivityAetherDivideModule: *const c_void,                // 0x198
            pub RogueTournModule: *const c_void,                          // 0x1a0
            pub HeliobusModule: *const c_void,                            // 0x1a8
            pub WorldShop4ThModule: *const c_void,                        // 0x1b0
            pub ActivityTrackPhotoModule: *const c_void,                  // 0x1b8
            pub ChallengeModule: *const c_void,                           // 0x1c0
            pub StarFightModule: *const c_void,                           // 0x1c8
            pub PersonalizeModule: *const c_void,                         // 0x1d0
            pub SpaceZooModule: *const c_void,                            // 0x1d8
            pub ActivityGuessTheSilhouetteModule: *const c_void,          // 0x1e0
            pub EvolveBuildModule: *const c_void,                         // 0x1e8
            pub MissionTimelineModule: *const c_void,                     // 0x1f0
            pub TrainModule: *const c_void,                               // 0x1f8
            pub MusicAlbumModule: *const c_void,                          // 0x200
            pub GamePlayLockModule: *const c_void,                        // 0x208
            pub AchievementModule: *const c_void,                         // 0x210
            pub FightFestModule: *const c_void,                           // 0x218
            pub ChatModule: *const c_void,                                // 0x220
            pub GridFightModule: *const c_void,                           // 0x228
            pub ActivityClockParkModule: *const c_void,                   // 0x230
            pub PerformanceRecallModule: *const c_void,                   // 0x238
            pub GachaModule: *const c_void,                               // 0x240
            pub ActivityPhotoExhibitionModule: *const c_void,             // 0x248
            pub PlayerModule: *const c_void,                              // 0x250
            pub TalkModule: *const c_void,                                // 0x258
            pub LobbyModule: *const c_void,                               // 0x260
            pub BattleCollegeModule: *const c_void,                       // 0x268
            pub DrinkMakerModule: *const c_void,                          // 0x270
            pub TreasureDungeonModule: *const c_void,                     // 0x278
            pub RecommendModule: *const c_void,                           // 0x280
            pub TeamModule: *const c_void,                                // 0x288
            pub MultiplayerGameModule: *const c_void,                     // 0x290
            pub NavMapModule: *const c_void,                              // 0x298
            pub ChessRogueModule: *const c_void,                          // 0x2a0
            pub CatchGhostModule: *const c_void,                          // 0x2a8
            pub MovieRacingModule: *const c_void,                         // 0x2b0
            pub MissionModule: *const c_void,                             // 0x2b8
            pub MapRotationModule: *const c_void,                         // 0x2c0
            pub ShareModule: *const c_void,                               // 0x2c8
            pub FormationMoveModule: *const c_void,                       // 0x2d0
            pub BattlePassModule: *const c_void,                          // 0x2d8
            pub FarmModule: *const c_void,                                // 0x2e0
            pub AvatarModule: *const c_void,                              // 0x2e8
            pub RogueArcadeModule: *const c_void,                         // 0x2f0
            pub ActivityModule: *const c_void,                            // 0x2f8
            pub MessageModule: *const c_void,                             // 0x300
            pub AetherDivideModule: *const c_void,                        // 0x308
            pub OperationModule: *const c_void,                           // 0x310
            pub ToastQueueModule: *const c_void,                          // 0x318
            pub PlanetFesModule: *const c_void,                           // 0x320
            pub SwitchHandModule: *const c_void,                          // 0x328
            pub BoxingClubModule: *const c_void,                          // 0x330
            pub RaidModule: *const c_void,                                // 0x338
            pub ScheduleModule: *const c_void,                            // 0x340
            pub FloorConnectivityModule: *const c_void,                   // 0x348
            pub CumulativeConsumptionModule: *const c_void,               // 0x350
            pub PayModule: *const c_void,                                 // 0x358
            pub MultiPathAvatarModule: *const c_void,                     // 0x360
            pub PamSkinModule: *const c_void,                             // 0x368
            pub TravelBrochureModule: *const c_void,                      // 0x370
            pub FriendModule: *const c_void,                              // 0x378
            pub FindChestModule: *const c_void,                           // 0x380
            pub HeartDialModule: *const c_void,                           // 0x388
            pub RogueMagicModule: *const c_void,                          // 0x390
            pub RogueHandbookModule: *const c_void,                       // 0x398
            pub FiveDimModule: *const c_void,                             // 0x3a0
            pub BattleEventModule: *const c_void,                         // 0x3a8
            pub RechargeShopModule: *const c_void,                        // 0x3b0
            pub PetModule: *const c_void,                                 // 0x3b8
            pub EntityScoreModule: *const c_void,                         // 0x3c0
            pub ActivityFeverTimeModule: *const c_void,                   // 0x3c8
            pub RoleTrialModule: *const c_void,                           // 0x3d0
            pub ExpeditionModule: *const c_void,                          // 0x3d8
            pub StoryTokenModule: *const c_void,                          // 0x3e0
            pub TutorialSupportModule: *const c_void,                     // 0x3e8
            pub CompanionMissionActivityModule: *const c_void,            // 0x3f0
            pub MapPropOverrideConditionModule: *const c_void,            // 0x3f8
            pub ShopModule: *const c_void,                                // 0x400
            pub InventoryModule: *const c_void,                           // 0x408
            pub LuaDataModule: *const c_void,                             // 0x410
            pub LoadingTipsModule: *const c_void,                         // 0x418
            pub EarlyAccessModule: *const c_void,                         // 0x420
            pub BattleModule: *const c_void,                              // 0x428
            pub ItemComposeModule: *const c_void,                         // 0x430
            pub RogueModule: *const c_void,                               // 0x438
            pub BigMapModule: *const c_void,                              // 0x440
            pub RollShopModule: *const c_void,                            // 0x448
            pub ActivitySwordTrainingModule: *const c_void,               // 0x450
            pub DialogueModule: *const c_void,                            // 0x458
            pub EntityTimeRewindModule: *const c_void,                    // 0x460
            pub MuseumModule: *const c_void,                              // 0x468
            pub MatchThreeModule: *const c_void,                          // 0x470
            pub PhotoGraphModule: *const c_void,                          // 0x478
            pub MarbleModule: *const c_void,                              // 0x480
            pub AdventureModule: *const c_void,                           // 0x488
            pub EraFlipperModule: *const c_void,                          // 0x490
            pub MaterialSubmissionModule: *const c_void,                  // 0x498
            pub isInited: bool,                                           // 0x4a0
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct AvatarServantData {
            pub il2cpp_object: Il2CppObject,
            pub _SkillDataMap: *const c_void,   // 0x10
            pub _ServantRowData: *const c_void, // 0x18
            pub _Row: *const c_void,            // 0x20
            pub _AvatarData: *const AvatarData, // 0x28
            pub _Json: *const c_void,           // 0x30
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct AvatarData {
            pub il2cpp_object: Il2CppObject,
            pub PromotedBeforeData__BackingField: *const c_void,     // 0x10
            pub GrowUpBeforeData__BackingField: *const c_void,       // 0x18
            pub UltraSkillConfig__BackingField: *const c_void,       // 0x20
            pub _SkillDataMap: *const c_void,                        // 0x28
            pub Row__BackingField: *const c_void,                    // 0x30
            pub CombatPowerData__BackingField: *const c_void,        // 0x38
            pub _ExtraPropertyAddition: *const c_void,               // 0x40
            pub AvatarPropertyData__BackingField: *const c_void,     // 0x48
            pub RelicsData__BackingField: *const c_void,             // 0x50
            pub ServantData__BackingField: *const AvatarServantData, // 0x58
            pub _AvatarName: *const c_void,                          // 0x60
            pub _SkinIDList: u32,                                    // 0x68
            pub LevelUpedBeforeData__BackingField: *const c_void,    // 0x70
            pub SpecialRow__BackingField: *const c_void,             // 0x78
            pub _AvatarRowData: *const c_void,                       // 0x80
            pub _TrialEquipment: *const c_void,                      // 0x88
            pub SkillTreeData: *const c_void,                        // 0x90
            pub HasTakenPromotionRewardList__BackingField: u32,      // 0x98
            pub Level__BackingField: u32,                            // 0xa0
            pub RealID__BackingField: u32,                           // 0xa4
            pub EquipmentUID__BackingField: u32,                     // 0xa8
            pub Promotion__BackingField: u32,                        // 0xac
            pub _BaseID: u32,                                        // 0xb0
            pub Rank__BackingField: u32,                             // 0xb4
            pub _AdventurePlayerID: u32,                             // 0xb8
            pub AvatarType__BackingField: [u8; 0x4],                 // 0xbc
            pub DressedSkinID__BackingField: u32,                    // 0xc0
            pub IsNew__BackingField: bool,                           // 0xc4
            pub IsMarked__BackingField: bool,                        // 0xc5
            pub IsDisplayOnly__BackingField: bool,                   // 0xc6
            pub CurrentExp__BackingField: u32,                       // 0xc8
            pub SpecialAvatarID__BackingField: u32,                  // 0xcc
            pub FirstMetTimeStamp: u64,                              // 0xd0
        }
    }
    pub mod gamecore {
        use std::ffi::c_void;

        use crate::sr::{il2cpp_types::{
            Il2CppArray,
            Il2CppObject,
        }, types::HBIAGLPHICO};

        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
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
        #[derive(Debug, Clone, Copy)]
        pub struct LineUpCharacter {
            pub il2cpp_object: Il2CppObject,
            pub BattleRelicItemModule: *const c_void,                        // 0x10
            pub BattleEquipmentList: *const Il2CppArray<Il2CppObject>,       // 0x18
            pub SkillTreePointList: *const Il2CppArray<Il2CppObject>,        // 0x20
            pub SpiritPassiveList: u32,                                      // 0x28
            pub ChangedSkillTreePointList: *const Il2CppArray<Il2CppObject>, // 0x30
            pub BattleGridAvatarData: *const c_void,                         // 0x38
            pub SpecialAvatarID: u32,                                        // 0x40
            pub CharacterRowIndex: u32,                                      // 0x44
            pub CharacterSP_Denominator: FixPoint,                           // 0x48
            pub SpiritLineupType: [u8; 0x4],                                 // 0x50
            pub CharacterAvatarType: [u8; 0x4],                              // 0x54
            pub CharacterPromotion: u32,                                     // 0x58
            pub CharacterRank: u32,                                          // 0x5c
            pub CharacterHPRatio: FixPoint,                                  // 0x60
            pub AssistUid: u32,                                              // 0x68
            pub Index: u32,                                                  // 0x6c
            pub CharacterLevel: u32,                                         // 0x70
            pub TotalPower: u32,                                             // 0x74
            pub CharacterSP_Numerator: FixPoint,                             // 0x78
            pub CharacterID: u32,                                            // 0x80
            pub WorldLevel: u32,                                             // 0x84
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct FixPoint {
            pub m_rawValue: i64, // 0x0
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct BattleLineupData {
            pub il2cpp_object: Il2CppObject,
            pub Context: *const c_void,                                       // 0x10
            pub DeferCreateTrialPlayerDic: *const c_void,                     // 0x18
            pub BattleExtraPropertyAdditionDict__BackingField: *const c_void, // 0x20
            pub LightTeam: *const Il2CppArray<LineUpCharacter>,               // 0x28
            pub MazeBuffAdded: *const Il2CppArray<Il2CppObject>,              // 0x30
            pub _TemplateVariables: *const c_void,                            // 0x38
            pub AdditionalTemplateVariables: *const c_void,                   // 0x40
            pub ExtraTeam: *const Il2CppArray<LineUpCharacter>,               // 0x48
            pub SpecialAvatarLevelAreaConfigs: *const c_void,                 // 0x50
            pub _LevelPath: *const c_void,                                    // 0x58
            pub TeamBuffIDList: u32,                                          // 0x60
            pub WorldLevel: u32,                                              // 0x68
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct EntityManager {
            pub il2cpp_object: Il2CppObject,
            pub _PauseEntityTimeSlowIndexDic: *const Il2CppArray<Il2CppObject>, // 0x10
            pub _SnapshotEntityMap: *const c_void,                              // 0x18
            pub LevelEntity__BackingField: *const GameEntity,                   // 0x20
            pub _EntityUniqueNameDict: *const Il2CppArray<Il2CppObject>,        // 0x28
            pub _AllTeamEntityList: *const Il2CppArray<GameEntity>,             // 0x30
            pub EntityGORoot__BackingField: *const c_void,                      // 0x38
            pub DataViewUILeaveSummonerOfUncreatedServant__BackingField: *const GameEntity, // 0x40
            pub GroupGORoot__BackingField: *const c_void,                       // 0x48
            pub LittleGameGORoot__BackingField: *const c_void,                  // 0x50
            pub PerformanceGORoot__BackingField: *const c_void,                 // 0x58
            pub PlayerGORoot__BackingField: *const c_void,                      // 0x60
            pub DataViewUISelectFadeOutSummonerEntity__BackingField: *const GameEntity, // 0x68
            pub DataViewUISelectSummonerOfUncreatedServant__BackingField: *const GameEntity, // 0x70
            pub DataViewUISelectFadeInEntity__BackingField: *const GameEntity,  // 0x78
            pub _AllTeamEntity: *const Il2CppArray<GameEntity>,                 // 0x80
            pub _UniqueNamedEntityDictionary: *const c_void,                    // 0x88
            pub _ServerEntityIDToEntityDict: *const c_void,                     // 0x90
            pub _GroupEntityIDToEntityDict: *const c_void,                      // 0x98
            pub _ProcessEntityTeamChangeDelg: *const c_void,                    // 0xa0
            pub _OwnerWorldRef: *const c_void,                                  // 0xa8
            pub DataViewUISelectFadeOutEntity__BackingField: *const GameEntity, // 0xb0
            pub DataViewUISelectFadeInFollowEntities__BackingField: *const c_void, // 0xb8
            pub _AllEntityDictionary: *const c_void,                            // 0xc0
            pub _UseUniqueSnapshot: bool,                                       // 0xc8
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct GameEntity {
            pub il2cpp_object: Il2CppObject,
            pub _TickComponentList: *const c_void,                 // 0x10
            pub _ComponentArrayRef: *const c_void,                 // 0x18
            pub _UnityGO: *const c_void,                           // 0x20
            pub _CurTickListRef: [u8; 0x10],                       // 0x28
            pub HoyoTagContainer: *const c_void,                   // 0x38
            pub _UnstageReasonKey: *const c_void,                  // 0x40
            pub TimeScaleStack: *const c_void,                     // 0x48
            pub _LateUpdateComponentList: *const c_void,           // 0x50
            pub TickLodTemplate: *const c_void,                    // 0x58
            pub TagComponentContainer: *const c_void,              // 0x60
            pub _TickLodProxy: *const c_void,                      // 0x68
            pub OnTeamChange: *const c_void,                       // 0x70
            pub _OwnerWorldRef: *const c_void,                     // 0x78
            pub _ComponentArray: *const Il2CppArray<Il2CppObject>, // 0x80
            pub WorldTimeScaleAdpator: *const c_void,              // 0x88
            pub OnStageStateChange: *const c_void,                 // 0x90
            pub _DestroyWaitList: *const c_void,                   // 0x98
            pub Name__BackingField: *const c_void,                 // 0xa0
            pub NameForGameCore__BackingField: *const c_void,      // 0xa8
            pub DisposeCallback: *const c_void,                    // 0xb0
            pub _ComponentList: *const c_void,                     // 0xb8
            pub Disposing: bool,                                   // 0xc0
            pub IsHero__BackingField: bool,                        // 0xc1
            pub KillImmediately: bool,                             // 0xc2
            pub IsFakeAvatar__BackingField: bool,                  // 0xc3
            pub _IsOnStage: bool,                                  // 0xc4
            pub _Tickable: bool,                                   // 0xc5
            pub Visible__BackingField: bool,                       // 0xc6
            pub _ShouldLateUpdate: bool,                           // 0xc7
            pub LastTickFrame__BackingField: u64,                  // 0xc8
            pub LastTickTime__BackingField: f32,                   // 0xd0
            pub _AliveState: [u8; 0x4],                            // 0xd4
            pub _ForceTickLodLowestReason: *const c_void,          // 0xd8
            pub ForceIgnoreTickLodBistSet: u32,                    // 0xe0
            pub HasDisposed: bool,                                 // 0xe4
            pub _IsRegisterEnviroChara: bool,                      // 0xe5
            pub IsLoaded__BackingField: bool,                      // 0xe6
            pub IsStoryMode__BackingField: bool,                   // 0xe7
            pub TickLodBoundSize__BackingField: f32,               // 0xe8
            pub _GroupEntityID: u32,                               // 0xec
            pub LastTickBucket__BackingField: i32,                 // 0xf0
            pub _ServerEntityID: u32,                              // 0xf4
            pub RuntimeID__BackingField: u32,                      // 0xf8
            pub ObjectFeature__BackingField: [u8; 0x4],            // 0xfc
            pub _Team: TeamType,                                   // 0x100
            pub CampID__BackingField: [u8; 0x4],                   // 0x104
            pub _EntityType: EntityType,                           // 0x108
            pub _GroupID: u32,                                     // 0x10c
            pub _TickDelayFrameCount: u32,                         // 0x110
        }
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct TurnBasedGameMode {
            pub il2cpp_object: Il2CppObject,
            pub _CurrentSkillCharacter: *const c_void,	// 0x10
            pub _performParam: *const c_void,	// 0x18
            pub StageBattleEventMgr__BackingField: *const c_void,	// 0x20
            pub _ActionDelayChangeStamp: [u8; 0x18],	// 0x28
            pub _allowQuitStates: *const Il2CppArray<Il2CppObject>,	// 0x40
            pub BattleChangeAvatarManager__BackingField: *const c_void,	// 0x48
            pub _SkillAddBuffPerformList: *const Il2CppArray<Il2CppObject>,	// 0x50
            pub _EventProcessor: *const c_void,	// 0x58
            pub LastKillSkill__BackingField: *const c_void,	// 0x60
            pub _EntityCustomUnselectableDatas: *const Il2CppArray<Il2CppObject>,	// 0x68
            pub _ModifierPerformCamerContext: *const c_void,	// 0x70
            pub _UnselectableEntities: *const Il2CppArray<GameEntity>,	// 0x78
            pub _AllTeamCharacters: *const Il2CppArray<GameEntity>,	// 0x80
            pub _CachedDynamicSkillTargetSelection: *const GameEntity,	// 0x88
            pub _LimboRevivableEntities: *const c_void,	// 0x90
            pub _EvolveBuildGearMgr: *const c_void,	// 0x98
            pub _OverrieWaveMonsterPerformDatas: *const Il2CppArray<Il2CppObject>,	// 0xa0
            pub _LevelLockedFeatureSet: *const c_void,	// 0xa8
            pub TurnActionDelayCostChangeSource__BackingField: *const GameEntity,	// 0xb0
            pub OwnerBattleInstanceRef__BackingField: *const c_void,	// 0xb8
            pub _LimboEntities: *const Il2CppArray<HBIAGLPHICO>,	// 0xc0
            pub _TurnStateFSM: *const c_void,	// 0xc8
            pub _AllOffTeamCharacters: *const Il2CppArray<GameEntity>,	// 0xd0
            pub _LimboEntitiesWaitAbilityFinish: *const Il2CppArray<HBIAGLPHICO>,	// 0xd8
            pub _ActionDelayLinkMgr: *const c_void,	// 0xe0
            pub _WaitingAbilityList: *const Il2CppArray<Il2CppObject>,	// 0xe8
            pub TimeGameStart: *const c_void,	// 0xf0
            pub BattleCounter: *const c_void,	// 0xf8
            pub SkillUsageLog__BackingField: *const c_void,	// 0x100
            pub _VersusBarMgr: *const c_void,	// 0x108
            pub LastSummonMonsterList: *const Il2CppArray<GameEntity>,	// 0x110
            pub PerformDelayExecuteList: *const Il2CppArray<Il2CppObject>,	// 0x118
            pub _SomatoModifierPerforms: *const Il2CppArray<Il2CppObject>,	// 0x120
            pub _ActionEntityList: *const Il2CppArray<GameEntity>,	// 0x128
            pub AssistantAvatarEntity__BackingField: *const GameEntity,	// 0x130
            pub GridFightMananger__BackingField: *const c_void,	// 0x138
            pub LastKillCaster__BackingField: *const GameEntity,	// 0x140
            pub BattleEventInitedData__BackingField: *const c_void,	// 0x148
            pub _SwordTrainingMgr: *const c_void,	// 0x150
            pub _CurModifierPerformSeq: *const c_void,	// 0x158
            pub _EntityModifierPerforms: *const c_void,	// 0x160
            pub CurMainMonster__BackingField: *const GameEntity,	// 0x168
            pub _LastBreakMonster: *const GameEntity,	// 0x170
            pub _CurrentTurnActionEntity: *const GameEntity,	// 0x178
            pub CurrentWaveMainMonsterIDPool__BackingField: u32,	// 0x180
            pub _ActionEntityListSnapshot: *const Il2CppArray<GameEntity>,	// 0x188
            pub _AidDetail: *const c_void,	// 0x190
            pub _CurrentActionDelayModifyGroup: *const Il2CppArray<GameEntity>,	// 0x198
            pub LastZombie__BackingField: *const GameEntity,	// 0x1a0
            pub PrepareAbility__BackingField: *const c_void,	// 0x1a8
            pub _AvatarChangeParam: *const c_void,	// 0x1b0
            pub ThisTurnAnimEvents: *const c_void,	// 0x1b8
            pub LastTurnSnapshot: *const c_void,	// 0x1c0
            pub _RogueInBattleData: *const c_void,	// 0x1c8
            pub _ReplayData: *const c_void,	// 0x1d0
            pub _InsertAbilityList: *const Il2CppArray<Il2CppObject>,	// 0x1d8
            pub _CurrentTurnTargetEntity: *const GameEntity,	// 0x1e0
            pub DamageQueue__BackingField: *const c_void,	// 0x1e8
            pub MonsterWaveTextInfo: *const c_void,	// 0x1f0
            pub CurrentMVPEntity__BackingField: *const GameEntity,	// 0x1f8
            pub ActionBarMgr__BackingField: *const c_void,	// 0x200
            pub _LinkTeammateList: *const Il2CppArray<GameEntity>,	// 0x208
            pub LastKillTargetList__BackingField: *const Il2CppArray<GameEntity>,	// 0x210
            pub CurrentTurnOwnerEntity__BackingField: *const GameEntity,	// 0x218
            pub _LimboEntitiesSkipSettlement: *const Il2CppArray<HBIAGLPHICO>,	// 0x220
            pub _ActionDelayOrderTrigger: *const c_void,	// 0x228
            pub _InsertUltraSkillParamsQueue: *const Il2CppArray<Il2CppObject>,	// 0x230
            pub _PhaseModifierList: *const Il2CppArray<Il2CppObject>,	// 0x238
            pub _CommonSkillPoolNames: *const c_void,	// 0x240
            pub _RelationGroupMgr: *const c_void,	// 0x248
            pub _AttackingEntityList: *const c_void,	// 0x250
            pub _TurnCounter: u32,	// 0x258
            pub _DarkTeamTurnCount: u32,	// 0x25c
            pub WaveMonsterMaxCount__BackingField: i32,	// 0x260
            pub MuteLastKillTriggered: bool,	// 0x264
            pub AddOpCountOnInsertUltraWaitOrder: bool,	// 0x265
            pub _RequireMakeLimboEntitiesDie: bool,	// 0x266
            pub _AutoBattle: bool,	// 0x267
            pub BattleResultState__BackingField: *const c_void,	// 0x268
            pub ChallengeTurnLimitType__BackingField: [u8; 0x4],	// 0x270
            pub LastKillFinish__BackingField: bool,	// 0x274
            pub IsManualExitBattle: bool,	// 0x275
            pub TurnOwnerPrepareAbilityUsed__BackingField: bool,	// 0x276
            pub ApplyUIOperateOnSkillDisableChange: bool,	// 0x277
            pub _ModifierPerformTimeScale: f32,	// 0x278
            pub ChallengeTurnLimit__BackingField: u32,	// 0x27c
            pub _HitPerformMinTimer: f32,	// 0x280
            pub ElapsedActionDelay__BackingField: FixPoint,	// 0x288
            pub CurrentInsertSkillSkipActionFlag: bool,	// 0x290
            pub BattleResultAsWin: bool,	// 0x291
            pub CertainlyLoseInAdvance__BackingField: bool,	// 0x292
            pub _SkillExecutionEventState: [u8; 0x4],	// 0x294
            pub CertainlyWinInAdvance__BackingField: bool,	// 0x298
            pub PrepareAbilityFinish__BackingField: bool,	// 0x299
            pub _LastReplayAutoBattle: bool,	// 0x29a
            pub ApplyUIOperateOnDisableActionFlagChange: bool,	// 0x29b
            pub _NextModifierIndex: i32,	// 0x29c
            pub WinFlag: bool,	// 0x2a0
            pub IsManualRetryExitBattle: bool,	// 0x2a1
            pub LocalWinFlag__BackingField: [u8; 0x2],	// 0x2a2
            pub ForbidAI: bool,	// 0x2a4
            pub _ModifierEndingPerformRemainedTime: f32,	// 0x2a8
            pub CurrentWaveStageID__BackingField: u32,	// 0x2ac
            pub CurrentModeState__BackingField: [u8; 0x4],	// 0x2b0
            pub _DeathVersion: u32,	// 0x2b4
            pub ShowCutinUIState__BackingField: [u8; 0x4],	// 0x2b8
            pub PauseState__BackingField: [u8; 0x4],	// 0x2bc
            pub _ChallengeTurnAcc: u32,	// 0x2c0
            pub _CachedDynamicSkillInput: [u8; 0x4],	// 0x2c4
            pub _LightTeamTurnCount: u32,	// 0x2c8
            pub TurnActionDelayCostRatio__BackingField: FixPoint,	// 0x2d0
            pub CurrentWaveIndexInStage__BackingField: u32,	// 0x2d8
            pub _NextAbilityIndex: i32,	// 0x2dc
            pub TurnOwnerActionPhaseEnd__BackingField: bool,	// 0x2e0
            pub ClearUltraSkillEffect: bool,	// 0x2e1
            pub _IsUseLinkSkill: bool,	// 0x2e2
            pub SkipDeathHandle__BackingField: bool,	// 0x2e3
            pub _HoldFrameForCapture: u32,	// 0x2e4
            pub _PrevTickModeState: [u8; 0x4],	// 0x2e8
            pub RealTimeCounter__BackingField: f32,	// 0x2ec
            pub _ModifierPerformTimeTotal: f32,	// 0x2f0
            pub _ActionEntityListInited: bool,	// 0x2f4
            pub IsActionOrder1UsedTBSkill__BackingField: bool,	// 0x2f5
            pub _IsReplayBeingSaved: bool,	// 0x2f6
            pub _GamePauseFlag: bool,	// 0x2f7
            pub StanceCountDownSPChangeValue__BackingField: f32,	// 0x2f8
            pub UseSkillOneMoreDefaultSkill: [u8; 0x4],	// 0x2fc
            pub _OperationCounter: u32,	// 0x300
            pub TurnEndKeep: bool,	// 0x304
            pub IsTeamFormationExpansion__BackingField: bool,	// 0x305
            pub _IsCreatingNewWave: bool,	// 0x306
            pub _IsModifierPerformCameraSet: bool,	// 0x307
            pub _ModifierPerformTimerTotal: f32,	// 0x308
            pub _DamageCounter: u32,	// 0x30c
            pub ThisTurnAnimEventCount: i32,	// 0x310
            pub BattleFinishReason: [u8; 0x4],	// 0x314
            pub PendingMonsterToWave__BackingField: bool,	// 0x318
            pub SkipCameraDitherByLastKill: bool,	// 0x319
            pub _HoldFrameForCaptureFlag: bool,	// 0x31a
            pub IsUseSkillOneMore: bool,	// 0x31b
            pub _OverrideAILocked: bool,	// 0x31c
            pub AutoInsertUltraSkill: bool,	// 0x31d
            pub IsLastKillTriggered: bool,	// 0x31e
            pub _CurrentTurnActionEntitySkipActionFlag: bool,	// 0x31f
            pub _WaveMonsterCurrentCount: i32,	// 0x320
            pub _CurrentTurnTeam: TeamType	// 0x324
        }
    }
}
