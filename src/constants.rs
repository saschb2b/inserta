use bevy::prelude::*;

// ============================================================================
// Screen Layout (1280x800 target resolution)
// ============================================================================
// Screen center is (0, 0)
// Top-left: (-640, 400), Bottom-right: (640, -400)
// Arena is centered but shifted up slightly to make room for action bar

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 800.0;

// Arena vertical offset (shift up to make room for action bar)
pub const ARENA_Y_OFFSET: f32 = 40.0;

// Grid layout
pub const GRID_WIDTH: i32 = 6;
pub const GRID_HEIGHT: i32 = 3;
pub const PLAYER_AREA_WIDTH: i32 = 3;

// Panel geometry (flat top-down MMBN style)
// Scaled up for 1280x800 - arena should be ~1000px wide, ~300px tall
pub const TILE_W: f32 = 160.0;
pub const TILE_H: f32 = 96.0;
pub const TILE_STEP_X: f32 = 164.0;
pub const TILE_STEP_Y: f32 = 100.0;
pub const ROW_SKEW_X: f32 = 0.0; // No skew for MMBN flat style

// Panel 3D depth effect
pub const PANEL_DEPTH: f32 = 16.0;

// Sprite alignment
// Anchor is relative to sprite center: -0.5 = bottom, 0 = center, 0.5 = top
// The fighter sprite has some padding at the bottom, so we adjust slightly
pub const FIGHTER_ANCHOR: Vec2 = Vec2::new(0.0, -0.25);

// Entity offsets (relative to tile floor point)
// Negative Y moves the character down so feet align with panel bottom
pub const CHARACTER_OFFSET: Vec2 = Vec2::new(0.0, 0.0);

// Bullets (scaled up proportionally)
pub const BULLET_OFFSET: Vec2 = Vec2::new(110.0, 110.0);
// MMBN speed: ~6-8 frames per tile at 60 FPS (approx 0.10 - 0.133s)
// We use 0.12s (approx 7.2 frames) for parity with standard attacks
pub const BULLET_MOVE_TIMER: f32 = 0.12;
pub const BULLET_DRAW_SIZE: Vec2 = Vec2::new(18.0, 18.0);

// Muzzle flash
pub const MUZZLE_OFFSET: Vec2 = Vec2::new(135.0, 110.0);

// Fighter rendering
// Scale to be roughly 2 panels tall (like in MMBN)
pub const FIGHTER_DRAW_SIZE: Vec2 = Vec2::new(340.0, 340.0);

// Slime enemy rendering (16x16 base sprites, scaled up)
pub const SLIME_DRAW_SIZE: Vec2 = Vec2::new(128.0, 128.0);
pub const SLIME_ANCHOR: Vec2 = Vec2::new(0.0, -0.40);
pub const SLIME_OFFSET: Vec2 = Vec2::new(0.0, -8.0);
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
pub const COLOR_GRID_LINE: Color = Color::srgba(0.15, 0.25, 0.5, 0.12); // was 0.2
pub const COLOR_GRID_LINE_BRIGHT: Color = Color::srgba(0.25, 0.45, 0.9, 0.20); // was 0.35

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

// Bullet trail highlight (yellow glow on tiles)
pub const COLOR_BULLET_HIGHLIGHT: Color = Color::srgba(1.0, 0.9, 0.3, 0.5);

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
pub const SHOOT_COOLDOWN: f32 = 0.35; // Player shoot cooldown
pub const MOVE_COOLDOWN: f32 = 0.15;

// Enemy AI timing
pub const ENEMY_MOVE_COOLDOWN: f32 = 1.2; // Time between enemy moves
pub const ENEMY_SHOOT_COOLDOWN: f32 = 2.0; // Time between enemy shots
pub const ENEMY_CHARGE_TIME: f32 = 0.5; // Startup telegraph before shooting

pub const FLASH_TIME: f32 = 0.08;
pub const MUZZLE_TIME: f32 = 0.06;

// Damage
pub const PLAYER_DAMAGE: i32 = 1; // Damage player deals
pub const ENEMY_DAMAGE: i32 = 10; // Damage enemy deals

// ============================================================================
// Action System
// ============================================================================

// Charged Shot action
pub const CHARGED_SHOT_COOLDOWN: f32 = 3.0; // Cooldown after use
pub const CHARGED_SHOT_CHARGE_TIME: f32 = 0.8; // Time to charge up
pub const CHARGED_SHOT_DAMAGE: i32 = 25; // Damage dealt
pub const CHARGED_SHOT_SIZE: Vec2 = Vec2::new(40.0, 40.0);
pub const COLOR_CHARGED_SHOT: Color = Color::srgb(1.0, 0.5, 0.1); // Orange

// Heal action
pub const HEAL_COOLDOWN: f32 = 8.0; // Longer cooldown for heal
pub const HEAL_CHARGE_TIME: f32 = 0.0; // Instant cast
pub const HEAL_AMOUNT: i32 = 20; // HP restored

// Shield action
pub const SHIELD_COOLDOWN: f32 = 6.0; // Cooldown after shield expires
pub const SHIELD_CHARGE_TIME: f32 = 0.0; // Instant activation
pub const SHIELD_DURATION: f32 = 2.0; // Duration of invulnerability

// WideSword action
pub const WIDESWORD_COOLDOWN: f32 = 4.0; // Cooldown after use
pub const WIDESWORD_CHARGE_TIME: f32 = 0.3; // Quick charge for melee
pub const WIDESWORD_DAMAGE: i32 = 40; // High damage melee attack
pub const WIDESWORD_SLASH_DURATION: f32 = 0.25; // Visual slash duration

// Action Bar UI
pub const ACTION_BAR_Y: f32 = -340.0; // Bottom of screen (800/2 - 60 margin)
pub const ACTION_SLOT_SIZE: f32 = 56.0; // Size of each slot
pub const ACTION_SLOT_SPACING: f32 = 12.0; // Gap between slots
pub const ACTION_SLOT_COUNT: usize = 4; // Max slots (only 2 used for now)

// Action slot colors
pub const COLOR_ACTION_SLOT_BG: Color = Color::srgba(0.1, 0.1, 0.2, 0.85);
pub const COLOR_ACTION_SLOT_BORDER: Color = Color::srgb(0.4, 0.4, 0.6);
pub const COLOR_ACTION_SLOT_READY: Color = Color::srgb(0.3, 0.7, 0.3);
pub const COLOR_ACTION_COOLDOWN: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);
pub const COLOR_ACTION_CHARGE: Color = Color::srgb(1.0, 0.8, 0.2);
pub const COLOR_ACTION_KEY_TEXT: Color = Color::srgb(0.9, 0.9, 0.9);

// Action icons (using colored squares for now, can be replaced with sprites later)
pub const COLOR_CHARGED_SHOT_ICON: Color = Color::srgb(1.0, 0.5, 0.1);
pub const COLOR_HEAL_ICON: Color = Color::srgb(0.3, 0.9, 0.4);
pub const COLOR_SHIELD_ICON: Color = Color::srgb(0.3, 0.6, 1.0); // Blue shield
pub const COLOR_WIDESWORD_ICON: Color = Color::srgb(0.9, 0.3, 0.5); // Pink/red sword

// Shield visual
pub const COLOR_SHIELD: Color = Color::srgba(0.3, 0.6, 1.0, 0.5); // Semi-transparent blue

// WideSword visual
pub const COLOR_WIDESWORD_SLASH: Color = Color::srgba(1.0, 0.4, 0.6, 0.8); // Pink slash
