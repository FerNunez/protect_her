use std::time::Duration;

use bevy::math::Vec2;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::components::{FromPlayer, Laser, Movable, Player, SpriteScale, SpriteSize, Velocity};

use crate::resources::{GameState, GameTextures, PlayerState, WinSize};
use crate::{BASE_SPRITE_SCALE, PLAYER_LASER_SIZE, PLAYER_LASER_SPEED};

impl Default for PlayerState {
    fn default() -> Self {
        Self { alive: false }
    }
}

impl PlayerState {
    //fn dead(&mut self) {
    //    self.alive = false;
    //}
    fn spawned(&mut self) {
        self.alive = true;
    }
}

// Player
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_spawn_system.run_if(on_timer(Duration::from_secs_f64(0.1))),
                player_keyboard_event_system,
                player_fire_system,
            ),
        );
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    game_state: Res<GameState>,
    mut player_state: ResMut<PlayerState>,
) {
    if !player_state.alive {
        commands
            .spawn(SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(
                    BASE_SPRITE_SCALE.0 * game_state.zoom,
                    BASE_SPRITE_SCALE.1 * game_state.zoom,
                    0.,
                )),
                ..Default::default()
            })
            .insert(Player)
            .insert(Movable)
            .insert(SpriteScale::from(BASE_SPRITE_SCALE))
            .insert(Velocity { x: 0., y: 0. });
        player_state.spawned();
    }
}

fn player_keyboard_event_system(
    kboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kboard.pressed(KeyCode::Q) {
            -1.
        } else if kboard.pressed(KeyCode::D) {
            1.
        } else {
            0.
        };

        velocity.y = if kboard.pressed(KeyCode::S) {
            -1.
        } else if kboard.pressed(KeyCode::Z) {
            1.
        } else {
            0.
        };
    }
}

fn player_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    game_state: Res<GameState>,
    win_size: Res<WinSize>,
    kboard: Res<Input<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        let player_position = Vec2::new(player_tf.translation.x, player_tf.translation.y);
        let mouse_position = window_query.single().cursor_position();

        let (velocity, angle) = match mouse_position {
            Some(mouse_position) => {
                // here remeber that position of player Y is positive going down
                let direction_vector = Vec2::new(
                    mouse_position.x - win_size.w / 2. - player_position.x,
                    mouse_position.y - win_size.h / 2. + player_position.y,
                );

                let angle = direction_vector.angle_between(Vec2 { x: 0.0, y: -1.0 });

                (
                    Velocity {
                        x: (direction_vector.x / direction_vector.length()) * PLAYER_LASER_SPEED,
                        y: -direction_vector.y / direction_vector.length() * PLAYER_LASER_SPEED,
                    },
                    angle,
                )
            }
            None => (Velocity { x: 0., y: 0. }, 0.0),
        };

        // with key
        if kboard.just_pressed(KeyCode::Space) {
            // probably set here a OPP? single call
            commands
                .spawn(SpriteBundle {
                    texture: game_textures.player_laser.clone(),
                    // TODO: player_y as a part of the SPRITE?
                    transform: Transform::from_xyz(player_position.x, player_position.y, 0.)
                        .with_scale(Vec3::new(
                            BASE_SPRITE_SCALE.0 * game_state.zoom,
                            BASE_SPRITE_SCALE.1 * game_state.zoom,
                            0.,
                        ))
                        .with_rotation(Quat::from_rotation_z(angle)),
                    ..Default::default()
                })
                .insert(Movable)
                .insert(velocity)
                .insert(FromPlayer)
                .insert(Laser)
                .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                .insert(SpriteScale::from(BASE_SPRITE_SCALE));
        }
        // without key
        // spawn all other skins
    }
}
