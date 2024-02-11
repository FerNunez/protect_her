use std::time::Duration;

use bevy::{
    input::mouse::MouseWheel, math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide,
    utils::HashSet, window::PrimaryWindow,
};

use components::{
    BeingHitted, Coin, CoinText, Damage, Enemy, FromPlayer, Health, Laser, Movable, Player,
    SpawnCoin, SpawnSkill, SpriteSize, Velocity, UI,
};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use resources::{
    AtomaticPlayerSkillList, EnemyCount, GameState, GameTextures, PlayerSkill, PlayerState, WinSize,
};

mod components;
mod resources;

mod enemy;
mod player;

const NUM_ENEMIES_MAX: u32 = 1000;

const BASE_SPRITE_SCALE: f32 = 1.;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 400.;
const RESOLUTION: (f32, f32) = (2560., 1440.);

const EGG_SPRITE: &str = "egg.png";
const EGG_SIZE: (f32, f32) = (282., 303.);
const EGG_SCALE: f32 = 0.12;

const SPERM: &str = "sperm.png";
const SPERM_SCALE: f32 = 0.3;
const SPERM_SPEED: f32 = 0.2;
const SPERM_SIZE: (f32, f32) = (144., 75.0);
const SPERM_HEALTH: f32 = 10.;

const PLAYER_LASER_SPRITE: &str = "laser_b_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (17., 55.);
const PLAYER_LASER_SPEED: f32 = 1.3;
const PLAYER_DAMAGE: f32 = 2.;
const PLAYER_LASER_SCALE: f32 = 0.4;

const FRAMES_HITTED: u16 = 10;

const COIN_SPRITE: &str = "watermelon.png";
const COIN_SIZE: (f32, f32) = (16., 16.);
const COIN_SCALE: f32 = 1.2;

const SKILL_SPRITE: &str = "TikTok.png";
const SKILL_SIZE: (f32, f32) = (24., 24.);
const SKILL_SCALE: f32 = 1.;

const CAMERA_WINDOWS_MARGIN: f32 = 75.;

fn zoom_system(
    game_state: ResMut<GameState>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for mut projection in camera_query.iter_mut() {
        projection.scale = game_state.zoom;
    }
}

fn move_camera_system(
    win_size: Res<WinSize>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<(&Transform, &SpriteSize), With<Player>>,
) {
    if let Ok((player_tf, player_size)) = player_query.get_single() {
        let player_left = player_tf.translation.x - player_size.0.x * player_tf.scale.x;
        let player_right = player_tf.translation.x + player_size.0.x * player_tf.scale.x;

        let player_top = player_tf.translation.y + player_size.0.y * player_tf.scale.y;
        let player_bottom = player_tf.translation.y - player_size.0.y * player_tf.scale.y;

        if let Ok(mut camera_tf) = camera_query.get_single_mut() {
            let camera_left = camera_tf.translation.x - win_size.w / 2.;
            let camera_right = camera_tf.translation.x + win_size.w / 2.;

            let camera_top = camera_tf.translation.y + win_size.h / 2.;
            let camera_bottom = camera_tf.translation.y - win_size.h / 2.;

            if player_left < camera_left + CAMERA_WINDOWS_MARGIN{
                camera_tf.translation.x -= camera_left+CAMERA_WINDOWS_MARGIN - player_left;
            }
            else if  player_right > camera_right - CAMERA_WINDOWS_MARGIN {
                camera_tf.translation.x -= camera_right-CAMERA_WINDOWS_MARGIN - player_right;
            }
            if player_top > camera_top - CAMERA_WINDOWS_MARGIN {
                camera_tf.translation.y -= camera_top-CAMERA_WINDOWS_MARGIN - player_top;
            }
            else if player_bottom < camera_bottom + CAMERA_WINDOWS_MARGIN {
                camera_tf.translation.y -= camera_bottom+CAMERA_WINDOWS_MARGIN - player_bottom;
            }
        }
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
        player: asset_server.load(EGG_SPRITE),
        enemy: asset_server.load(SPERM),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        coin: asset_server.load(COIN_SPRITE),
        skill: asset_server.load(SKILL_SPRITE),
    };
    commands.insert_resource(game_texture);

    commands.insert_resource(PlayerState::default());

    let game_state = GameState { zoom: 1., coins: 0 };
    commands.insert_resource(game_state);

    commands.insert_resource(EnemyCount { alive: 0, dead: 0 });

    commands.insert_resource(PlayerSkill {
        timer: Timer::new(Duration::from_millis(100), TimerMode::Once),
    });
    commands.insert_resource(AtomaticPlayerSkillList(Vec::new()));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Coins ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 50.,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 50.,
                color: Color::GOLD,
                ..default()
            }),
        ])
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        CoinText,
    ));
}

