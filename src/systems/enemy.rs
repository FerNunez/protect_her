use crate::prelude::*;

pub fn enemy_spawn_system(
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

pub fn enemy_target_player(
    mut commands: Commands,
    player_state: Res<PlayerState>,
    mut enemy_query: Query<(Entity, &mut Velocity, &mut Transform), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if player_state.alive {
        if let Ok(player_transform) = player_query.get_single() {
            for (enemy_entity, mut enemy_velocity, mut enemy_transform) in enemy_query.iter_mut() {
                let direction_vector = Vec2::new(
                    player_transform.translation.x - enemy_transform.translation.x,
                    -(player_transform.translation.y - enemy_transform.translation.y),
                );

                enemy_velocity.x = (direction_vector.x / direction_vector.length()) * SPERM_SPEED;
                enemy_velocity.y = (direction_vector.y / direction_vector.length()) * SPERM_SPEED;

                let delta = Vec2::new(
                    enemy_velocity.x * TIME_STEP * BASE_SPEED,
                    enemy_velocity.y * TIME_STEP * BASE_SPEED,
                );

                if delta.x != 0. || delta.y != 0. {
                    let destination = Vec2::new(
                        enemy_transform.translation.x + delta.x,
                        enemy_transform.translation.y + delta.y,
                    );

                    commands.spawn(WantsToMove {
                        entity: enemy_entity,
                        destination,
                    });
                }

                let angle = direction_vector.angle_between(Vec2 { x: 0.0, y: -1.0 }) + PI / 2.;
                //enemy_transform.rotation = Quat::from_rotation_z(angle );
                commands.spawn(WantsToRotate {
                    entity: enemy_entity,
                    angle,
                });
            }
        }
    }
}
