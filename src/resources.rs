use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerAnimations {
    pub idle_texture: Handle<Image>,
    pub idle_layout: Handle<TextureAtlasLayout>,
    pub walking_texture: Handle<Image>,
    pub walking_layout: Handle<TextureAtlasLayout>,
}
