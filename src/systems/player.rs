//use std::f32::consts::PI;
//use std::time::Duration;
//
//use bevy::math::Vec2;
//use bevy::window::PrimaryWindow;
//use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::prelude::*;
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};

pub fn player_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut automatic_player_skill_list: ResMut<AtomaticPlayerSkillList>,
    mut player_skill: ResMut<PlayerSkill>,
    time: Res<Time>,
    win_size: Res<WinSize>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    query: Query<&Transform, With<Player>>,
    camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_tf) = query.get_single() {
        let player_position = Vec2::new(player_tf.translation.x, player_tf.translation.y);
        // get vector velocity
        if let Ok(camera_tf) = camera_query.get_single() {
            let mouse_position_from_window = window_query.single().cursor_position();
            //info!("mouse pos from window: {:?}", mouse_position_from_window);

            let (velocity, angle) = match mouse_position_from_window {
                Some(mouse_position_from_window) => {
                    // mouse pos from window frame (inverted y) to map frame
                    let mouse_pos_from_camera = Vec2::new(
                        mouse_position_from_window.x,
                        win_size.h - mouse_position_from_window.y,
                    );
                    let win_size_gap = Vec2::new(win_size.w / 2., win_size.h / 2.);
                    let camera_pos = Vec2::new(camera_tf.translation.x, camera_tf.translation.y);
                    let camera_corner_pos = camera_pos - win_size_gap/2.;
                    let mouse_pos_from_origin = camera_corner_pos + mouse_pos_from_camera;
                    //let mouse_position_from_map = mouse_position_from_window + camera_pos + win_size_gap;

                    //info!("mouse pos xy: {:?}", mouse_pos);
                    //info!("camera_pos xy: {:?}", camera_pos);
                    // info!("win_sizes: {:?}", win_size_gap);
                    //info!("player pos xy: {:?}", player_position);
                    // here remeber that position of player Y is positive going down
                    let direction_vector = mouse_pos_from_origin - player_position.xy();

                    let angle = direction_vector.angle_between(Vec2 { x: 0.0, y: -1.0 });

                    (
                        Velocity {
                            x: (direction_vector.x / direction_vector.length())
                                * PLAYER_LASER_SPEED,
                            y: -direction_vector.y / direction_vector.length() * PLAYER_LASER_SPEED,
                        },
                        angle,
                    )
                }
                None => (Velocity { x: 0., y: 0. }, 0.0),
            };

            // with key
            if mouse_button.pressed(MouseButton::Left) {
                // probably set here a OPP? single call
                player_skill.timer.tick(time.delta());
                if player_skill.timer.finished() {
                    player_skill.timer.reset();
                    commands
                        .spawn(SpriteBundle {
                            texture: game_textures.player_laser.clone(),
                            // TODO: player_y as a part of the SPRITE?
                            transform: Transform::from_xyz(
                                player_position.x,
                                player_position.y,
                                1.,
                            )
                            .with_scale(Vec3::new(PLAYER_LASER_SCALE, PLAYER_LASER_SCALE, 0.))
                            .with_rotation(Quat::from_rotation_z(angle)),
                            ..Default::default()
                        })
                        .insert(Movable)
                        .insert(velocity)
                        .insert(FromPlayer)
                        .insert(Projectile)
                        .insert(Damage(PLAYER_DAMAGE))
                        .insert(SpriteSize::from(PLAYER_LASER_SIZE));
                }
            }
        }
        // without key
        // spawn all other skins
        for auto_player_skill in &mut automatic_player_skill_list.0 {
            auto_player_skill.timer.tick(time.delta());

            if auto_player_skill.timer.finished() {
                auto_player_skill.timer.reset();
                for angle in (0..=360).step_by(20) {
                    let angle = (angle as f32) * PI / 180.;
                    let x = angle.cos();
                    let y = angle.sin();

                    commands
                        .spawn(SpriteBundle {
                            texture: game_textures.player_laser.clone(),
                            // TODO: player_y as a part of the SPRITE?
                            transform: Transform::from_xyz(
                                player_position.x,
                                player_position.y,
                                0.,
                            )
                            .with_scale(Vec3::new(PLAYER_LASER_SCALE, PLAYER_LASER_SCALE, 0.))
                            .with_rotation(Quat::from_rotation_z(angle - PI / 2.)),
                            ..Default::default()
                        })
                        .insert(Movable)
                        .insert(Velocity { x, y })
                        .insert(FromPlayer)
                        .insert(Projectile)
                        .insert(Damage(PLAYER_DAMAGE))
                        .insert(SpriteSize::from(PLAYER_LASER_SIZE));
                }
            }
        }
    }
}

