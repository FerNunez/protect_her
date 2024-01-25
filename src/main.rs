use bevy::{
    input::mouse::MouseWheel, math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide,
    utils::HashSet, window::PrimaryWindow,
};

use components::{Enemy, FromPlayer, Health, Laser, Movable, SpriteSize, Velocity};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use resources::{EnemyCount, GameState, GameTextures, PlayerState, WinSize};

use crate::components::BeingHitted;

mod components;
mod resources;

mod enemy;
mod player;

const NUM_ENEMIES_MAX: u32 = 1000;

const SPRITE_SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
const PLAYER_SPRITE: &str = "player_a_01.png";
//const PLAYER_SIZE: (f32, f32) = (144., 75.0);
const RESOLUTION: (f32, f32) = (2560., 1440.);
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_SPEED: f32 = 0.22;
const ENEMY_SIZE: (f32, f32) = (144., 75.0);

const PLAYER_LASER_SPRITE: &str = "laser_b_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (17., 55.);
const PLAYER_LASER_SPEED: f32 = 1.8;

const FRAMES_HITTED: u16 = 10;

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
        enemy: asset_server.load(ENEMY_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
    };
    commands.insert_resource(game_texture);

    commands.insert_resource(PlayerState::default());

    let game_state = GameState { zoom: 0.5 };
    commands.insert_resource(game_state);

    commands.insert_resource(EnemyCount { alive: 0, dead: 0 });
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

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    mut enemy_query: Query<(Entity, &Transform, &SpriteSize, &mut Health), With<Enemy>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        //println!("Test2");
        if despawned_entities.contains(&laser_entity) {
            continue;
        }

        let laser_scale = Vec2::from(laser_tf.scale.xy());

        for (enemy_entity, enemy_tf, enemy_size, mut health) in enemy_query.iter_mut() {
            if despawned_entities.contains(&enemy_entity)
                || despawned_entities.contains(&laser_entity)
            {
                continue;
            }

            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            // Collision
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            // perform collision
            if let Some(_) = collision {
                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);

                health.0 -= 1;

                // spawn Explosion at enemy tf
                commands.entity(enemy_entity).insert(BeingHitted(0));

                if health.0 == 0 {
                    enemy_count.alive -= 1;
                    enemy_count.dead += 1;

                    commands.entity(enemy_entity).despawn();
                    despawned_entities.insert(enemy_entity);

                    //commands.spawn(ExplotionHere);
                    // spawn coin!
                }

                // command action to change color of image
            }
        }
    }
}

fn animate_being_hitted(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BeingHitted, &mut Sprite)>,
) {
    for (entity, mut frame_hitted, mut sprite) in query.iter_mut() {
        frame_hitted.0 += 1;
        sprite.color.set_a(0.2);

        if frame_hitted.0 >= FRAMES_HITTED {
            commands.entity(entity).remove::<BeingHitted>();
            sprite.color.set_a(1.);
        }
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
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                zoom_system,
                movable_system,
                user_mouse_handler_zoom_event_system,
                player_laser_hit_enemy_system,
                animate_being_hitted,
            ),
        )
        .run();
}
