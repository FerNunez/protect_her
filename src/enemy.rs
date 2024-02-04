use rand::{thread_rng, Rng};
use std::f32::consts::PI;
use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::components::{Enemy, Health, Movable, Player, SpriteSize, Velocity};

use crate::resources::{EnemyCount, GameTextures, PlayerState, WinSize};
use crate::{NUM_ENEMIES_MAX, SPERM_HEALTH, SPERM_SCALE, SPERM_SIZE, SPERM_SPEED};

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
                    SPERM_SCALE,
                    SPERM_SCALE,
                    0.,
                )),
                ..Default::default()
            })
            .insert(Enemy)
            .insert(Movable)
            .insert(Velocity { x: 0., y: 0. })
            .insert(SpriteSize::from(SPERM_SIZE))
            .insert(Health(SPERM_HEALTH));

        enemy_count.alive += 1;
    }
}

fn enemy_target_player(
    player_state: Res<PlayerState>,
    mut enemy_query: Query<(&mut Velocity, &mut Transform), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if player_state.alive {
        if let Ok(player_transform) = player_query.get_single() {
            for (mut enemy_velocity, mut enemy_transform) in enemy_query.iter_mut() {
                enemy_velocity.x = player_transform.translation.x - enemy_transform.translation.x;
                enemy_velocity.y = player_transform.translation.y - enemy_transform.translation.y;
                let speed = bevy::prelude::Vec2::from((enemy_velocity.x, enemy_velocity.y));
                enemy_velocity.x *= (SPERM_SPEED / speed.length()) * 0.35;
                enemy_velocity.y *= SPERM_SPEED / speed.length() * 0.35;

                let direction_vector = Vec2::new(enemy_velocity.x, -enemy_velocity.y);
                let angle = direction_vector.angle_between(Vec2 { x: 0.0, y: -1.0 });

                enemy_transform.rotation = Quat::from_rotation_z(angle + PI / 2.);
            }
        }
    }
}