pub fn player_keyboard_dash_system(
    kboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    player_query: Query<Entity, (With<Player>, With<CanDash>, Without<Dash>)>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        if kboard.just_pressed(KeyCode::Space) {
            commands.entity(player_entity).insert(Dash {
                timer: Timer::new(Duration::from_millis(50), TimerMode::Once),
                velocity_offset: 25.,
            });
        }
    }
}

pub fn player_dash_system(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform, &mut Dash), (With<Player>, With<Movable>)>,
) {
    if let Ok((player_entity, mut player_tf, mut player_dash)) = player_query.get_single_mut() {
        player_dash.timer.tick(time.delta());

        if player_dash.timer.just_finished() {
            commands.entity(player_entity).remove::<Dash>();
            return;
        } else {
            player_tf.translation.x += player_dash.velocity_offset;
        }
    }
}

pub fn player_laser_hit_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    laser_query: Query<
        (Entity, &Transform, &SpriteSize, &Damage),
        (With<Projectile>, With<FromPlayer>),
    >,
    mut enemy_query: Query<(Entity, &Transform, &SpriteSize, &mut Health), With<Enemy>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    for (laser_entity, laser_tf, laser_size, laser_damage) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue;
        }

        for (enemy_entity, enemy_tf, enemy_size, mut health) in enemy_query.iter_mut() {
            if despawned_entities.contains(&enemy_entity)
                || despawned_entities.contains(&laser_entity)
            {
                continue;
            }

            // TODO: why/4
            let laser_aabb2d = Aabb2d::new(laser_tf.translation.xy(), laser_size.0 / 2.);
            let bounding_circle =
                BoundingCircle::new(enemy_tf.translation.xy(), enemy_size.0.x / 4.);

            if bounding_circle.intersects(&laser_aabb2d) {
                commands.spawn(Collide {
                    from: laser_entity,
                    to: enemy_entity,
                    pos: enemy_tf.translation.xy(),
                });
                commands.entity(laser_entity).insert(HasCollided);
            }

            // Collisioin
            //let collision = collide(
            //    laser_tf.translation,
            //    laser_size.0 * laser_scale,
            //    enemy_tf.translation,
            //    enemy_size.0 * enemy_scale,
            //);

            //// perform collision
            //if let Some(_) = collision {
            //    commands.spawn(Collide {
            //        from: laser_entity,
            //        to: enemy_entity,
            //        pos: enemy_tf.translation.xy(),
            //    });
            //    commands.entity(laser_entity).insert(HasCollided);

            //    //commands.entity(laser_entity).despawn();
            //    //despawned_entities.insert(laser_entity);

            //    // health.0 -= laser_damage.0;

            //    // // spawn Explosion at enemy tf
            //    // commands.entity(enemy_entity).insert(BeingHitted(0));

            //    // if health.0 <= 0.0 {
            //    //     enemy_count.alive -= 1;
            //    //     enemy_count.dead += 1;

            //    //     commands.entity(enemy_entity).despawn();
            //    //     despawned_entities.insert(enemy_entity);

            //    //     //commands.spawn(ExplotionHere);
            //    //     commands.spawn(SpawnCoin(Vec2::new(
            //    //         enemy_tf.translation.x,
            //    //         enemy_tf.translation.y,
            //    //     )));
            //    // }

            //    // command action to change color of image
            //}
        }
    }
}

