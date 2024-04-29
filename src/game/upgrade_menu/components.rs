use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Clone, PartialEq)]
pub enum UpgradeOption {
    GetWhip,
    GetForceField,
    GetFireBall,
    FireBallProjectile,
    FireBallDamage,
    FireBallSpeed,
    FireBallCoolDown,
    ForceFieldProjectile,
    ForceFieldDamage,
    ForceFieldLifeTime,
    ForceFieldCoolDown,
    WhipCount,
    WhipDamage,
    WhipCoolDown,
    HealthUp,
    SpeedUp,
    RegenerationUp,
}

impl UpgradeOption {

    pub fn pick_random_option() -> UpgradeOption {
        // use rand::Rng to pick random upgrade
        let mut rng = rand::thread_rng();
        let random_upgrade = rng.gen_range(0..17);
        match random_upgrade {
            0 => UpgradeOption::GetWhip,
            1 => UpgradeOption::GetForceField,
            2 => UpgradeOption::GetFireBall,
            3 => UpgradeOption::FireBallProjectile,
            4 => UpgradeOption::FireBallDamage,
            5 => UpgradeOption::FireBallSpeed,
            6 => UpgradeOption::FireBallCoolDown,
            7 => UpgradeOption::ForceFieldProjectile,
            8 => UpgradeOption::ForceFieldDamage,
            9 => UpgradeOption::ForceFieldLifeTime,
            10 => UpgradeOption::ForceFieldCoolDown,
            11 => UpgradeOption::WhipCount,
            12 => UpgradeOption::WhipDamage,
            13 => UpgradeOption::WhipCoolDown,
            14 => UpgradeOption::HealthUp,
            15 => UpgradeOption::SpeedUp,
            16 => UpgradeOption::RegenerationUp,
            _ => UpgradeOption::GetWhip,
        }

    }

    pub fn get_name(&self) -> &str {
        match self {
            UpgradeOption::GetWhip => "Get Whip",
            UpgradeOption::GetForceField => "Get Force Field",
            UpgradeOption::GetFireBall => "Get Fire Ball",
            UpgradeOption::FireBallProjectile => "Fire Ball Projectile +1",
            UpgradeOption::FireBallDamage => "Fire Ball Damage +10%",
            UpgradeOption::FireBallSpeed => "Fire Ball Speed +10%",
            UpgradeOption::FireBallCoolDown => "Fire Ball Cool Down -10%",
            UpgradeOption::ForceFieldProjectile => "Force Field Projectile +1",
            UpgradeOption::ForceFieldDamage => "Force Field Damage +10%",
            UpgradeOption::ForceFieldLifeTime => "Force Field Life Time +10%",
            UpgradeOption::ForceFieldCoolDown => "Force Field Cool Down -10%",
            UpgradeOption::WhipCount => "Whip Count +1",
            UpgradeOption::WhipDamage => "Whip Damage +10%",
            UpgradeOption::WhipCoolDown => "Whip Cool Down -10%",
            UpgradeOption::HealthUp => "Max Health +10%",
            UpgradeOption::SpeedUp => "Speed +10%",
            UpgradeOption::RegenerationUp => "Regeneration +0.1/s",
        }
    }
}