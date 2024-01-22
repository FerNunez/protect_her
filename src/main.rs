use bevy::{input::mouse::MouseWheel, prelude::*, window::PrimaryWindow};

use components::{Movable, Velocity};
use player::PlayerPlugin;
use resources::{GameState, GameTextures, PlayerState, WinSize};

mod components;
mod resources;

mod player;

const SPRITE_SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
const PLAYER_SPRITE: &str = "player_a_01.png";
//const PLAYER_SIZE: (f32, f32) = (144., 75.0);
const RESOLUTION: (f32, f32) = (2560., 1440.);
const ENEMY_SPRITE: &str = "enemy_a_01.png";

fn zoom_system(game_state: ResMut<GameState>, mut query: Query<&mut Transform, With<Sprite>>) {
    for mut transform in query.iter_mut() {
        let scale = &mut transform.scale;
        scale.x = game_state.zoom * SPRITE_SCALE;
        scale.y = game_state.zoom * SPRITE_SCALE;
    }
}

fn user_mouse_handler_zoom_event_system(
    mut scroll_evr: EventReader<MouseWheel>,
    mut game_state: ResMut<GameState>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                game_state.zoom += ev.y / 10.;
            }
            MouseScrollUnit::Pixel => {}
        }
    }

    if game_state.zoom <= 0.0 {
        game_state.zoom = 0.1;
    }
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = windows_query.get_single().unwrap();
    let (win_w, win_h) = (window.resolution.width(), window.resolution.height());

    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    let game_texture = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_texture);

    commands.insert_resource(PlayerState::default());

    let game_state = GameState { zoom: 0.5 };
    commands.insert_resource(game_state);
}

fn movable_system(
    game_state: ResMut<GameState>,
    mut query: Query<(&Velocity, &mut Transform), With<Movable>>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED * game_state.zoom;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED * game_state.zoom;
    }
}

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
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                zoom_system,
                movable_system,
                user_mouse_handler_zoom_event_system,
            ),
        )
        .run();
}
