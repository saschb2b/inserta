// ============================================================================
// Enemy Behaviors - The LEGO blocks for enemy AI
// ============================================================================

use bevy::prelude::*;

// ============================================================================
// Movement Behaviors
// ============================================================================

/// How an enemy moves around the arena
#[derive(Debug, Clone)]
pub enum MovementBehavior {
    /// Doesn't move at all (turrets, stationary enemies)
    Stationary,

    /// Random movement within territory (current slime behavior)
    Random {
        /// Chance to stay in place (0.0-1.0)
        idle_chance: f32,
    },

    /// Moves toward the player's row
    ChaseRow,

    /// Moves toward the player (both X and Y, stays in territory)
    ChasePlayer,

    /// Patrols back and forth horizontally
    PatrolHorizontal,

    /// Patrols up and down vertically  
    PatrolVertical,

    /// Hides, then peeks out to attack
    HideAndPeek {
        hide_duration: f32,
        peek_duration: f32,
    },

    /// Teleports to random positions
    Teleport {
        /// Minimum time between teleports
        min_interval: f32,
        /// Maximum time between teleports
        max_interval: f32,
    },

    /// Stays at back row, only moves vertically
    BackRowOnly,

    /// Mirrors player's Y position
    MirrorPlayer,

    /// Advances one column toward player periodically
    Advance {
        /// Columns to advance before retreating
        max_advance: i32,
    },
}

impl Default for MovementBehavior {
    fn default() -> Self {
        MovementBehavior::Random { idle_chance: 0.33 }
    }
}

impl MovementBehavior {
    /// Get the base movement cooldown for this behavior
    pub fn base_cooldown(&self) -> f32 {
        match self {
            MovementBehavior::Stationary => f32::MAX,
            MovementBehavior::Random { .. } => 1.2,
            MovementBehavior::ChaseRow => 1.0,
            MovementBehavior::ChasePlayer => 0.8,
            MovementBehavior::PatrolHorizontal => 1.0,
            MovementBehavior::PatrolVertical => 1.0,
            MovementBehavior::HideAndPeek { .. } => 0.5,
            MovementBehavior::Teleport { min_interval, .. } => *min_interval,
            MovementBehavior::BackRowOnly => 1.5,
            MovementBehavior::MirrorPlayer => 0.3,
            MovementBehavior::Advance { .. } => 2.0,
        }
    }
}

// ============================================================================
// Attack Behaviors
// ============================================================================

/// How an enemy attacks
#[derive(Debug, Clone)]
pub enum AttackBehavior {
    /// Doesn't attack (used with contact damage only)
    None,

    /// Fires a single projectile in a direction
    Projectile {
        damage: i32,
        /// Projectile speed (tiles per second, higher = faster)
        speed: f32,
        /// Charge time before firing (for telegraph)
        charge_time: f32,
    },

    /// Fires multiple projectiles in a spread
    ProjectileSpread {
        damage: i32,
        speed: f32,
        charge_time: f32,
        /// Number of projectiles
        count: i32,
        /// Rows to target (relative to enemy, e.g., [-1, 0, 1] for 3-way)
        row_offsets: Vec<i32>,
    },

    /// Fires a projectile that travels along the ground (shockwave)
    ShockWave {
        damage: i32,
        speed: f32,
        charge_time: f32,
    },

    /// Melee attack on adjacent tile
    Melee {
        damage: i32,
        /// Reach in tiles (1 = adjacent only)
        range: i32,
        charge_time: f32,
    },

    /// Area attack hitting multiple tiles
    AreaAttack {
        damage: i32,
        charge_time: f32,
        /// Tiles to hit relative to enemy position
        pattern: Vec<(i32, i32)>,
    },

    /// Drops a bomb that explodes after delay
    Bomb {
        damage: i32,
        /// Time until explosion
        fuse_time: f32,
        /// Explosion radius in tiles
        radius: i32,
    },

    /// Laser beam that hits entire row instantly
    LaserBeam {
        damage: i32,
        charge_time: f32,
        /// Duration the beam stays active
        duration: f32,
    },

    /// Summons other enemies
    Summon {
        /// Enemy type to summon (by ID string for now)
        summon_id: String,
        /// Max summons alive at once
        max_summons: i32,
        charge_time: f32,
    },
}

