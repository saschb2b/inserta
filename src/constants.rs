use bevy::prelude::*;

// Grid layout
pub const GRID_WIDTH: i32 = 6;
pub const GRID_HEIGHT: i32 = 3;
pub const PLAYER_AREA_WIDTH: i32 = 3;

// Panel geometry (flat top-down MMBN style)
pub const TILE_W: f32 = 100.0;
pub const TILE_H: f32 = 60.0;
pub const TILE_STEP_X: f32 = 102.0;
pub const TILE_STEP_Y: f32 = 62.0;
pub const ROW_SKEW_X: f32 = 0.0; // No skew for MMBN flat style
pub const TILE_TOP_INSET: f32 = 0.0;

// Panel 3D depth effect
pub const PANEL_DEPTH: f32 = 10.0;

// Sprite alignment
pub const FIGHTER_ANCHOR: Vec2 = Vec2::new(0.0, -0.36);

// Entity offsets
pub const CHARACTER_OFFSET: Vec2 = Vec2::new(0.0, 0.0);

// Bullets
pub const BULLET_OFFSET: Vec2 = Vec2::new(70.0, 70.0);
pub const BULLET_MOVE_TIMER: f32 = 0.06;
pub const BULLET_DRAW_SIZE: Vec2 = Vec2::new(12.0, 12.0);

// Muzzle flash
pub const MUZZLE_OFFSET: Vec2 = Vec2::new(86.0, 70.0);

// Fighter rendering
pub const FIGHTER_DRAW_SIZE: Vec2 = Vec2::new(180.0, 180.0);

// Z layers
pub const Z_BACKGROUND: f32 = -10.0;
pub const Z_GRID_LINES: f32 = -5.0;
pub const Z_GRID_SHADOW: f32 = -1.0;
pub const Z_PANEL_SIDE: f32 = 0.0;
pub const Z_PANEL_TOP: f32 = 0.5;
pub const Z_PANEL_GLOW: f32 = 0.6;
pub const Z_PANEL_SHINE: f32 = 0.7;
pub const Z_CHARACTER: f32 = 10.0;
pub const Z_BULLET: f32 = 12.0;
pub const Z_UI: f32 = 20.0;

// Render tuning
pub const DEPTH_Y_TO_Z: f32 = 0.002;

// ============================================================================
// MMBN-Style Color Palette
// ============================================================================

// Background - dark cyber blue
pub const COLOR_BACKGROUND: Color = Color::srgb(0.05, 0.05, 0.15);

// Cyber grid lines
pub const COLOR_GRID_LINE: Color = Color::srgba(0.15, 0.25, 0.5, 0.2);
pub const COLOR_GRID_LINE_BRIGHT: Color = Color::srgba(0.2, 0.35, 0.7, 0.35);

// ============================================================================
// Player panels - Red/Orange (MMBN style)
// ============================================================================
// Darkest outer border
pub const COLOR_PLAYER_PANEL_DARK: Color = Color::srgb(0.45, 0.12, 0.08);
// Frame/grid lines between panels
pub const COLOR_PLAYER_PANEL_FRAME: Color = Color::srgb(0.75, 0.25, 0.15);
// Inner panel surface (brightest)
pub const COLOR_PLAYER_PANEL_TOP: Color = Color::srgb(0.92, 0.45, 0.35);
// Front face (3D depth)
pub const COLOR_PLAYER_PANEL_SIDE: Color = Color::srgb(0.55, 0.15, 0.10);

// ============================================================================
// Enemy panels - Blue (MMBN style)
// ============================================================================
// Darkest outer border
pub const COLOR_ENEMY_PANEL_DARK: Color = Color::srgb(0.08, 0.15, 0.45);
// Frame/grid lines between panels
pub const COLOR_ENEMY_PANEL_FRAME: Color = Color::srgb(0.15, 0.35, 0.75);
// Inner panel surface (brightest)
pub const COLOR_ENEMY_PANEL_TOP: Color = Color::srgb(0.35, 0.55, 0.92);
// Front face (3D depth)
pub const COLOR_ENEMY_PANEL_SIDE: Color = Color::srgb(0.10, 0.20, 0.55);

// ============================================================================
// Panel effects
// ============================================================================
pub const COLOR_PANEL_HIGHLIGHT: Color = Color::srgba(1.0, 1.0, 1.0, 0.35);
pub const COLOR_PANEL_SHADOW: Color = Color::srgba(0.0, 0.0, 0.0, 0.4);

// Characters
pub const COLOR_ENEMY: Color = Color::srgb(0.82, 0.2, 0.86);

// Combat effects
pub const COLOR_BULLET: Color = Color::srgb(1.0, 0.95, 0.2);
pub const COLOR_MUZZLE: Color = Color::srgba(1.0, 0.7, 0.2, 0.9);

// UI
pub const COLOR_TEXT: Color = Color::WHITE;
pub const COLOR_TEXT_SHADOW: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);
pub const COLOR_HP_PLATE: Color = Color::srgba(0.0, 0.0, 0.0, 0.5);

// Gameplay
pub const SHOOT_COOLDOWN: f32 = 0.12;
pub const MOVE_COOLDOWN: f32 = 0.15;

pub const FLASH_TIME: f32 = 0.08;
pub const MUZZLE_TIME: f32 = 0.06;
