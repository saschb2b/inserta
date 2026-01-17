use bevy::image::TextureAtlasLayout;
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct FighterSprites {
    pub layout: Handle<TextureAtlasLayout>,
    pub idle: Handle<Image>,
    pub walk: Handle<Image>,
    pub shoot: Handle<Image>,
    pub idle_frames: usize,
    pub walk_frames: usize,
    pub shoot_frames: usize,
}

#[derive(Resource, Clone)]
pub struct SlimeSprites {
    pub layout: Handle<TextureAtlasLayout>, // For idle/dead (3x3 grid)
    pub shoot_layout: Handle<TextureAtlasLayout>, // For shooting (3x4 grid)
    pub idle: Handle<Image>,
    pub shoot: Handle<Image>,
    pub dead: Handle<Image>,
    pub idle_frames: usize,
    pub shoot_frames: usize,
    pub dead_frames: usize,
}

#[derive(Resource, Clone)]
pub struct ProjectileSprites {
    pub blaster_image: Handle<Image>,
    pub blaster_layout: Handle<TextureAtlasLayout>,
    pub blaster_charged_image: Handle<Image>,
    pub blaster_charged_layout: Handle<TextureAtlasLayout>,
}

// ============================================================================
// Projectile Animation Component
// ============================================================================

/// Tracks projectile animation state (launch, travel, impact, finish)
#[derive(Component, Clone, Debug)]
pub struct ProjectileAnimation {
    /// Frame indices: [launch, travel, impact, finish]
    pub frame_indices: [usize; 4],
    /// Current animation state
    pub state: ProjectileAnimationState,
    /// Timer for transitioning between states
    pub timer: Timer,
    /// Whether this is a charged projectile (affects sprite used)
    pub is_charged: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum ProjectileAnimationState {
    /// First frame when projectile launches
    Launch = 0,
    /// Frame while projectile is traveling
    Travel = 1,
    /// Frame on impact with enemy
    Impact = 2,
    /// Final frame before despawning
    Finish = 3,
}

impl ProjectileAnimation {
    /// Create a new projectile animation with standard 4-frame blaster layout
    pub fn blaster(is_charged: bool) -> Self {
        Self {
            frame_indices: [0, 1, 2, 3], // frames 0-3 for blaster
            state: ProjectileAnimationState::Launch,
            timer: Timer::from_seconds(0.0, TimerMode::Once),
            is_charged,
        }
    }
}