fn movable_system(mut query: Query<(&Velocity, &mut Transform), With<Movable>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    laser_query: Query<(Entity, &Transform, &SpriteSize, &Damage), (With<Laser>, With<FromPlayer>)>,
    mut enemy_query: Query<(Entity, &Transform, &SpriteSize, &mut Health), With<Enemy>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    for (laser_entity, laser_tf, laser_size, laser_damage) in laser_query.iter() {
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

                health.0 -= laser_damage.0;

                // spawn Explosion at enemy tf
                commands.entity(enemy_entity).insert(BeingHitted(0));

                if health.0 <= 0.0 {
                    enemy_count.alive -= 1;
                    enemy_count.dead += 1;

                    commands.entity(enemy_entity).despawn();
                    despawned_entities.insert(enemy_entity);

                    //commands.spawn(ExplotionHere);
                    commands.spawn(SpawnCoin(Vec2::new(
                        enemy_tf.translation.x,
                        enemy_tf.translation.y,
                    )));
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

fn spawn_coin_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &SpawnCoin)>,
) {
    for (entity, spawn_coin) in query.iter() {
        let x = spawn_coin.0.x;
        let y = spawn_coin.0.y;

        let sprite_size = (COIN_SCALE * COIN_SIZE.0, COIN_SCALE * COIN_SIZE.1);
        commands
            .spawn(SpriteBundle {
                texture: game_textures.coin.clone(),
                transform: Transform::from_xyz(x, y, 0.0)
                    .with_scale(Vec3::new(COIN_SCALE, COIN_SCALE, 0.)),
                ..Default::default()
            })
            .insert(SpriteSize::from(sprite_size))
            // TODO: fix this. create muiltiplication (f32, f"2)*f32
            .insert(Coin);

        commands.entity(entity).despawn();
    }
}

fn player_pickup_coin_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    coin_query: Query<(Entity, &Transform, &SpriteSize), With<Coin>>,
    player_query: Query<(&Transform, &SpriteSize), With<Player>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();
    let mut skill_spawned = false;

    if let Ok((player_tf, player_size)) = player_query.get_single() {
        let player_scale = Vec2::from(player_tf.scale.xy());

        for (coin_entity, coin_tf, coin_size) in coin_query.iter() {
            if despawned_entities.contains(&coin_entity) {
                continue;
            }

            let coin_scale = Vec2::from(coin_tf.scale.xy());

            // Collision
            let collision = collide(
                player_tf.translation,
                player_size.0 * player_scale,
                coin_tf.translation,
                coin_size.0 * coin_scale,
            );

            // perform collision
            if let Some(_) = collision {
                commands.entity(coin_entity).despawn();
                despawned_entities.insert(coin_entity);
                game_state.coins += 1;

                if game_state.coins == 2 && !skill_spawned {
                    commands.spawn(SpawnSkill(Vec2::new(0., 0.)));
                    skill_spawned = true;
                }
            }
        }
    }
}

fn spawn_skill_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
    spawn_skill_query: Query<(Entity, &SpawnSkill)>,
) {
    if let Ok((entity, _spawn_skill)) = spawn_skill_query.get_single() {
        commands
            .spawn(SpriteBundle {
                texture: game_textures.skill.clone(),
                transform: Transform::from_xyz(
                    (-win_size.w / 2.) + 10.,
                    (win_size.h / 2.) - 10.,
                    0.,
                )
                .with_scale(Vec3::new(SKILL_SCALE, SKILL_SCALE, 1.)),
                ..Default::default()
            })
            .insert(UI)
            .insert(SpriteSize::from(SKILL_SIZE));

        commands.entity(entity).despawn();
    }
}

fn player_pickup_skill_system(
    mut commands: Commands,
    mut player_skill_list: ResMut<AtomaticPlayerSkillList>,
    skill_query: Query<(Entity, &Transform, &SpriteSize), With<UI>>,
    player_query: Query<(&Transform, &SpriteSize), With<Player>>,
) {
    if let Ok((player_tf, player_size)) = player_query.get_single() {
        let player_scale = Vec2::from(player_tf.scale.xy());

        for (skill_entity, skill_tf, skill_size) in skill_query.iter() {
            let skill_scale = Vec2::from(skill_tf.scale.xy());

            // Collision
            let collision = collide(
                player_tf.translation,
                player_size.0 * player_scale,
                skill_tf.translation,
                skill_size.0 * skill_scale,
            );

            // perform collision
            if let Some(_) = collision {
                commands.entity(skill_entity).despawn();
                player_skill_list.0.push(PlayerSkill {
                    timer: Timer::new(Duration::from_millis(1500), TimerMode::Once),
                });
            }
        }
    }
}

fn text_update_system(game_state: Res<GameState>, mut query: Query<&mut Text, With<CoinText>>) {
    for mut text in &mut query {
        text.sections[1].value = format!("{}", game_state.coins)
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
