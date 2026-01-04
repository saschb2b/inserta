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

#[derive(Resource, Clone)]
pub struct SlimeSprites {
    pub layout: Handle<TextureAtlasLayout>, // For idle/dead (3x3 grid)
    pub shoot_layout: Handle<TextureAtlasLayout>, // For shooting (3x4 grid)
    pub idle: Handle<Image>,
    pub shoot: Handle<Image>,
    pub dead: Handle<Image>,
    pub idle_frames: usize,
    pub shoot_frames: usize,
    pub dead_frames: usize,
}
