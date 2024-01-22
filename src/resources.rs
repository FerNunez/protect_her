use bevy::prelude::{Handle, Image, Resource};

// Resources
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct GameState {
    pub zoom: f32,
}

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
}

#[derive(Resource)]
pub struct PlayerState {
    pub alive: bool,
}
