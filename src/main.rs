mod prelude {

    pub use bevy::{
        animation::prelude, input::mouse::MouseWheel, math::Vec3Swizzles, prelude::*,
        sprite::collide_aabb::collide, time::common_conditions::on_timer, utils::HashSet,
        window::PrimaryWindow,
    };

    pub use rand::{thread_rng, Rng};
    pub use std::f32::consts::PI;
    pub use std::time::Duration;

    pub use crate::components::*;
    pub use crate::resources::*;
    pub use crate::systems::*;

    pub const NUM_ENEMIES_MAX: u32 = 1000;

    pub const BASE_SPRITE_SCALE: f32 = 1.;
    pub const TIME_STEP: f32 = 1. / 60.;
    pub const BASE_SPEED: f32 = 400.;
    pub const RESOLUTION: (f32, f32) = (2560., 1440.);

    pub const EGG_SPRITE: &str = "egg.png";
    pub const EGG_SIZE: (f32, f32) = (282., 303.);
    pub const EGG_SCALE: f32 = 0.12;

    pub const SPERM: &str = "sperm.png";
    pub const SPERM_SCALE: f32 = 0.3;
    pub const SPERM_SPEED: f32 = 0.2;
    pub const SPERM_SIZE: (f32, f32) = (144., 75.0);
    pub const SPERM_HEALTH: f32 = 10.;

    pub const PLAYER_LASER_SPRITE: &str = "laser_b_01.png";
    pub const PLAYER_LASER_SIZE: (f32, f32) = (17., 55.);
    pub const PLAYER_LASER_SPEED: f32 = 1.3;
    pub const PLAYER_DAMAGE: f32 = 2.;
    pub const PLAYER_LASER_SCALE: f32 = 0.4;

    pub const FRAMES_HITTED: u16 = 10;

    pub const COIN_SPRITE: &str = "watermelon.png";
    pub const COIN_SIZE: (f32, f32) = (16., 16.);
    pub const COIN_SCALE: f32 = 1.2;

    pub const SKILL_SPRITE: &str = "TikTok.png";
    pub const SKILL_SIZE: (f32, f32) = (24., 24.);
    pub const SKILL_SCALE: f32 = 1.;

    pub const CAMERA_WINDOWS_MARGIN: f32 = 75.;
}

mod components;
mod enemy_builder;
mod player_builder;
mod resources;
mod systems;

use crate::systems::camera::*;
use crate::systems::setup::*;

use crate::systems::animation::*;
use crate::systems::cinematics::*;
use crate::systems::coin::*;
use crate::systems::input_handler::*;
use crate::systems::player::*;
use crate::systems::skill::*;
use crate::systems::ui::*;


use crate::enemy_builder::*;
use crate::player_builder::*;
use prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Rgba {
            red: (0.04),
            green: (0.04),
            blue: (0.04),
            alpha: (1.0),
        }))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Protect her!".to_string(),
                resolution: (RESOLUTION.0 / 2., RESOLUTION.1 / 2.).into(),
                position: WindowPosition::At(IVec2::new(0, 0)),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                zoom_system,
                move_camera_system,
                movable_system,
                user_mouse_handler_zoom_event_system,
                player_laser_hit_enemy_system,
                animate_being_hitted,
                spawn_coin_system,
                player_pickup_coin_system,
                spawn_skill_system,
                player_pickup_skill_system,
                text_update_system,
            ),
        )
        .run();
}
