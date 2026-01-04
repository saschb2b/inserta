use bevy::image::TextureAtlasLayout;
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct FighterSprites {
    pub layout: Handle<TextureAtlasLayout>,
    pub idle: Handle<Image>,
    pub walk: Handle<Image>,
    pub shoot: Handle<Image>,
    pub frames: usize,
}
