#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Card {
    // level one
    Geezard,
    Funguar,
    BiteBug,
    RedBat,
    Gayla,
    Gesper,
    FastitocalonF,
    BloodSoul,
    Caterchipillar,
    Cockatrice,

    // level two
    Grat,
    Buel,
    Mesmerize,
    GlacialEye,
    Belhelmel,
    Thrustaevis,
    Anacondaur,
    Creeps,
    Grendel,
    Jelleye,
    GrandMantis,

    // level three
    Forbidden,
    Armadodo,
    TriFace,
    Fastitocalon,
    SnowLion,
    Ochu,
    DeathClaw,
    Cactuar,
    Tonberry,
    AbyssWorm,

    // level four
    Turtapod,
    Vysage,
    TRexaur,
    Bomb,
    Blitz,
    Wendigo,
    Torama,
    Imp,
    BlueDragon,
    Adamantoise,
    Hexadragon,

    // level five
    IronGiant,
    Behemoth,
    Chimera,
    PuPu,
    Elastoid,
    GIM47N,
    Malboro,
    Elnoyle,
    TonberryKing,
    WedgeBiggs,

    // level six
    FujinRaijin,
    Elvoret,
    XATM092,
    Granaldo,
    Gerogero,
    Iguion,
    Abadon,
    Trauma,
    Oilboyle,
    ShumiTribe,
    Krysta,

    // level seven
    Propagator,
    JumboCactuar,
    TriPoint,
    Gargantua,
    MobileType8,
    Sphinxara,
    Tiamat,
    BGH251F2,
    RedGiant,
    Catoblepas,
    UltimaWeapon,

    // level eight
    ChubbyChocobo,
    Angelo,
    Gilgamesh,
    MiniMog,
    Chicobo,
    Quezacotl,
    Shiva,
    Ifrit,
    Siren,
    Sacred,
    Minotaur,

    // level nine
    Carbuncle,
    Diablos,
    Leviathan,
    Odin,
    Pandemona,
    Cerberus,
    Alexander,
    Phoenix,
    Bahamut,
    Doomtrain,
    Eden,

    // level ten
    Ward,
    Kiros,
    Laguna,
    Selphie,
    Quistis,
    Irvine,
    Zell,
    Rinoa,
    Edea,
    Seifer,
    Squall,
}

