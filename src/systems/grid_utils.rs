use crate::resources::ArenaLayout;
use bevy::prelude::*;

// ============================================================================
// Grid Utility Functions (using ArenaLayout resource)
// ============================================================================

/// Converts a logical tile coordinate to the world-space **floor point** where
/// character feet should be placed.
///
/// This version uses the ArenaLayout resource for responsive scaling.
pub fn tile_floor_world_scaled(layout: &ArenaLayout, x: i32, y: i32) -> Vec2 {
    layout.tile_floor_world(x, y)
}

/// World-space position for the **sprite center** of a tile at (x, y).
///
/// This version uses the ArenaLayout resource for responsive scaling.
pub fn tile_sprite_world_scaled(layout: &ArenaLayout, x: i32, y: i32) -> Vec2 {
    layout.tile_sprite_world(x, y)
}

/// World-space **center** of the tile's visible area (excluding lip).
pub fn tile_center_world_scaled(layout: &ArenaLayout, x: i32, y: i32) -> Vec2 {
    layout.tile_floor_world(x, y)
}
