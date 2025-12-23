#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Magic {
    // offensive
    Water,
    Aero,
    Bio,
    Demi,
    Quake,
    Tornado,
    Holy,
    Flare,
    Meteor,
    Ultima,
    Apocalypse,
    Fire,
    Fira,
    Firaga,
    Blizzard,
    Blizzara,
    Blizzaga,
    Thunder,
    Thundara,
    Thundaga,

    // restorative
    Esuna,
    Cure,
    Cura,
    Curaga,
    Life,
    FullLife,
    Regen,

    // defensive / status
    Scan,
    Sleep,
    Blind,
    Silence,
    Confuse,
    Berserk,
    Break,
    Zombie,
    Death,
    Double,
    Triple,
    Dispel,
    Protect,
    Shell,
    Reflect,
    Float,
    Drain,
    Haste,
    Slow,
    Stop,
    Meltdown,
    Pain,
    Aura,
}

impl Magic {
    pub const fn purpose(self) -> Purpose {
        match self {
            // offensive
            Magic::Water
            | Magic::Aero
            | Magic::Bio
            | Magic::Demi
            | Magic::Quake
            | Magic::Tornado
            | Magic::Holy
            | Magic::Flare
            | Magic::Meteor
            | Magic::Ultima
            | Magic::Apocalypse
            | Magic::Fire
            | Magic::Fira
            | Magic::Firaga
            | Magic::Blizzard
            | Magic::Blizzara
            | Magic::Blizzaga
            | Magic::Thunder
            | Magic::Thundara
            | Magic::Thundaga => Purpose::Offensive,

            // restorative
            Magic::Esuna
            | Magic::Cure
            | Magic::Cura
            | Magic::Curaga
            | Magic::Life
            | Magic::FullLife
            | Magic::Regen => Purpose::Restorative,

            // defensive / status
            Magic::Scan
            | Magic::Sleep
            | Magic::Blind
            | Magic::Silence
            | Magic::Confuse
            | Magic::Berserk
            | Magic::Break
            | Magic::Zombie
            | Magic::Death
            | Magic::Double
            | Magic::Triple
            | Magic::Dispel
            | Magic::Protect
            | Magic::Shell
            | Magic::Reflect
            | Magic::Float
            | Magic::Drain
            | Magic::Haste
            | Magic::Slow
            | Magic::Stop
            | Magic::Meltdown
            | Magic::Pain
            | Magic::Aura => Purpose::Defensive,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            // offensive
            Magic::Water => "Water",
            Magic::Aero => "Aero",
            Magic::Bio => "Bio",
            Magic::Demi => "Demi",
            Magic::Quake => "Quake",
            Magic::Tornado => "Tornado",
            Magic::Holy => "Holy",
            Magic::Flare => "Flare",
            Magic::Meteor => "Meteor",
            Magic::Ultima => "Ultima",
            Magic::Apocalypse => "Apocalypse",
            Magic::Fire => "Fire",
            Magic::Fira => "Fira",
            Magic::Firaga => "Firaga",
            Magic::Blizzard => "Blizzard",
            Magic::Blizzara => "Blizzara",
            Magic::Blizzaga => "Blizzaga",
            Magic::Thunder => "Thunder",
            Magic::Thundara => "Thundara",
            Magic::Thundaga => "Thundaga",

            // restorative
            Magic::Esuna => "Esuna",
            Magic::Cure => "Cure",
            Magic::Cura => "Cura",
            Magic::Curaga => "Curaga",
            Magic::Life => "Life",
            Magic::FullLife => "Full-Life",
            Magic::Regen => "Regen",

            // defensive / status
            Magic::Scan => "Scan",
            Magic::Sleep => "Sleep",
            Magic::Blind => "Blind",
            Magic::Silence => "Silence",
            Magic::Confuse => "Confuse",
            Magic::Berserk => "Berserk",
            Magic::Break => "Break",
            Magic::Zombie => "Zombie",
            Magic::Death => "Death",
            Magic::Double => "Double",
            Magic::Triple => "Triple",
            Magic::Dispel => "Dispel",
            Magic::Protect => "Protect",
            Magic::Shell => "Shell",
            Magic::Reflect => "Reflect",
            Magic::Float => "Float",
            Magic::Drain => "Drain",
            Magic::Haste => "Haste",
            Magic::Slow => "Slow",
            Magic::Stop => "Stop",
            Magic::Meltdown => "Meltdown",
            Magic::Pain => "Pain",
            Magic::Aura => "Aura",
        }
    }
}
