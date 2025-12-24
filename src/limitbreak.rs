#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum LimitBreak {
    UltraWaves,
    Electrocute,
    LDeath,
    Degenerator,
    AquaBreath,
    MicroMissiles,
    Acid,
    GatlingGun,
    FireBreath,
    BadBreath,
    WhiteWind,
    HomingLaser,
    MightyGuard,
    RayBomb,
    ShockwavePulsar,
}

impl LimitBreak {
    pub const fn name(self) -> &'static str {
        match self {
            LimitBreak::UltraWaves => "Ultra Waves",
            LimitBreak::Electrocute => "Electrocute",
            LimitBreak::LDeath => "L?Death",
            LimitBreak::Degenerator => "Degenerator",
            LimitBreak::AquaBreath => "Aqua Breath",
            LimitBreak::MicroMissiles => "Micro Missiles",
            LimitBreak::Acid => "Acid",
            LimitBreak::GatlingGun => "Gatling Gun",
            LimitBreak::FireBreath => "Fire Breath",
            LimitBreak::BadBreath => "Bad Breath",
            LimitBreak::WhiteWind => "White Wind",
            LimitBreak::HomingLaser => "Homing Laser",
            LimitBreak::MightyGuard => "Mighty Guard",
            LimitBreak::RayBomb => "Ray Bomb",
            LimitBreak::ShockwavePulsar => "Shockwave Pulsar",
        }
    }
}
