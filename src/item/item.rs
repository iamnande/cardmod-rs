#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Item {
    // restorative
    Potion,
    PotionPlus,
    HiPotion,
    HiPotionPlus,
    XPotion,
    MegaPotion,

    // refinement
    PhoenixDown,
    MegaPhoenix,
    MStonePiece,
    MagicStone,
    WizardStone,
    OchuTentacle,
    HealingWater,
    CockatricePinion,
    ZombiePowder,
    Lightweight,
    SharpSpike,
    Screw,
    SawBlade,
    MesmerizeBlade,
    VampireFang,
    FuryFragment,
    BetrayalSword,
    SleepPowder,
    LifeRing,
    DragonFang,

    // forbidden medicine
    Elixir,
    Megalixir,

    // status recovery
    Antidote,
    Soft,
    EyeDrops,
    EchoScreen,
    HolyWater,
    Remedy,
    RemedyPlus,

    // invincibility
    HeroTrial,
    Hero,
    HolyWarTrial,
    HolyWar,

    // spell stones
    ShellStone,
    ProtectStone,
    AuraStone,
    DeathStone,
    HolyStone,
    FlareStone,
    MeteorStone,
    UltimaStone,

    // gf summon
    GysahlGreens,
    PhoenixPinion,
    Friendship,

    // shelters
    Tent,
    PetHouse,
    Cottage,

    // gf recovery
    GPotion,
    GHiPotion,
    GMegaPotion,
    GReturner,

    // rename card
    RenameCard,

    // gf ability
    HPJScroll,
    StrJScroll,
    VitJScroll,
    MagJScroll,
    SprJScroll,
    LuckJScroll,
    AegisAmulet,
    ElemAtk,
    ElemGuard,
    StatusAtk,
    StatusGuard,
    RosettaStone,

    // command ability
    MagicScroll,
    GFScroll,
    DrawScroll,
    ItemScroll,
    GamblerSpirit,
    HealingRing,
    PhoenixSpirit,
    MedKit,
    BombSpirit,
    HungryCookpot,
    MegsAmulet,

    // gf enhancement
    SteelPipe,
    StarFragment,
    EnergyCrystal,
    SamanthaSoul,
    HealingMail,
    SilverSail,
    GoldArmor,
    DiamondArmor,

    // character ability
    RegenRing,
    GiantsRing,
    GaeasRing,
    StrengthLove,
    PowerWrist,
    HyperWrist,
    TurtleShell,
    Orihalcon,
    Adamantine,
    RuneArmlet,
    ForceArmlet,
    MagicArmlet,
    Circlet,
    HypnoCrown,
    RoyalCrown,
    JetEngine,
    RocketEngine,
    MoonCurtain,
    SteelCurtain,
    GlowCurtain,
    Accelerator,
    MonksCode,
    KnightsCode,
    DocsCode,
    HundredNeedles,
    ThreeStars,
    Ribbon,

    // ammo
    NormalAmmo,
    ShotgunAmmo,
    DarkAmmo,
    FireAmmo,
    DemolitionAmmo,
    FastAmmo,
    APAmmo,
    PulseAmmo,

    // blue magic
    SpiderWeb,
    CoralFragment,
    CurseSpike,
    BlackHole,
    WaterCrystal,
    Missile,
    MysteryFluid,
    RunningFire,
    InfernoFang,
    MalboroTentacle,
    Whisper,
    LaserCannon,
    Barrier,
    PowerGenerator,
    DarkMatter,

    // gf compatibility
    BombFragment,
    RedFang,
    ArcticWind,
    NorthWind,
    DynamoStone,
    ShearFeather,
    VenomFang,
    SteelOrb,
    MoonStone,
    DinoBone,
    Windmill,
    DragonSkin,
    FishFin,
    DragonFin,
    SilencePowder,
    PoisonPowder,
    DeadSpirit,
    ChefsKnife,
    CactusThorn,
    ShamanStone,

    // fuel
    Fuel,

    // stat boosting
    HPUp,
    StrUp,
    VitUp,
    MagUp,
    SprUp,
    SpdUp,
    LuckUp,

    // luvluv g
    LuvLuvG,
}

