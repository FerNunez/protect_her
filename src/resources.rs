use crate::prelude::*;

// Resources
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

// TODO: Review logic of zoom + clicks

#[derive(Resource)]
pub struct GameState {
    pub zoom: f32,
    pub coins: u64,
    pub egg_spawn_position: IVec2,
    pub player_spawn_position: IVec2,
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
    pub egg_tentacles: Handle<Image>,
    pub egg_blue: Handle<Image>,
    pub egg_yellow: Handle<Image>,
    pub egg_red: Handle<Image>,
    pub egg_eye: Handle<Image>,
    // debug
    pub pixel: Handle<Image>,
    pub map_texture: Handle<Image>,
    pub enemy_tail_animation: Handle<Image>,
    pub player_animation: Handle<Image>,
}

#[derive(Resource)]
pub struct GameAtlaseLayouts {
    pub map: Handle<TextureAtlasLayout>,
    pub enemy_tail_animation: Handle<TextureAtlasLayout>,
    pub player_animation: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct PlayerState {
    pub alive: bool,
}
#[derive(Resource)]
pub struct EggState {
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
#[derive(Resource)]
pub struct AnimationsList {
    pub player: HashSet<Animation>,
}
