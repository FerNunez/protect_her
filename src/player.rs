use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer,};

use crate::components::{Movable, Velocity, Player};

use crate::SPRITE_SCALE;
use crate::resources::{PlayerState,GameTextures, };

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
            ),
        );
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut player_state: ResMut<PlayerState>,
) {
    if !player_state.alive {
        commands
            .spawn(SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(
                    SPRITE_SCALE,
                    SPRITE_SCALE,
                    1.0,
                )),
                ..Default::default()
            })
            .insert(Player)
            .insert(Movable)
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
