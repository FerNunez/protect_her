use bevy::{
    math::Vec2,
    prelude::{Handle, Image, Resource},
    sprite::TextureAtlasLayout,
    time::Timer,
};

// Resources
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct GameState {
    pub zoom: f32,
    pub coins: u64,
}

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub enemy: Handle<Image>,
    pub enemy_tail: Handle<Image>,
    pub player_laser: Handle<Image>,
    pub coin: Handle<Image>,
    pub skill: Handle<Image>,
    pub floor: Handle<Image>,
    pub wall: Handle<Image>,
    // debug
    pub pixel: Handle<Image>,
    pub map_texture: Handle<Image>,
    pub enemy_tail_animation: Handle<Image>,
}

#[derive(Resource)]
pub struct GameAtlaseLayouts {
    pub map: Handle<TextureAtlasLayout>,
    pub enemy_tail_animation: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct PlayerState {
    pub alive: bool,
}

#[derive(Resource)]
pub struct EnemyCount {
    pub alive: u32,
    pub dead: u32,
}

#[derive(Resource)]
pub struct PlayerSkill {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct AtomaticPlayerSkillList(pub Vec<PlayerSkill>);

#[derive(Resource)]
pub struct WaveLevel(pub u32);

#[derive(Resource)]
pub struct LastMouse {
    pub pos: Vec2,
}