impl Card {
    pub const fn level(self) -> Level {
        match self {
            Card::Geezard
            | Card::Funguar
            | Card::BiteBug
            | Card::RedBat
            | Card::Gayla
            | Card::Gesper
            | Card::FastitocalonF
            | Card::BloodSoul
            | Card::Caterchipillar
            | Card::Cockatrice => Level::One,

            Card::Grat
            | Card::Buel
            | Card::Mesmerize
            | Card::GlacialEye
            | Card::Belhelmel
            | Card::Thrustaevis
            | Card::Anacondaur
            | Card::Creeps
            | Card::Grendel
            | Card::Jelleye
            | Card::GrandMantis => Level::Two,

            Card::Forbidden
            | Card::Armadodo
            | Card::TriFace
            | Card::Fastitocalon
            | Card::SnowLion
            | Card::Ochu
            | Card::DeathClaw
            | Card::Cactuar
            | Card::Tonberry
            | Card::AbyssWorm => Level::Three,

            Card::Turtapod
            | Card::Vysage
            | Card::TRexaur
            | Card::Bomb
            | Card::Blitz
            | Card::Wendigo
            | Card::Torama
            | Card::Imp
            | Card::BlueDragon
            | Card::Adamantoise
            | Card::Hexadragon => Level::Four,

            Card::IronGiant
            | Card::Behemoth
            | Card::Chimera
            | Card::PuPu
            | Card::Elastoid
            | Card::GIM47N
            | Card::Malboro
            | Card::Elnoyle
            | Card::TonberryKing
            | Card::WedgeBiggs => Level::Five,

            Card::FujinRaijin
            | Card::Elvoret
            | Card::XATM092
            | Card::Granaldo
            | Card::Gerogero
            | Card::Iguion
            | Card::Abadon
            | Card::Trauma
            | Card::Oilboyle
            | Card::ShumiTribe
            | Card::Krysta => Level::Six,

            Card::Propagator
            | Card::JumboCactuar
            | Card::TriPoint
            | Card::Gargantua
            | Card::MobileType8
            | Card::Sphinxara
            | Card::Tiamat
            | Card::BGH251F2
            | Card::RedGiant
            | Card::Catoblepas
            | Card::UltimaWeapon => Level::Seven,

            Card::ChubbyChocobo
            | Card::Angelo
            | Card::Gilgamesh
            | Card::MiniMog
            | Card::Chicobo
            | Card::Quezacotl
            | Card::Shiva
            | Card::Ifrit
            | Card::Siren
            | Card::Sacred
            | Card::Minotaur => Level::Eight,

            Card::Carbuncle
            | Card::Diablos
            | Card::Leviathan
            | Card::Odin
            | Card::Pandemona
            | Card::Cerberus
            | Card::Alexander
            | Card::Phoenix
            | Card::Bahamut
            | Card::Doomtrain
            | Card::Eden => Level::Nine,

            Card::Ward
            | Card::Kiros
            | Card::Laguna
            | Card::Selphie
            | Card::Quistis
            | Card::Irvine
            | Card::Zell
            | Card::Rinoa
            | Card::Edea
            | Card::Seifer
            | Card::Squall => Level::Ten,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Card::Geezard => "Geezard",
            Card::Funguar => "Funguar",
            Card::BiteBug => "Bite Bug",
            Card::RedBat => "Red Bat",
            Card::Gayla => "Gayla",
            Card::Gesper => "Gesper",
            Card::FastitocalonF => "Fastitocalon-F",
            Card::BloodSoul => "Blood Soul",
            Card::Caterchipillar => "Caterchipillar",
            Card::Cockatrice => "Cockatrice",
            Card::Grat => "Grat",
            Card::Buel => "Buel",
            Card::Mesmerize => "Mesmerize",
            Card::GlacialEye => "Glacial Eye",
            Card::Belhelmel => "Belhelmel",
            Card::Thrustaevis => "Thrustaevis",
            Card::Anacondaur => "Anacondaur",
            Card::Creeps => "Creeps",
            Card::Grendel => "Grendel",
            Card::Jelleye => "Jelleye",
            Card::GrandMantis => "Grand Mantis",
            Card::Forbidden => "Forbidden",
            Card::Armadodo => "Armadodo",
            Card::TriFace => "Tri-Face",
            Card::Fastitocalon => "Fastitocalon",
            Card::SnowLion => "Snow Lion",
            Card::Ochu => "Ochu",
            Card::DeathClaw => "Death Claw",
            Card::Cactuar => "Cactuar",
            Card::Tonberry => "Tonberry",
            Card::AbyssWorm => "Abyss Worm",
            Card::Turtapod => "Turtapod",
            Card::Vysage => "Vysage",
            Card::TRexaur => "T-Rexaur",
            Card::Bomb => "Bomb",
            Card::Blitz => "Blitz",
            Card::Wendigo => "Wendigo",
            Card::Torama => "Torama",
            Card::Imp => "Imp",
            Card::BlueDragon => "Blue Dragon",
            Card::Adamantoise => "Adamantoise",
            Card::Hexadragon => "Hexadragon",
            Card::IronGiant => "Iron Giant",
            Card::Behemoth => "Behemoth",
            Card::Chimera => "Chimera",
            Card::PuPu => "PuPu",
            Card::Elastoid => "Elastoid",
            Card::GIM47N => "GIM47N",
            Card::Malboro => "Malboro",
            Card::Elnoyle => "Elnoyle",
            Card::TonberryKing => "Tonberry King",
            Card::WedgeBiggs => "Wedge, Biggs",
            Card::FujinRaijin => "Fujin, Raijin",
            Card::Elvoret => "Elvoret",
            Card::XATM092 => "X-ATM092",
            Card::Granaldo => "Granaldo",
            Card::Gerogero => "Gerogero",
            Card::Iguion => "Iguion",
            Card::Abadon => "Abadon",
            Card::Trauma => "Trauma",
            Card::Oilboyle => "Oilboyle",
            Card::ShumiTribe => "Shumi Tribe",
            Card::Krysta => "Krysta",
            Card::Propagator => "Propagator",
            Card::JumboCactuar => "Jumbo Cactuar",
            Card::TriPoint => "Tri-Point",
            Card::Gargantua => "Gargantua",
            Card::MobileType8 => "Mobile Type 8",
            Card::Sphinxara => "Sphinxara",
            Card::Tiamat => "Tiamat",
            Card::BGH251F2 => "BGH251F2",
            Card::RedGiant => "Red Giant",
            Card::Catoblepas => "Catoblepas",
            Card::UltimaWeapon => "Ultima Weapon",
            Card::ChubbyChocobo => "Chubby Chocobo",
            Card::Angelo => "Angelo",
            Card::Gilgamesh => "Gilgamesh",
            Card::MiniMog => "MiniMog",
            Card::Chicobo => "Chicobo",
            Card::Quezacotl => "Quezacotl",
            Card::Shiva => "Shiva",
            Card::Ifrit => "Ifrit",
            Card::Siren => "Siren",
            Card::Sacred => "Sacred",
            Card::Minotaur => "Minotaur",
            Card::Carbuncle => "Carbuncle",
            Card::Diablos => "Diablos",
            Card::Leviathan => "Leviathan",
            Card::Odin => "Odin",
            Card::Pandemona => "Pandemona",
            Card::Cerberus => "Cerberus",
            Card::Alexander => "Alexander",
            Card::Phoenix => "Phoenix",
            Card::Bahamut => "Bahamut",
            Card::Doomtrain => "Doomtrain",
            Card::Eden => "Eden",
            Card::Ward => "Ward",
            Card::Kiros => "Kiros",
            Card::Laguna => "Laguna",
            Card::Selphie => "Selphie",
            Card::Quistis => "Quistis",
            Card::Irvine => "Irvine",
            Card::Zell => "Zell",
            Card::Rinoa => "Rinoa",
            Card::Edea => "Edea",
            Card::Seifer => "Seifer",
            Card::Squall => "Squall",
        }
    }
}
