// ============================================================================
// Action Behaviors - The building blocks for action effects
// ============================================================================
//
// These enums define WHAT an action does and HOW it targets.
// Combine them in blueprints to create unique actions.

use super::Element;

// ============================================================================
// Targeting - WHERE does the action affect?
// ============================================================================

/// How an action selects its targets
#[derive(Debug, Clone)]
pub enum ActionTarget {
    /// Affects the user only (heals, buffs)
    OnSelf,

    /// Affects a single tile in front of user
    SingleTile {
        /// Offset from user position (positive = toward enemy)
        range: i32,
    },

    /// Affects an entire column (like WideSword)
    Column {
        /// Which column relative to user (positive = toward enemy)
        x_offset: i32,
    },

    /// Affects an entire row (like shockwave)
    Row {
        /// Starting position offset
        x_offset: i32,
        /// Whether it travels or hits instantly
        traveling: bool,
    },

    /// Affects multiple specific tiles in a pattern
    Pattern {
        /// Tiles relative to user position
        tiles: Vec<(i32, i32)>,
    },

    /// Projectile that travels forward
    Projectile {
        /// Starting offset from user
        x_offset: i32,
        /// Whether it pierces through enemies
        piercing: bool,
    },

    /// Projectile spread (like Shotgun)
    ProjectileSpread {
        x_offset: i32,
        /// Additional rows to hit (e.g., [-1, 0, 1] for 3-way)
        spread_rows: Vec<i32>,
    },

    /// Area around the user
    AreaAroundSelf {
        /// Radius in tiles
        radius: i32,
    },

    /// Area around target position
    AreaAtPosition {
        /// Offset from user
        x_offset: i32,
        y_offset: i32,
        /// Pattern of tiles relative to center
        pattern: Vec<(i32, i32)>,
    },

    /// Entire enemy area
    EnemyArea,

    /// Random tile(s) in enemy area
    RandomEnemy {
        /// Number of random targets
        count: i32,
    },
}

impl Default for ActionTarget {
    fn default() -> Self {
        ActionTarget::SingleTile { range: 1 }
    }
}

// ============================================================================
// Effects - WHAT does the action do?
// ============================================================================

/// The primary effect of an action
#[derive(Debug, Clone)]
pub enum ActionEffect {
    /// Deals damage to targets
    Damage {
        /// Base damage value
        amount: i32,
        /// Element type (for weaknesses)
        element: Element,
        /// Whether it can crit
        can_crit: bool,
        /// Whether it breaks guards/shields
        guard_break: bool,
    },

    /// Heals the user
    Heal { amount: i32 },

    /// Creates a protective shield
    Shield {
        /// Duration in seconds
        duration: f32,
        /// Damage threshold (None = blocks all, Some(n) = blocks damage < n)
        threshold: Option<i32>,
    },

    /// Makes user invisible/invincible
    Invisibility { duration: f32 },

    /// Steals enemy panel(s)
    StealPanel {
        /// Number of columns to steal
        columns: i32,
    },

    /// Cracks/destroys panels
    CrackPanel {
        /// Whether to crack (true) or destroy (false)
        crack_only: bool,
    },

    /// Repairs panels
    RepairPanel,

    /// Pushes targets back
    Knockback {
        /// Tiles to push
        distance: i32,
    },

    /// Stuns targets
    Stun {
        /// Duration in seconds
        duration: f32,
    },

    /// Drains HP from target to user
    Drain { amount: i32 },

    /// Multi-hit attack
    MultiHit {
        /// Damage per hit
        damage_per_hit: i32,
        /// Number of hits
        hit_count: i32,
        /// Element
        element: Element,
    },

    /// Delayed effect (bombs)
    Delayed {
        /// Time until effect triggers
        delay: f32,
        /// The effect to trigger
        effect: Box<ActionEffect>,
    },

    /// Combined effects (e.g., damage + heal)
    Combo { effects: Vec<ActionEffect> },
}

impl Default for ActionEffect {
    fn default() -> Self {
        ActionEffect::Damage {
            amount: 10,
            element: Element::None,
            can_crit: false,
            guard_break: false,
        }
    }
}

impl ActionEffect {
    /// Helper to create a simple damage effect
    pub fn damage(amount: i32) -> Self {
        ActionEffect::Damage {
            amount,
            element: Element::None,
            can_crit: false,
            guard_break: false,
        }
    }

    /// Helper to create elemental damage
    pub fn elemental_damage(amount: i32, element: Element) -> Self {
        ActionEffect::Damage {
            amount,
            element,
            can_crit: false,
            guard_break: false,
        }
    }

    /// Helper to create heal effect
    pub fn heal(amount: i32) -> Self {
        ActionEffect::Heal { amount }
    }

    /// Helper to create basic shield
    pub fn shield(duration: f32) -> Self {
        ActionEffect::Shield {
            duration,
            threshold: None,
        }
    }

    /// Helper to create aura shield (blocks damage under threshold)
    pub fn aura(duration: f32, threshold: i32) -> Self {
        ActionEffect::Shield {
            duration,
            threshold: Some(threshold),
        }
    }
}

// ============================================================================
// Secondary Effects / Modifiers
// ============================================================================

/// Secondary properties that modify the main effect
#[derive(Debug, Clone, Default)]
pub struct ActionModifiers {
    /// Damage multiplier based on missing HP (like Muramasa)
    pub missing_hp_scaling: Option<f32>,

    /// Damage based on remaining HP
    pub current_hp_scaling: Option<f32>,

    /// Extra damage to specific element
    pub bonus_vs_element: Option<(Element, f32)>,

    /// Ignores target's defense/armor
    pub ignore_defense: bool,

    /// Breaks guards/shields
    pub guard_break: bool,

    /// Can hit invisible enemies
    pub hits_invis: bool,

    /// Destroys obstacles in path
    pub destroys_obstacles: bool,

    /// Flickers/phases through attacks
    pub phasing: bool,

    /// Random chance to instant-delete (like MagicMan)
    pub instant_delete_chance: Option<f32>,
}
