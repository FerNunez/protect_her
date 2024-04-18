use crate::prelude::*;

pub fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    game_atlas_layout: Res<GameAtlaseLayouts>,
    map: Res<Map>,
    mut enemy_count: ResMut<EnemyCount>,
    camera_query: Query<&Transform, (With<Camera>, Without<Player>)>,
    win_size: Res<WinSize>,
    mut wave_level: ResMut<WaveLevel>,
) {
    let spawn_number = wave_level.0 * wave_level.0 * ENEMY_SPAWN_RATE;

    for _ in 0..spawn_number {
        if enemy_count.alive < NUM_ENEMIES_MAX {
            let mut rng = thread_rng();

            //// compoute the start x/y
            //let x = if rng.gen_bool(0.5) { w_span } else { -w_span };
            let mut x = (rng.gen_range(0..MAP_SIZE_IN_TILES.0) * TILE_SIZE.0) as f32;
            let mut y = (rng.gen_range(0..MAP_SIZE_IN_TILES.1) * TILE_SIZE.1) as f32;

            if let Ok(camera_tf) = camera_query.get_single() {
                let camera_left = camera_tf.translation.x - win_size.w / 2.;
                let camera_right = camera_tf.translation.x + win_size.w / 2.;

                let camera_top = camera_tf.translation.y + win_size.h / 2.;
                let camera_bottom = camera_tf.translation.y - win_size.h / 2.;
                loop {
                    let pos = Vec2::new(x, y);
                    let inside_view =
                        x > camera_left && x < camera_right && y > camera_bottom && y < camera_top;

                    if map.can_enter_tile(&pos) && !inside_view {
                        break;
                    } else {
                        x = (rng.gen_range(0..MAP_SIZE_IN_TILES.0) * TILE_SIZE.0) as f32;
                        y = (rng.gen_range(0..MAP_SIZE_IN_TILES.1) * TILE_SIZE.1) as f32;
                    }
                }
            }
            let sperm = commands
                .spawn(SpriteBundle {
                    texture: game_textures.enemy.clone(),
                    transform: Transform::from_xyz(x, y, 1.0).with_scale(Vec3::new(
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
                .insert(Health(SPERM_HEALTH))
                .insert(CanFly)
                .id();

            let tail_animation = Animation::new(
                0,
                6,
                Timer::new(Duration::from_millis(200), TimerMode::Repeating),
            );

            let tail = commands
                .spawn(SpriteSheetBundle {
                    texture: game_textures.enemy_tail_animation.clone(),
                    transform: Transform::from_xyz(15., 0., 0.0)
                        .with_rotation(Quat::from_rotation_z(PI)),
                    atlas: TextureAtlas {
                        layout: game_atlas_layout.enemy_tail_animation.clone(),
                        index: tail_animation.first_index,
                    },
                    ..Default::default()
                })
                .insert(CanFly)
                .insert(tail_animation)
                .id();

            commands.entity(sperm).push_children(&[tail]);
            enemy_count.alive += 1;
        }
    }

    wave_level.0 += 1;
}

// drunk movement
pub fn enemy_target_player(
    mut commands: Commands,
    player_state: Res<PlayerState>,
    mut enemy_query: Query<(Entity, &mut Velocity, &mut Transform), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if player_state.alive {
        if let Ok(player_transform) = player_query.get_single() {
            for (enemy_entity, mut enemy_velocity, enemy_transform) in enemy_query.iter_mut() {
                let direction_vector = Vec2::new(
                    player_transform.translation.x - enemy_transform.translation.x,
                    -(player_transform.translation.y - enemy_transform.translation.y),
                );

                enemy_velocity.x = (direction_vector.x / direction_vector.length()) * SPERM_SPEED;
                enemy_velocity.y = -(direction_vector.y / direction_vector.length()) * SPERM_SPEED;

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
