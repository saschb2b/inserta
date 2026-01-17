use bevy::prelude::*;

use crate::components::{GridPosition, RenderConfig};
use crate::constants::DEPTH_Y_TO_Z;
use crate::resources::ArenaLayout;

pub fn update_transforms(
    layout: Res<ArenaLayout>,
    mut query: Query<(&GridPosition, &RenderConfig, &mut Transform)>,
) {
    for (pos, render, mut transform) in &mut query {
        // Entities are positioned relative to the floor point.
        let floor = layout.tile_floor_world(pos.x, pos.y);
        let depth = -floor.y * DEPTH_Y_TO_Z;

        // Scale the offset by the layout scale factor
        transform.translation.x = floor.x + render.offset.x * layout.scale;
        transform.translation.y = floor.y + render.offset.y * layout.scale;
        transform.translation.z = render.base_z + depth;
    }
}
