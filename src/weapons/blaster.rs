//! Blaster - The default starting weapon
//!
//! A reliable energy pistol that rewards skilled timing.
//!
//! ## Characteristics
//! - **Single Shot**: Tap to fire one projectile. No automatic fire.
//! - **Charged Shot**: Hold to charge, release to fire a powerful shot.
//! - **Balanced**: Not spammable, but consistent damage output.
//!
//! ## Strategy
//! - Use single shots as "filler damage" while repositioning
//! - Master the charge timing for burst damage opportunities
//! - Charged shots are worth 5x the damage of normal shots

use super::{CriticalConfig, DamageConfig, DamageType, FalloffConfig, WeaponStats};
use bevy::prelude::*;

/// Blaster weapon constants
pub mod constants {
    use bevy::prelude::*;

    // Damage
    pub const BLASTER_DAMAGE: i32 = 1;
    pub const BLASTER_CHARGED_DAMAGE: i32 = 5;

    // Timing
    pub const BLASTER_CHARGE_TIME: f32 = 0.6; // Time to fully charge
    pub const BLASTER_FIRE_COOLDOWN: f32 = 0.25; // Cooldown after any shot

    // Critical hits
    pub const BLASTER_CRIT_CHANCE: f32 = 0.08; // 8% crit chance
    pub const BLASTER_CRIT_MULTIPLIER: f32 = 1.5; // 1.5x on crit

    // Projectile
    pub const BLASTER_RANGE: i32 = 6; // Full arena width
    pub const BLASTER_PROJECTILE_SPEED: f32 = 20.0;
    pub const BLASTER_PROJECTILE_SIZE: Vec2 = Vec2::new(16.0, 16.0);
    pub const BLASTER_CHARGED_SIZE: Vec2 = Vec2::new(28.0, 28.0);

    // Colors
    pub const BLASTER_COLOR: Color = Color::srgb(0.2, 0.8, 1.0); // Cyan energy
    pub const BLASTER_CHARGED_COLOR: Color = Color::srgb(0.4, 0.9, 1.0); // Bright cyan
}

use constants::*;

/// Create the stats for the Blaster weapon
pub fn blaster_stats() -> WeaponStats {
    WeaponStats {
        name: "Blaster".to_string(),

        // Normal shot: 1 damage, filler shots
        damage: DamageConfig {
            amount: BLASTER_DAMAGE,
            damage_type: DamageType::Physical,
        },

        // Charged shot: 5 damage, rewarding timing mastery
        charged_damage: Some(DamageConfig {
            amount: BLASTER_CHARGED_DAMAGE,
            damage_type: DamageType::Physical,
        }),

        // Charge time - not too long, but requires commitment
        charge_time: BLASTER_CHARGE_TIME,

        // Low crit chance, but crits feel good
        critical: CriticalConfig {
            chance: BLASTER_CRIT_CHANCE,
            multiplier: BLASTER_CRIT_MULTIPLIER,
            orange_multiplier: 2.5,
            red_multiplier: 4.0,
        },

        // Quick cooldown - can fire rapidly if just tapping
        fire_cooldown: BLASTER_FIRE_COOLDOWN,

        // No falloff for blaster - consistent damage at all ranges
        falloff: FalloffConfig::none(),

        // Full arena range
        range: BLASTER_RANGE,

        // Fast projectile
        projectile_speed: BLASTER_PROJECTILE_SPEED,

        // Visual configuration
        projectile_size: BLASTER_PROJECTILE_SIZE,
        projectile_color: BLASTER_COLOR,
        charged_projectile_size: BLASTER_CHARGED_SIZE,
        charged_projectile_color: BLASTER_CHARGED_COLOR,
    }
}