impl Default for AttackBehavior {
    fn default() -> Self {
        AttackBehavior::Projectile {
            damage: 10,
            speed: 8.0,
            charge_time: 0.5,
        }
    }
}

impl AttackBehavior {
    /// Get the base attack cooldown for this behavior
    pub fn base_cooldown(&self) -> f32 {
        match self {
            AttackBehavior::None => f32::MAX,
            AttackBehavior::Projectile { .. } => 2.0,
            AttackBehavior::ProjectileSpread { .. } => 3.0,
            AttackBehavior::ShockWave { .. } => 2.5,
            AttackBehavior::Melee { .. } => 1.5,
            AttackBehavior::AreaAttack { .. } => 3.0,
            AttackBehavior::Bomb { .. } => 4.0,
            AttackBehavior::LaserBeam { .. } => 5.0,
            AttackBehavior::Summon { .. } => 8.0,
        }
    }

    /// Get the charge time for telegraph
    pub fn charge_time(&self) -> f32 {
        match self {
            AttackBehavior::None => 0.0,
            AttackBehavior::Projectile { charge_time, .. } => *charge_time,
            AttackBehavior::ProjectileSpread { charge_time, .. } => *charge_time,
            AttackBehavior::ShockWave { charge_time, .. } => *charge_time,
            AttackBehavior::Melee { charge_time, .. } => *charge_time,
            AttackBehavior::AreaAttack { charge_time, .. } => *charge_time,
            AttackBehavior::Bomb { .. } => 0.3,
            AttackBehavior::LaserBeam { charge_time, .. } => *charge_time,
            AttackBehavior::Summon { charge_time, .. } => *charge_time,
        }
    }

    /// Get the damage for this attack
    pub fn damage(&self) -> i32 {
        match self {
            AttackBehavior::None => 0,
            AttackBehavior::Projectile { damage, .. } => *damage,
            AttackBehavior::ProjectileSpread { damage, .. } => *damage,
            AttackBehavior::ShockWave { damage, .. } => *damage,
            AttackBehavior::Melee { damage, .. } => *damage,
            AttackBehavior::AreaAttack { damage, .. } => *damage,
            AttackBehavior::Bomb { damage, .. } => *damage,
            AttackBehavior::LaserBeam { damage, .. } => *damage,
            AttackBehavior::Summon { .. } => 0,
        }
    }
}

// ============================================================================
// Enemy Traits - Optional modifiers
// ============================================================================

/// Collection of traits that modify enemy behavior
#[derive(Debug, Clone, Default)]
pub struct EnemyTraits {
    /// Flat damage reduction on all incoming damage
    pub armor: i32,

    /// HP regeneration per second
    pub hp_regen_per_sec: f32,

    /// Immune to flinching/knockback
    pub super_armor: bool,

    /// Takes reduced damage from elemental attacks (0.0-1.0 = reduction %)
    pub elemental_resist: f32,

    /// Explodes on death dealing damage
    pub death_explosion: Option<DeathExplosion>,

    /// Spawns minions on death
    pub death_spawn: Option<DeathSpawn>,

    /// Enrages (faster/stronger) when below HP threshold
    pub enrage: Option<EnrageThreshold>,

    /// Becomes invulnerable periodically
    pub phase_immunity: Option<PhaseImmunity>,
}

#[derive(Debug, Clone)]
pub struct DeathExplosion {
    pub damage: i32,
    pub radius: i32,
}

#[derive(Debug, Clone)]
pub struct DeathSpawn {
    pub enemy_id: String,
    pub count: i32,
}

#[derive(Debug, Clone)]
pub struct EnrageThreshold {
    /// HP percentage to trigger enrage (0.0-1.0)
    pub threshold: f32,
    /// Attack speed multiplier when enraged
    pub attack_speed_mult: f32,
    /// Move speed multiplier when enraged
    pub move_speed_mult: f32,
}

#[derive(Debug, Clone)]
pub struct PhaseImmunity {
    /// Duration of immunity phase
    pub immune_duration: f32,
    /// Duration of vulnerable phase
    pub vulnerable_duration: f32,
}
