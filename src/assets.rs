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
