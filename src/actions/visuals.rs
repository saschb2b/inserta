// ============================================================================
// Action Visuals - Visual configuration for actions
// ============================================================================

use bevy::prelude::*;

/// Visual configuration for an action
#[derive(Debug, Clone)]
pub struct ActionVisuals {
    /// Color for the action icon in UI
    pub icon_color: Color,

    /// Color for the main effect (slash, projectile, etc.)
    pub effect_color: Color,

    /// Optional sprite path for the effect
    pub effect_sprite: Option<String>,

    /// Size of the visual effect
    pub effect_size: Vec2,

    /// How long the visual lasts
    pub effect_duration: f32,

    /// Flash color when action is used
    pub flash_color: Option<Color>,

    /// Whether the effect has animation frames
    pub animated: bool,
}

impl Default for ActionVisuals {
    fn default() -> Self {
        Self {
            icon_color: Color::WHITE,
            effect_color: Color::WHITE,
            effect_sprite: None,
            effect_size: Vec2::new(64.0, 64.0),
            effect_duration: 0.25,
            flash_color: None,
            animated: false,
        }
    }
}

impl ActionVisuals {
    /// Create visuals with just an icon color
    pub fn icon_only(color: Color) -> Self {
        Self {
            icon_color: color,
            ..default()
        }
    }

    /// Create visuals for a sword slash
    pub fn sword_slash(icon_color: Color, slash_color: Color) -> Self {
        Self {
            icon_color,
            effect_color: slash_color,
            effect_size: Vec2::new(80.0, 200.0),
            effect_duration: 0.25,
            ..default()
        }
    }

    /// Create visuals for a projectile
    pub fn projectile(icon_color: Color, proj_color: Color) -> Self {
        Self {
            icon_color,
            effect_color: proj_color,
            effect_size: Vec2::new(48.0, 48.0),
            effect_duration: 2.0, // Longer for travel time
            ..default()
        }
    }

    /// Create visuals for a healing effect
    pub fn heal(icon_color: Color, flash: Color) -> Self {
        Self {
            icon_color,
            effect_color: flash,
            effect_size: Vec2::ZERO, // No projectile
            effect_duration: 0.3,
            flash_color: Some(flash),
            ..default()
        }
    }

    /// Create visuals for a shield effect
    pub fn shield(icon_color: Color, shield_color: Color) -> Self {
        Self {
            icon_color,
            effect_color: shield_color,
            effect_size: Vec2::new(120.0, 160.0),
            effect_duration: 0.0, // Stays until shield expires
            ..default()
        }
    }

    /// Create visuals for an explosion
    pub fn explosion(icon_color: Color, explosion_color: Color, size: Vec2) -> Self {
        Self {
            icon_color,
            effect_color: explosion_color,
            effect_size: size,
            effect_duration: 0.4,
            flash_color: Some(Color::WHITE),
            ..default()
        }
    }
}

// ============================================================================
// Preset Colors (MMBN style palette)
// ============================================================================

pub mod colors {
    use bevy::prelude::Color;

    // Recovery
    pub const HEAL_GREEN: Color = Color::srgb(0.3, 0.9, 0.4);

    // Shields
    pub const SHIELD_BLUE: Color = Color::srgba(0.3, 0.6, 1.0, 0.5);
    pub const BARRIER_CYAN: Color = Color::srgba(0.2, 0.9, 0.9, 0.6);
    pub const AURA_GOLD: Color = Color::srgba(1.0, 0.85, 0.2, 0.5);

    // Swords
    pub const SWORD_WHITE: Color = Color::srgb(0.95, 0.95, 1.0);
    pub const SWORD_PINK: Color = Color::srgba(1.0, 0.4, 0.6, 0.8);
    pub const SWORD_FIRE: Color = Color::srgb(1.0, 0.4, 0.2);
    pub const SWORD_AQUA: Color = Color::srgb(0.2, 0.6, 1.0);
    pub const SWORD_ELEC: Color = Color::srgb(1.0, 1.0, 0.3);

    // Cannons
    pub const CANNON_YELLOW: Color = Color::srgb(1.0, 0.95, 0.2);
    pub const CANNON_ORANGE: Color = Color::srgb(1.0, 0.6, 0.1);

    // Bombs
    pub const BOMB_RED: Color = Color::srgb(0.9, 0.2, 0.2);
    pub const BOMB_ORANGE: Color = Color::srgb(1.0, 0.5, 0.1);

    // Elements
    pub const FIRE: Color = Color::srgb(1.0, 0.3, 0.1);
    pub const AQUA: Color = Color::srgb(0.2, 0.5, 1.0);
    pub const ELEC: Color = Color::srgb(1.0, 1.0, 0.2);
    pub const WOOD: Color = Color::srgb(0.2, 0.8, 0.3);

    // Waves/Ground
    pub const WAVE_GRAY: Color = Color::srgb(0.7, 0.7, 0.75);
    pub const WAVE_YELLOW: Color = Color::srgb(1.0, 0.9, 0.4);
}
