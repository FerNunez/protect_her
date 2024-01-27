use rand::{thread_rng, Rng};
use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::components::{Enemy, Health, Movable, Player, SpriteScale, SpriteSize, Velocity};

use crate::resources::{EnemyCount, GameState, GameTextures, PlayerState, WinSize};
use crate::{BASE_SPRITE_SCALE, ENEMY_HEALTH, ENEMY_SIZE};
use crate::{ENEMY_SPEED, NUM_ENEMIES_MAX};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                enemy_spawn_system.run_if(on_timer(Duration::from_secs_f64(1.))),
                enemy_target_player.run_if(resource_exists::<PlayerState>()),
            ),
        );
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    game_state: Res<GameState>,
    win_size: Res<WinSize>,
    mut enemy_count: ResMut<EnemyCount>,
) {
    if enemy_count.alive < NUM_ENEMIES_MAX {
        let mut rng = thread_rng();

        //// compoute the start x/y
        let w_span = win_size.w / 2.;
        let h_span = win_size.h / 2.;
        //let x = if rng.gen_bool(0.5) { w_span } else { -w_span };
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;

        commands
            .spawn(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform::from_xyz(x, y, 0.0).with_scale(Vec3::new(
                    BASE_SPRITE_SCALE.0 * game_state.zoom,
                    BASE_SPRITE_SCALE.1 * game_state.zoom,
                    0.,
                )),
                ..Default::default()
            })
            .insert(Enemy)
            .insert(Movable)
            .insert(Velocity { x: 0., y: 0. })
            .insert(SpriteSize::from(ENEMY_SIZE))
            .insert(SpriteScale::from(BASE_SPRITE_SCALE))
            .insert(Health(ENEMY_HEALTH));

        enemy_count.alive += 1;
    }
}

fn enemy_target_player(
    player_state: Res<PlayerState>,
    mut enemy_query: Query<(&mut Velocity, &Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if player_state.alive {
        if let Ok(player_transform) = player_query.get_single() {
            for (mut enemy_velocity, enemy_transform) in enemy_query.iter_mut() {
                enemy_velocity.x = player_transform.translation.x - enemy_transform.translation.x;
                enemy_velocity.y = player_transform.translation.y - enemy_transform.translation.y;
                let speed = bevy::prelude::Vec2::from((enemy_velocity.x, enemy_velocity.y));
                enemy_velocity.x *= (ENEMY_SPEED / speed.length()) * 0.35;
                enemy_velocity.y *= ENEMY_SPEED / speed.length() * 0.35;
            }
        }
    }
}
