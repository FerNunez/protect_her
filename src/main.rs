mod components;

mod egg;

mod enemy_builder;
mod map;
mod map_builder;
mod player_builder;
mod resources;
mod systems;
mod weapons;
mod prelude {

    pub use bevy::{
        input::mouse::MouseWheel, math::Vec3Swizzles, prelude::*,
        time::common_conditions::on_timer, utils::HashSet, window::PrimaryWindow,
    };

    pub use rand::{thread_rng, Rng};
    pub use std::f32::consts::PI;
    pub use std::time::Duration;

    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::resources::*;
    pub use crate::systems::*;

    pub const NUM_ENEMIES_MAX: u32 = 1000;
    pub const ENEMY_SPAWN_RATE_PER_MIN: f32 = 10.;

    pub const TIME_STEP: f32 = 1. / 60.;
    pub const BASE_SPEED: f32 = 300.;
    pub const SCREEN_SIZE: (i32, i32) = (2560, 1440);
    pub const MAP_SIZE_IN_TILES: (i32, i32) = (400, 300);
    pub const TILE_SIZE: (i32, i32) = (32, 32);
    pub const TILE_SCALE: i32 = 1;

    pub const EGG_SPRITE: &str = "matrix.png";
    pub const EGG_SIZE: (f32, f32) = (32., 32.);
    pub const EGG_SCALE: f32 = 1.;
    pub const EGG_SPEED: f32 = 5.;

    pub const SPERM: &str = "sperm_only_head.png";
    pub const SPERM_SCALE: f32 = 1.;
    pub const SPERM_SPEED: f32 = 1.5;
    pub const SPERM_SIZE: (f32, f32) = (12., 8.0);
    pub const SPERM_HEALTH: f32 = 10.;

    //pub const SPERM: &str = "sperm_32_32.png";
    //pub const SPERM_SCALE: f32 = 1.;
    //pub const SPERM_SPEED: f32 = 0.15;
    //pub const SPERM_SIZE: (f32, f32) = (144., 75.0);
    //pub const SPERM_HEALTH: f32 = 10.;

    //pub const PLAYER_LASER_SPRITE: &str = "laser_blue_18_32.png";
    pub const PLAYER_LASER_SPRITE: &str = "light.png";
    pub const PLAYER_LASER_SIZE: (f32, f32) = (4., 32.);
    pub const PLAYER_LASER_SPEED: f32 = 10.3;
    pub const PLAYER_DAMAGE: f32 = 2.;
    pub const PLAYER_LASER_SCALE: f32 = 1.;
    pub const PLAYER_SPEED: f32 = 200.0;
    pub const PLAYER_LASER_DESPAWN_DIST: f32 = 3000.;

    pub const FRAMES_HITTED: u16 = 10;

    pub const COIN_SPRITE: &str = "watermelon.png";
    pub const COIN_SIZE: (f32, f32) = (16., 16.);
    pub const COIN_SCALE: f32 = 1.2;

    pub const SKILL_SPRITE: &str = "TikTok.png";
    pub const SKILL_SIZE: (f32, f32) = (24., 24.);
    pub const SKILL_SCALE: f32 = 1.;

    pub const FLOOR_SPRITE: &str = "floor_pattern_dark.png";
    pub const WALL_SPRITE: &str = "map_edit.png";

    pub const CAMERA_WINDOWS_MARGIN: f32 = 275.;

    pub const MAX_WAVE_LEVEL: i32 = 100;
    pub const ENEMY_SPAWN_RATE: u32 = 20;
    pub const WAVE_TIMER: u64 = 10;
}

use crate::systems::camera::*;
use crate::systems::setup::*;

use crate::projectile::*;
use crate::systems::animation::*;
use crate::systems::cinematics::*;
use crate::systems::coin::*;
use crate::systems::input_handler::*;
use crate::systems::map_render::*;
use crate::systems::player::*;
use crate::systems::skill::*;
use crate::systems::ui::*;

use crate::enemy_builder::*;
use crate::player_builder::*;
use egg::*;
use prelude::map_render::render_map_system;
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
                resolution: (SCREEN_SIZE.0 as f32 * 0.7, SCREEN_SIZE.1 as f32 * 0.7).into(),
                //position: WindowPosition::At(IVec2::new(2 * SCREEN_SIZE.1 + 10, 10)),
                position: WindowPosition::At(IVec2::new(10, 10)),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(PreStartup, setup_system)
        .add_systems(Startup, render_map_system)
        .add_systems(Startup, egg_spawn_system)
        .add_systems(
            PreUpdate,
            (
                despawn_projectile_collision_system,
                despawn_projectile_position_system,
                update_render_map_system,
            ),
        )
        .add_systems(
            Update,
            (
                egg_target_move,
                zoom_system,
                move_camera_system,
                projectile_movement_system,
                user_mouse_handler_zoom_event_system,
                rotation_egg_parts,
                pull_force_system,
                //player_laser_hit_enemy_system,
                //animate_being_hitted,
                //spawn_coin_system,
                //player_pickup_coin_system,
                //spawn_skill_system,
                //player_pickup_skill_system,
                //text_update_system,
            ),
        )
        .add_systems(
            PostUpdate,
            (
                movable_system.after(player_keyboard_event_system),
                rotable_system,
                animate_sprite,
            ),
        )
        .run();
}