impl Item {
    pub const fn purpose(self) -> Purpose {
        match self {
            // restorative
            Item::Potion
            | Item::PotionPlus
            | Item::HiPotion
            | Item::HiPotionPlus
            | Item::XPotion
            | Item::MegaPotion => Purpose::Restorative,

            // refinement
            Item::PhoenixDown
            | Item::MegaPhoenix
            | Item::MStonePiece
            | Item::MagicStone
            | Item::WizardStone
            | Item::OchuTentacle
            | Item::HealingWater
            | Item::CockatricePinion
            | Item::ZombiePowder
            | Item::Lightweight
            | Item::SharpSpike
            | Item::Screw
            | Item::SawBlade
            | Item::MesmerizeBlade
            | Item::VampireFang
            | Item::FuryFragment
            | Item::BetrayalSword
            | Item::SleepPowder
            | Item::LifeRing
            | Item::DragonFang => Purpose::Refinement,

            Item::Elixir | Item::Megalixir => Purpose::ForbiddenMedicine,

            Item::Antidote
            | Item::Soft
            | Item::EyeDrops
            | Item::EchoScreen
            | Item::HolyWater
            | Item::Remedy
            | Item::RemedyPlus => Purpose::StatusRecovery,

            Item::HeroTrial
            | Item::Hero
            | Item::HolyWarTrial
            | Item::HolyWar => Purpose::Invincibility,

            Item::ShellStone
            | Item::ProtectStone
            | Item::AuraStone
            | Item::DeathStone
            | Item::HolyStone
            | Item::FlareStone
            | Item::MeteorStone
            | Item::UltimaStone => Purpose::SpellStones,

            Item::GysahlGreens
            | Item::PhoenixPinion
            | Item::Friendship => Purpose::GFSummon,

            Item::Tent | Item::PetHouse | Item::Cottage => Purpose::Shelters,

            Item::GPotion
            | Item::GHiPotion
            | Item::GMegaPotion
            | Item::GReturner => Purpose::GFRecovery,

            Item::RenameCard => Purpose::RenameCard,

            Item::HPJScroll
            | Item::StrJScroll
            | Item::VitJScroll
            | Item::MagJScroll
            | Item::SprJScroll
            | Item::LuckJScroll
            | Item::AegisAmulet
            | Item::ElemAtk
            | Item::ElemGuard
            | Item::StatusAtk
            | Item::StatusGuard
            | Item::RosettaStone => Purpose::GFAbility,

            Item::MagicScroll
            | Item::GFScroll
            | Item::DrawScroll
            | Item::ItemScroll
            | Item::GamblerSpirit
            | Item::HealingRing
            | Item::PhoenixSpirit
            | Item::MedKit
            | Item::BombSpirit
            | Item::HungryCookpot
            | Item::MegsAmulet => Purpose::CommandAbility,

            Item::SteelPipe
            | Item::StarFragment
            | Item::EnergyCrystal
            | Item::SamanthaSoul
            | Item::HealingMail
            | Item::SilverSail
            | Item::GoldArmor
            | Item::DiamondArmor => Purpose::GFEnhancement,

            Item::RegenRing
            | Item::GiantsRing
            | Item::GaeasRing
            | Item::StrengthLove
            | Item::PowerWrist
            | Item::HyperWrist
            | Item::TurtleShell
            | Item::Orihalcon
            | Item::Adamantine
            | Item::RuneArmlet
            | Item::ForceArmlet
            | Item::MagicArmlet
            | Item::Circlet
            | Item::HypnoCrown
            | Item::RoyalCrown
            | Item::JetEngine
            | Item::RocketEngine
            | Item::MoonCurtain
            | Item::SteelCurtain
            | Item::GlowCurtain
            | Item::Accelerator
            | Item::MonksCode
            | Item::KnightsCode
            | Item::DocsCode
            | Item::HundredNeedles
            | Item::ThreeStars
            | Item::Ribbon => Purpose::CharacterAbility,

            Item::NormalAmmo
            | Item::ShotgunAmmo
            | Item::DarkAmmo
            | Item::FireAmmo
            | Item::DemolitionAmmo
            | Item::FastAmmo
            | Item::APAmmo
            | Item::PulseAmmo => Purpose::Ammo,

            Item::SpiderWeb
            | Item::CoralFragment
            | Item::CurseSpike
            | Item::BlackHole
            | Item::WaterCrystal
            | Item::Missile
            | Item::MysteryFluid
            | Item::RunningFire
            | Item::InfernoFang
            | Item::MalboroTentacle
            | Item::Whisper
            | Item::LaserCannon
            | Item::Barrier
            | Item::PowerGenerator
            | Item::DarkMatter => Purpose::BlueMagic,

            Item::BombFragment
            | Item::RedFang
            | Item::ArcticWind
            | Item::NorthWind
            | Item::DynamoStone
            | Item::ShearFeather
            | Item::VenomFang
            | Item::SteelOrb
            | Item::MoonStone
            | Item::DinoBone
            | Item::Windmill
            | Item::DragonSkin
            | Item::FishFin
            | Item::DragonFin
            | Item::SilencePowder
            | Item::PoisonPowder
            | Item::DeadSpirit
            | Item::ChefsKnife
            | Item::CactusThorn
            | Item::ShamanStone => Purpose::GFCompatibility,

            Item::Fuel => Purpose::Fuel,

            Item::HPUp
            | Item::StrUp
            | Item::VitUp
            | Item::MagUp
            | Item::SprUp
            | Item::SpdUp
            | Item::LuckUp => Purpose::StatBoosting,

            Item::LuvLuvG => Purpose::LuvLuvG,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            // restorative
            Item::Potion => "Potion",
            Item::PotionPlus => "Potion+",
            Item::HiPotion => "Hi-Potion",
            Item::HiPotionPlus => "Hi-Potion+",
            Item::XPotion => "X-Potion",
            Item::MegaPotion => "Mega-Potion",

            // refinement
            Item::PhoenixDown => "Phoenix Down",
            Item::MegaPhoenix => "Mega Phoenix",
            Item::MStonePiece => "M-Stone Piece",
            Item::MagicStone => "Magic Stone",
            Item::WizardStone => "Wizard Stone",
            Item::OchuTentacle => "Ochu Tentacle",
            Item::HealingWater => "Healing Water",
            Item::CockatricePinion => "Cockatrice Pinion",
            Item::ZombiePowder => "Zombie Powder",
            Item::Lightweight => "Lightweight",
            Item::SharpSpike => "Sharp Spike",
            Item::Screw => "Screw",
            Item::SawBlade => "Saw Blade",
            Item::MesmerizeBlade => "Mesmerize Blade",
            Item::VampireFang => "Vampire Fang",
            Item::FuryFragment => "Fury Fragment",
            Item::BetrayalSword => "Betrayal Sword",
            Item::SleepPowder => "Sleep Powder",
            Item::LifeRing => "Life Ring",
            Item::DragonFang => "Dragon Fang",

            // forbidden medicine
            Item::Elixir => "Elixir",
            Item::Megalixir => "Megalixir",

            // status recovery
            Item::Antidote => "Antidote",
            Item::Soft => "Soft",
            Item::EyeDrops => "Eye Drops",
            Item::EchoScreen => "Echo Screen",
            Item::HolyWater => "Holy Water",
            Item::Remedy => "Remedy",
            Item::RemedyPlus => "Remedy+",

            // invincibility
            Item::HeroTrial => "Hero-trial",
            Item::Hero => "Hero",
            Item::HolyWarTrial => "Holy War-trial",
            Item::HolyWar => "Holy War",

            // spell stones
            Item::ShellStone => "Shell Stone",
            Item::ProtectStone => "Protect Stone",
            Item::AuraStone => "Aura Stone",
            Item::DeathStone => "Death Stone",
            Item::HolyStone => "Holy Stone",
            Item::FlareStone => "Flare Stone",
            Item::MeteorStone => "Meteor Stone",
            Item::UltimaStone => "Ultima Stone",

            // GF summon
            Item::GysahlGreens => "Gysahl Greens",
            Item::PhoenixPinion => "Phoenix Pinion",
            Item::Friendship => "Friendship",

            // shelters
            Item::Tent => "Tent",
            Item::PetHouse => "Pet House",
            Item::Cottage => "Cottage",

            // GF recovery
            Item::GPotion => "G-Potion",
            Item::GHiPotion => "G-Hi-Potion",
            Item::GMegaPotion => "G-Mega-Potion",
            Item::GReturner => "G-Returner",

            // rename card
            Item::RenameCard => "Rename Card",

            // GF ability
            Item::HPJScroll => "HP-J Scroll",
            Item::StrJScroll => "Str-J Scroll",
            Item::VitJScroll => "Vit-J Scroll",
            Item::MagJScroll => "Mag-J Scroll",
            Item::SprJScroll => "Spr-J Scroll",
            Item::LuckJScroll => "Luck-J Scroll",
            Item::AegisAmulet => "Aegis Amulet",
            Item::ElemAtk => "Elem Atk",
            Item::ElemGuard => "Elem Guard",
            Item::StatusAtk => "Status Atk",
            Item::StatusGuard => "Status Guard",
            Item::RosettaStone => "Rosetta Stone",

            // command ability
            Item::MagicScroll => "Magic Scroll",
            Item::GFScroll => "GF Scroll",
            Item::DrawScroll => "Draw Scroll",
            Item::ItemScroll => "Item Scroll",
            Item::GamblerSpirit => "Gambler Spirit",
            Item::HealingRing => "Healing Ring",
            Item::PhoenixSpirit => "Phoenix Spirit",
            Item::MedKit => "Med Kit",
            Item::BombSpirit => "Bomb Spirit",
            Item::HungryCookpot => "Hungry Cookpot",
            Item::MegsAmulet => "Meg's Amulet",

            // GF enhancement
            Item::StellPipe => "Stell Pipe",
            Item::StarFragment => "Star Fragment",
            Item::EnergyCrystal => "Energy Crystal",
            Item::SamanthaSoul => "Samantha Soul",
            Item::HealingMail => "Healing Mail",
            Item::SilverSail => "Silver Sail",
            Item::GoldArmor => "Gold Armor",
            Item::DiamondArmor => "Diamond Armor",

            // character ability
            Item::RegenRing => "Regen Ring",
            Item::GiantsRing => "Giant's Ring",
            Item::GaeasRing => "Gaea's Ring",
            Item::StrengthLove => "Strength Love",
            Item::PowerWrist => "Power Wrist",
            Item::HyperWrist => "Hyper Wrist",
            Item::TurtleShell => "Turtle Shell",
            Item::Orihalcon => "Orihalcon",
            Item::Adamantine => "Adamantine",
            Item::RuneArmlet => "Rune Armlet",
            Item::ForceArmlet => "Force Armlet",
            Item::MagicArmlet => "Magic Armlet",
            Item::Circlet => "Circlet",
            Item::HypnoCrown => "Hypno Crown",
            Item::RoyalCrown => "Royal Crown",
            Item::JetEngine => "Jet Engine",
            Item::RocketEngine => "Rocket Engine",
            Item::MoonCurain => "Moon Curain",
            Item::SteelCurtain => "Steel Curtain",
            Item::GlowCurtain => "Glow Curtain",
            Item::Accelerator => "Accelerator",
            Item::MonksCode => "Monk's Code",
            Item::KnightsCode => "Knight's Code",
            Item::DocsCode => "Doc's Code",
            Item::HundredNeedles => "Hundred Needles",
            Item::ThreeStars => "Three Stars",
            Item::Ribbon => "Ribbon",

            // ammo
            Item::NormalAmmo => "Normal Ammo",
            Item::ShotgunAmmo => "Shotgun Ammo",
            Item::DarkAmmo => "Dark Ammo",
            Item::FireAmmo => "Fire Ammo",
            Item::DemolitionAmmo => "Demolition Ammo",
            Item::FastAmmo => "Fast Ammo",
            Item::APAmmo => "AP Ammo",
            Item::PulseAmmo => "Pulse Ammo",

            // GF compatibility
            Item::BombFragment => "Bomb Fragment",
            Item::RedFang => "Red Fang",
            Item::ArcticWind => "Arctic Wind",
            Item::NorthWind => "North Wind",
            Item::DynamoStone => "Dynamo Stone",
            Item::ShearFeather => "Shear Feather",
            Item::VenomFang => "Venom Fang",
            Item::SteelOrb => "Steel Orb",
            Item::MoonStone => "Moon Stone",
            Item::DinoBone => "Dino Bone",
            Item::Windmill => "Windmill",
            Item::DragonSkin => "Dragon Skin",
            Item::FishFin => "Fish Fin",
            Item::DragonFin => "Dragon Fin",
            Item::SilencePowder => "Silence Powder",
            Item::PoisonPowder => "Poison Powder",
            Item::DeadSpirit => "Dead Spirit",
            Item::ChefsKnife => "Chef's Knife",
            Item::CactusThorn => "Cactus Thorn",
            Item::ShamanStone => "Shaman Stone",

            // fuel
            Item::Fuel => "Fuel",

            // stat boosting
            Item::HPUp => "HP Up",
            Item::StrUp => "Str Up",
            Item::VitUp => "Vit Up",
            Item::MagUp => "Mag Up",
            Item::SprUp => "Spr Up",
            Item::SpdUp => "Spd Up",
            Item::LuckUp => "Luck Up",

            // luvluv g
            Item::LuvLuvG => "LuvLuv G",
        }
    }
}
