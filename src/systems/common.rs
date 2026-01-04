use bevy::prelude::*;

use crate::components::{GridPosition, RenderConfig};
use crate::constants::DEPTH_Y_TO_Z;
use crate::systems::grid_utils::tile_floor_world;

pub fn update_transforms(mut query: Query<(&GridPosition, &RenderConfig, &mut Transform)>) {
    for (pos, render, mut transform) in &mut query {
        // Entities are positioned relative to the floor point.
        let floor = tile_floor_world(pos.x, pos.y);
        let depth = -floor.y * DEPTH_Y_TO_Z;

        transform.translation.x = floor.x + render.offset.x;
        transform.translation.y = floor.y + render.offset.y;
        transform.translation.z = render.base_z + depth;
    }
}
