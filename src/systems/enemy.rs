use crate::prelude::*;

pub fn enemy_speed_scaling_system(
    time: Res<Time>,
    mut velocity_query: Query<(&AliveDuration, &mut SpeedScalingInTime)>,
) {
    for (alive_duration, mut speed_scaling) in velocity_query.iter_mut() {
        let speed_scale = (time.elapsed() - alive_duration.0).as_secs_f32() * 0.0001;

        speed_scaling.0 += speed_scale;
    }
}

pub fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    game_atlas_layout: Res<GameAtlaseLayouts>,
    map: Res<Map>,
    mut enemy_count: ResMut<EnemyCount>,
    camera_query: Query<&Transform, (With<Camera>, Without<Player>)>,
    win_size: Res<WinSize>,
    mut wave_level: ResMut<WaveLevel>,
    time: Res<Time>,
) {
    let spawn_number = wave_level.0 * wave_level.0 * ENEMY_SPAWN_RATE;

    for _ in 0..spawn_number {
        if enemy_count.alive < NUM_ENEMIES_MAX {
            let mut rng = thread_rng();

            //// compoute the start x/y
            //let x = if rng.gen_bool(0.5) { w_span } else { -w_span };
            //let mut x = (rng.gen_range(0..MAP_SIZE_IN_TILES.0) * TILE_SIZE.0) as f32;
            //let mut y = (rng.gen_r.ange(0..MAP_SIZE_IN_TILES.1) * TILE_SIZE.1) as f32;
            let spawn_pos = Vec2::new((MAP_SIZE_IN_TILES.0*TILE_SIZE.0/2) as f32, 0.);
            let x = rng.gen_range(-SPAWN_AREA_SIZE.0/2..SPAWN_AREA_SIZE.0/2);
            let y = rng.gen_range(-SPAWN_AREA_SIZE.1/2..SPAWN_AREA_SIZE.1/2);

            let x = x as f32+spawn_pos.x;
            let y = y as f32+spawn_pos.y;

            //if let Ok(camera_tf) = camera_query.get_single() {
            //    let camera_left = camera_tf.translation.x - win_size.w / 2.;
            //    let camera_right = camera_tf.translation.x + win_size.w / 2.;

            //    let camera_top = camera_tf.translation.y + win_size.h / 2.;
            //    let camera_bottom = camera_tf.translation.y - win_size.h / 2.;
            //    //loop {
            //    //    let pos = Vec2::new(x, y);
            //    //    let inside_view =
            //    //        x > camera_left && x < camera_right && y > camera_bottom && y < camera_top;

            //    //    if map.can_enter_tile(&pos) && !inside_view {
            //    //        break;
            //    //    } else {
            //    //        x = (rng.gen_range(0..MAP_SIZE_IN_TILES.0) * TILE_SIZE.0) as f32;
            //    //        y = (rng.gen_range(0..MAP_SIZE_IN_TILES.1) * TILE_SIZE.1) as f32;
            //    //    }
            //    //}
            //}
            let sperm = commands
                .spawn(SpriteBundle {
                    texture: game_textures.enemy.clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, 2.0).with_scale(Vec3::new(
                        SPERM_SCALE,
                        SPERM_SCALE,
                        1.,
                    )),
                    ..Default::default()
                })
                .insert(Enemy::new())
                .insert(Movable)
                .insert(Velocity { x: 0., y: 0. })
                .insert(SpriteSize::from(SPERM_SIZE))
                .insert(Health(SPERM_HEALTH))
                .insert(CanFly)
                .insert(SpeedScalingInTime(20.))
                .insert(AliveDuration(time.elapsed().clone()))
                .id();

            let tail_animation = Animation::new(0, 6);

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
                .insert(AnimationTimer::new_from_millis(100))
                .id();

            commands.entity(sperm).push_children(&[tail]);
            enemy_count.alive += 1;
        }
    }

    wave_level.0 += 1;
}

// drunk movement
pub fn enemy_target_egg(
    time: Res<Time>,
    mut commands: Commands,
    egg_state: Res<PlayerState>,
    mut enemy_query: Query<
        (
            Entity,
            &Enemy,
            &mut Velocity,
            &mut Transform,
            &SpeedScalingInTime,
        ),
        Without<Egg>,
    >,
    emmbrion_query: Query<
        &GlobalTransform,
        (With<Embrion>, Without<Egg>, Without<Enemy>, Without<Player>),
    >,
) {
    if egg_state.alive {
        if let Ok(egg_transform) = emmbrion_query.get_single() {
            for (enemy_entity, enemy, mut enemy_velocity, enemy_transform, speed_scaling) in
                enemy_query.iter_mut()
            {
                //let (destination, angle) = enemy.bicycle_model(enemy_transform, egg_transform.translation.xy());
                let (direction_vector, angle) =
                    enemy.perfect_model(&enemy_transform, &egg_transform.translation().xy());

                enemy_velocity.x = direction_vector.normalize_or_zero().x * speed_scaling.0;
                enemy_velocity.y = -direction_vector.normalize_or_zero().y * speed_scaling.0;
                let delta_time = time.delta().as_secs_f32();

                let delta = Vec2::new(enemy_velocity.x * delta_time, enemy_velocity.y * delta_time);

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

                //enemy_transform.rotation = Quat::from_rotation_z(angle );
                commands.spawn(WantsToRotate {
                    entity: enemy_entity,
                    angle,
                });
            }
        }
    }
}
