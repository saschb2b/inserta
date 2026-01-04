use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct MuzzleFlash;

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct HealthText;

#[derive(Component, Clone, Copy)]
pub struct RenderConfig {
    pub offset: Vec2,
    pub base_z: f32,
}

#[derive(Component)]
pub struct MoveTimer(pub Timer);

#[derive(Component)]
pub struct Lifetime(pub Timer);

#[derive(Component)]
pub struct BaseColor(pub Color);

#[derive(Component)]
pub struct FlashTimer(pub Timer);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FighterAnimState {
    Idle,
    Walk,
    Shoot,
}

#[derive(Component)]
pub struct FighterAnim {
    pub state: FighterAnimState,
    pub frame: usize,
    pub timer: Timer,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlimeAnimState {
    Idle,
    Shoot,
    Dead,
}

#[derive(Component)]
pub struct SlimeAnim {
    pub state: SlimeAnimState,
    pub frame: usize,
    pub timer: Timer,
}

/// Marker for the inner panel surface that can be highlighted
#[derive(Component)]
pub struct TilePanel {
    pub x: i32,
    pub y: i32,
}

/// Stores the base color of a tile panel for restoration after highlight
#[derive(Component)]
pub struct TileBaseColor(pub Color);

#[derive(Resource)]
pub struct InputCooldown(pub Timer);

#[derive(Resource)]
pub struct ShootCooldown(pub Timer);
