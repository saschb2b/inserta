use bevy::prelude::*;

// ============================================================================
// Global Progression Resources
// ============================================================================

/// Tracks the player's currency
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct PlayerCurrency {
    pub zenny: u64,
}

/// Tracks the current progression level (wave/stage)
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct GameProgress {
    pub current_level: u32,
    pub enemies_defeated: u32,
}

impl GameProgress {
    pub fn next_level(&mut self) {
        self.current_level += 1;
    }
}

/// Persistent stats that can be upgraded
#[derive(Resource, Debug, Clone, Copy)]
pub struct PlayerUpgrades {
    /// Weapon base damage upgrade count
    pub damage_level: u32,
    /// Max HP upgrade count
    pub health_level: u32,
    /// Fire rate (cooldown reduction) upgrade count
    pub fire_rate_level: u32,
    /// Critical chance upgrade count
    pub crit_chance_level: u32,
}

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum WaveState {
    #[default]
    Spawning,
    Active,
    Cleared,
}

impl Default for PlayerUpgrades {
    fn default() -> Self {
        Self {
            damage_level: 0,
            health_level: 0,
            fire_rate_level: 0,
            crit_chance_level: 0,
        }
    }
}

impl PlayerUpgrades {
    // Calculation helpers for actual values

    pub fn get_bonus_damage(&self) -> i32 {
        self.damage_level as i32 // +1 damage per level
    }

    pub fn get_max_hp(&self) -> i32 {
        100 + (self.health_level as i32 * 20) // +20 HP per level
    }

    pub fn get_cooldown_modifier(&self) -> f32 {
        // 5% faster fire rate per level, capped at some reasonable amount (e.g. 50%)
        let reduction = (self.fire_rate_level as f32 * 0.05).min(0.5);
        1.0 - reduction
    }

    pub fn get_crit_chance_bonus(&self) -> f32 {
        self.crit_chance_level as f32 * 0.02 // +2% crit chance per level
    }

    // Cost calculations

    pub fn cost_damage(&self) -> u64 {
        100 * (1.5_f32.powi(self.damage_level as i32) as u64)
    }

    pub fn cost_health(&self) -> u64 {
        50 * (1.2_f32.powi(self.health_level as i32) as u64)
    }

    pub fn cost_fire_rate(&self) -> u64 {
        150 * (1.6_f32.powi(self.fire_rate_level as i32) as u64)
    }

    pub fn cost_crit_chance(&self) -> u64 {
        200 * (1.8_f32.powi(self.crit_chance_level as i32) as u64)
    }
}
