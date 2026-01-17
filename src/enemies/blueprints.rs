// ============================================================================
// Enemy Blueprints - Complete enemy definitions
// ============================================================================
//
// A blueprint is a complete template for spawning an enemy.
// It combines stats, behaviors, traits, and visuals into one package.

use super::{
    AttackBehavior, EnemyAnimations, EnemyId, EnemyStats, EnemyTraits, EnemyVisuals,
    MovementBehavior,
};
use bevy::prelude::*;

/// Complete blueprint for an enemy type
#[derive(Debug, Clone)]
pub struct EnemyBlueprint {
    /// Unique identifier
    pub id: EnemyId,
    /// Display name
    pub name: &'static str,
    /// Base stats
    pub stats: EnemyStats,
    /// Movement behavior
    pub movement: MovementBehavior,
    /// Attack behavior
    pub attack: AttackBehavior,
    /// Optional traits/modifiers
    pub traits: EnemyTraits,
    /// Visual configuration
    pub visuals: EnemyVisuals,
}

impl EnemyBlueprint {
    /// Get the blueprint for a given enemy ID
    pub fn get(id: EnemyId) -> Self {
        match id {
            EnemyId::Slime => slime_blueprint(),
        }
    }

    /// Calculate scaled HP based on wave/level
    pub fn scaled_hp(&self, level: i32) -> i32 {
        // Base HP + 50 per level (can be customized per enemy)
        self.stats.base_hp + level * 50
    }
}

// ============================================================================
// Enemy Definitions - Add new enemies here!
// ============================================================================

/// Slime - Basic enemy with random movement and projectile attack
fn slime_blueprint() -> EnemyBlueprint {
    EnemyBlueprint {
        id: EnemyId::Slime,
        name: "Slime",
        stats: EnemyStats {
            base_hp: 100,
            contact_damage: 10,
            move_speed: 1.0,
            attack_speed: 0.5,
        },
        movement: MovementBehavior::Random { idle_chance: 0.33 },
        attack: AttackBehavior::Projectile {
            damage: 10,
            speed: 4.0, // tiles per second
            charge_time: 0.5,
        },
        traits: EnemyTraits::default(),
        visuals: EnemyVisuals {
            sprite_path: "enemies/slime".into(),
            draw_size: Vec2::new(128.0, 128.0),
            anchor: Vec2::new(0.0, -0.40),
            offset: Vec2::new(0.0, -8.0),
            flip_x: true,
            animations: EnemyAnimations {
                idle_grid: (3, 3),
                attack_grid: Some((3, 4)),
                hurt_grid: None,
                dead_grid: Some((3, 3)),

                idle_frames: 7,
                attack_frames: 10,
                hurt_frames: 0,
                dead_frames: 7,

                idle_fps: 8.0,
                attack_fps: 12.0,
                hurt_fps: 10.0,
                dead_fps: 10.0,

                idle_file: "IDLE - WALK.png".into(),
                attack_file: Some("SHOOTING.png".into()),
                hurt_file: None,
                dead_file: Some("DEAD.png".into()),
            },
        },
    }
}

// ============================================================================
// Example blueprints for future enemies (commented out)
// ============================================================================

/*
/// Mettaur - Hides under helmet, peeks out to attack with shockwave
fn mettaur_blueprint() -> EnemyBlueprint {
    EnemyBlueprint {
        id: EnemyId::Mettaur,
        name: "Mettaur",
        stats: EnemyStats {
            base_hp: 40,
            contact_damage: 10,
            move_speed: 0.8,
            attack_speed: 1.0,
        },
        movement: MovementBehavior::HideAndPeek {
            hide_duration: 2.0,
            peek_duration: 1.5,
        },
        attack: AttackBehavior::ShockWave {
            damage: 20,
            speed: 6.0,
            charge_time: 0.3,
        },
        traits: EnemyTraits {
            // Invulnerable while hidden (handled by HideAndPeek behavior)
            ..default()
        },
        visuals: EnemyVisuals {
            sprite_path: "enemies/mettaur".into(),
            draw_size: Vec2::new(96.0, 96.0),
            anchor: Vec2::new(0.0, -0.35),
            offset: Vec2::ZERO,
            flip_x: true,
            animations: EnemyAnimations::default(),
        },
    }
}

/// Canodumb - Stationary turret that shoots when player is in its row
fn canodumb_blueprint() -> EnemyBlueprint {
    EnemyBlueprint {
        id: EnemyId::Canodumb,
        name: "Canodumb",
        stats: EnemyStats {
            base_hp: 60,
            contact_damage: 0,
            move_speed: 0.0,
            attack_speed: 1.2,
        },
        movement: MovementBehavior::Stationary,
        attack: AttackBehavior::Projectile {
            damage: 15,
            speed: 10.0,
            charge_time: 0.2,
        },
        traits: EnemyTraits::default(),
        visuals: EnemyVisuals::default(),
    }
}

/// Swordy - Teleports next to player and slashes
fn swordy_blueprint() -> EnemyBlueprint {
    EnemyBlueprint {
        id: EnemyId::Swordy,
        name: "Swordy",
        stats: EnemyStats {
            base_hp: 80,
            contact_damage: 15,
            move_speed: 1.5,
            attack_speed: 1.0,
        },
        movement: MovementBehavior::Teleport {
            min_interval: 1.5,
            max_interval: 3.0,
        },
        attack: AttackBehavior::Melee {
            damage: 30,
            range: 1,
            charge_time: 0.4,
        },
        traits: EnemyTraits::default(),
        visuals: EnemyVisuals::default(),
    }
}

/// Bunny - Fast enemy that hops around, splits into smaller bunnies on death
fn bunny_blueprint() -> EnemyBlueprint {
    EnemyBlueprint {
        id: EnemyId::Bunny,
        name: "Bunny",
        stats: EnemyStats {
            base_hp: 30,
            contact_damage: 5,
            move_speed: 2.0,
            attack_speed: 0.5,
        },
        movement: MovementBehavior::ChasePlayer,
        attack: AttackBehavior::None, // Contact damage only
        traits: EnemyTraits {
            death_spawn: Some(DeathSpawn {
                enemy_id: "mini_bunny".into(),
                count: 2,
            }),
            ..default()
        },
        visuals: EnemyVisuals::default(),
    }
}
*/