pub fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut player_state: ResMut<PlayerState>,
) {
    if !player_state.alive {
        commands
            .spawn(SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform::from_xyz(
                    (MAP_SIZE_IN_TILES.0 * TILE_SIZE.0 / 2) as f32,
                    (MAP_SIZE_IN_TILES.1 * TILE_SIZE.1 / 2) as f32,
                    1.0,
                )
                .with_scale(Vec3::new(EGG_SCALE, EGG_SCALE, 0.)),
                ..Default::default()
            })
            .insert(Player)
            .insert(Movable)
            .insert(SpriteSize::from(EGG_SIZE))
            .insert(Velocity { x: 0., y: 0. })
            .insert(CanWallRide)
            .insert(CanDash);
        player_state.spawned();
    }
}

pub fn player_keyboard_event_system(
    mut commands: Commands,
    kboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (Entity, &mut Transform, &mut Velocity),
        (With<Player>, Without<AskingToMove>),
    >,
) {
    if let Ok((entity, transform, mut velocity)) = query.get_single_mut() {
        let angle = 0.01;
        velocity.x = if kboard.pressed(KeyCode::KeyA) {
            -1.
        } else if kboard.pressed(KeyCode::KeyD) {
            1.
        } else {
            0.
        };

        velocity.y = if kboard.pressed(KeyCode::KeyS) {
            -1.
        } else if kboard.pressed(KeyCode::KeyW) {
            1.
        } else {
            0.
        };

        let delta = Vec2::new(
            velocity.x * TIME_STEP * BASE_SPEED,
            velocity.y * TIME_STEP * BASE_SPEED,
        );

        if delta.x != 0. || delta.y != 0. {
            let destination = Vec2::new(
                transform.translation.x + delta.x,
                transform.translation.y + delta.y,
            );

            commands.spawn(WantsToMove {
                entity,
                destination,
            });

            commands.entity(entity).insert(AskingToMove);
        }

        commands.spawn(WantsToRotate { entity, angle });
    }
}

pub fn player_pickup_coin_system(
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
            //let collision = collide(
            //    player_tf.translation,
            //    player_size.0 * player_scale,
            //    coin_tf.translation,
            //    coin_size.0 * coin_scale,
            //);

            // perform collision
            //if let Some(_) = collision {
            //    commands.entity(coin_entity).despawn();
            //    despawned_entities.insert(coin_entity);
            //    game_state.coins += 1;

            //    if game_state.coins == 2 && !skill_spawned {
            //        commands.spawn(SpawnSkill(Vec2::new(0., 0.)));
            //        skill_spawned = true;
            //    }
            //}
        }
    }
}

pub fn player_pickup_skill_system(
    mut commands: Commands,
    mut player_skill_list: ResMut<AtomaticPlayerSkillList>,
    skill_query: Query<(Entity, &Transform, &SpriteSize), With<UI>>,
    player_query: Query<(&Transform, &SpriteSize), With<Player>>,
) {
    if let Ok((player_tf, player_size)) = player_query.get_single() {
        let player_scale = Vec2::from(player_tf.scale.xy());

        for (skill_entity, skill_tf, skill_size) in skill_query.iter() {
            let skill_scale = Vec2::from(skill_tf.scale.xy());

            //// Collision
            //let collision = collide(
            //    player_tf.translation,
            //    player_size.0 * player_scale,
            //    skill_tf.translation,
            //    skill_size.0 * skill_scale,
            //);

            //// perform collision
            //if let Some(_) = collision {
            //    commands.entity(skill_entity).despawn();
            //    player_skill_list.0.push(PlayerSkill {
            //        timer: Timer::new(Duration::from_millis(1500), TimerMode::Once),
            //    });
            //}
        }
    }
}
