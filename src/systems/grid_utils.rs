use crate::constants::*;
use bevy::prelude::*;

/// Converts a logical tile coordinate to the world-space **floor point** (the bottom-center
/// of the tile).
///
/// This matches how BN-style sprites are positioned: character sprites use `Anchor::BottomCenter`
/// so their feet can snap directly to the tile floor point.
///
/// The arena is offset vertically by `ARENA_Y_OFFSET` to make room for the action bar.
pub fn tile_floor_world(x: i32, y: i32) -> Vec2 {
    let center_x = (GRID_WIDTH as f32 - 1.0) / 2.0;
    let center_y = (GRID_HEIGHT as f32 - 1.0) / 2.0;

    let relative_x = (x as f32) - center_x;
    let relative_y = (y as f32) - center_y;

    let pos_x = relative_x * TILE_STEP_X + relative_y * ROW_SKEW_X;
    let pos_y = relative_y * TILE_STEP_Y + ARENA_Y_OFFSET;

    Vec2::new(pos_x, pos_y)
}

/// World-space **center** of the tile mesh for `tile_floor_world(x, y)`.
pub fn tile_center_world(x: i32, y: i32) -> Vec2 {
    tile_floor_world(x, y) + Vec2::new(0.0, TILE_H * 0.5)
}
