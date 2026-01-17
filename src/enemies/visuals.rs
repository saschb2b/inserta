// ============================================================================
// Enemy Visuals - Sprite and animation configuration
// ============================================================================

use bevy::prelude::*;

/// Visual configuration for an enemy type
#[derive(Debug, Clone)]
pub struct EnemyVisuals {
    /// Base sprite sheet path (relative to assets/)
    pub sprite_path: String,
    /// Size to draw the sprite at
    pub draw_size: Vec2,
    /// Anchor point adjustment
    pub anchor: Vec2,
    /// Position offset from tile
    pub offset: Vec2,
    /// Whether to flip sprite to face player (most enemies face left)
    pub flip_x: bool,
    /// Animation configuration
    pub animations: EnemyAnimations,
}

impl Default for EnemyVisuals {
    fn default() -> Self {
        Self {
            sprite_path: "enemies/slime".into(),
            draw_size: Vec2::new(128.0, 128.0),
            anchor: Vec2::new(0.0, -0.40),
            offset: Vec2::new(0.0, -8.0),
            flip_x: true,
            animations: EnemyAnimations::default(),
        }
    }
}

/// Animation configuration for sprite sheets
#[derive(Debug, Clone)]
pub struct EnemyAnimations {
    /// Grid size for texture atlas (columns, rows)
    pub idle_grid: (u32, u32),
    pub attack_grid: Option<(u32, u32)>,
    pub hurt_grid: Option<(u32, u32)>,
    pub dead_grid: Option<(u32, u32)>,

    /// Frame counts for each animation
    pub idle_frames: usize,
    pub attack_frames: usize,
    pub hurt_frames: usize,
    pub dead_frames: usize,

    /// FPS for each animation
    pub idle_fps: f32,
    pub attack_fps: f32,
    pub hurt_fps: f32,
    pub dead_fps: f32,

    /// Sprite file names (relative to sprite_path)
    pub idle_file: String,
    pub attack_file: Option<String>,
    pub hurt_file: Option<String>,
    pub dead_file: Option<String>,
}

impl Default for EnemyAnimations {
    fn default() -> Self {
        Self {
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
        }
    }
}

/// Resource to hold loaded sprite handles for an enemy type
#[derive(Debug, Clone)]
pub struct LoadedEnemySprites {
    pub idle: Handle<Image>,
    pub idle_layout: Handle<TextureAtlasLayout>,
    pub attack: Option<Handle<Image>>,
    pub attack_layout: Option<Handle<TextureAtlasLayout>>,
    pub hurt: Option<Handle<Image>>,
    pub hurt_layout: Option<Handle<TextureAtlasLayout>>,
    pub dead: Option<Handle<Image>>,
    pub dead_layout: Option<Handle<TextureAtlasLayout>>,
    pub config: EnemyAnimations,
}
