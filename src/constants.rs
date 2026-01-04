use bevy::prelude::*;

// Grid layout
pub const GRID_WIDTH: i32 = 6;
pub const GRID_HEIGHT: i32 = 3;
pub const PLAYER_AREA_WIDTH: i32 = 3;

// Panel geometry (trapezoids)
pub const TILE_W: f32 = 90.0;
pub const TILE_H: f32 = 48.0;
pub const TILE_STEP_X: f32 = 92.0;
pub const TILE_STEP_Y: f32 = 50.0;
pub const ROW_SKEW_X: f32 = 14.0;
pub const TILE_TOP_INSET: f32 = 12.0;

// Sprite alignment
// Our fighter frames are 128x128 but contain bottom padding.
// The actual feet are above the bottom edge, so we use a custom anchor.
// `Anchor::Custom(Vec2::new(0.0, -0.5))` would equal BottomCenter.
// Raising to about -0.36 compensates for ~18px padding.
pub const FIGHTER_ANCHOR: Vec2 = Vec2::new(0.0, -0.36);

// Entity offsets are relative to `tile_floor_world`.
pub const CHARACTER_OFFSET: Vec2 = Vec2::new(0.0, 0.0);

// Bullets are lane-based: they always spawn from the tile the player is standing on.
// This offset is relative to the tile floor point.
pub const BULLET_OFFSET: Vec2 = Vec2::new(70.0, 70.0);
pub const BULLET_MOVE_TIMER: f32 = 0.06; // seconds per tile
pub const BULLET_DRAW_SIZE: Vec2 = Vec2::new(12.0, 12.0);

// Optional: purely visual muzzle flash offset (tile-based as well)
pub const MUZZLE_OFFSET: Vec2 = Vec2::new(86.0, 70.0);

// Fighter rendering
pub const FIGHTER_DRAW_SIZE: Vec2 = Vec2::new(180.0, 180.0);

// Z layers
pub const Z_BACKGROUND: f32 = -10.0;
pub const Z_GRID_SHADOW: f32 = -1.0;
pub const Z_GRID: f32 = 0.0;
pub const Z_CHARACTER: f32 = 10.0;
pub const Z_BULLET: f32 = 12.0;
pub const Z_UI: f32 = 20.0;

// Render tuning
pub const DEPTH_Y_TO_Z: f32 = 0.0015;

// Colors (rough BN-inspired palette)
pub const COLOR_BACKGROUND: Color = Color::srgb(0.09, 0.05, 0.18);

pub const COLOR_PLAYER_PANEL: Color = Color::srgb(0.86, 0.28, 0.30);
pub const COLOR_ENEMY_PANEL: Color = Color::srgb(0.30, 0.58, 0.92);

pub const COLOR_PANEL_BORDER: Color = Color::srgb(0.78, 0.78, 0.82);
pub const COLOR_PANEL_SHADOW: Color = Color::srgba(0.0, 0.0, 0.0, 0.25);
pub const COLOR_PANEL_SHINE: Color = Color::srgba(1.0, 1.0, 1.0, 0.12);

pub const COLOR_PLAYER: Color = Color::srgb(0.2, 0.85, 0.25);
pub const COLOR_ENEMY: Color = Color::srgb(0.82, 0.2, 0.86);

pub const COLOR_BULLET: Color = Color::srgb(1.0, 0.95, 0.2);
pub const COLOR_MUZZLE: Color = Color::srgba(1.0, 0.7, 0.2, 0.9);

pub const COLOR_TEXT: Color = Color::WHITE;
pub const COLOR_TEXT_SHADOW: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);
pub const COLOR_HP_PLATE: Color = Color::srgba(0.0, 0.0, 0.0, 0.35);

// Gameplay
pub const SHOOT_COOLDOWN: f32 = 0.12;
pub const MOVE_COOLDOWN: f32 = 0.15;

// Bullet collision is tile-based (no radius needed)

pub const FLASH_TIME: f32 = 0.08;
pub const MUZZLE_TIME: f32 = 0.06;
