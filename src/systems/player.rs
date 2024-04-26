use crate::prelude::*;
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};

fn get_mouse_pos_from_origin(
    mouse_position_from_window: Vec2,
    window_size: Vec2,
    camera_position: Vec3,
) -> Vec2 {
    // mouse pos from window frame (inverted y) to map frame
    let mouse_pos_from_camera_corner = Vec2::new(
        mouse_position_from_window.x,
        window_size.y - mouse_position_from_window.y,
    );

    let win_size_gap = Vec2::new(window_size.x, window_size.y);
    let camera_pos = Vec2::new(camera_position.x, camera_position.y);
    let camera_corner_pos = camera_pos - (win_size_gap / 2.);
    let mouse_pos_from_origin = camera_corner_pos + mouse_pos_from_camera_corner;
    mouse_pos_from_origin
}
pub fn player_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut automatic_player_skill_list: ResMut<AtomaticPlayerSkillList>,
    mut player_skill: ResMut<PlayerSkill>,
    time: Res<Time>,
    win_size: Res<WinSize>,
    mut last_mouse: ResMut<LastMouse>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    query: Query<&Transform, (With<Player>, Without<InEdit>)>,
    camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_tf) = query.get_single() {
        let player_position = Vec2::new(player_tf.translation.x, player_tf.translation.y);
        // get vector velocity
        if let Ok(camera_tf) = camera_query.get_single() {
            //
            let mouse_position_from_window = match window_query.single().cursor_position() {
                Some(mouse_pos) => {
                    last_mouse.pos = mouse_pos;
                    mouse_pos
                }
                None => last_mouse.pos,
            };

            let win_size_gap = Vec2::new(win_size.w, win_size.h);
            let mouse_position = get_mouse_pos_from_origin(
                mouse_position_from_window,
                win_size_gap,
                camera_tf.translation,
            );
            let direction_vector_normalized = (mouse_position
                - Vec2::new(player_position.x, player_position.y))
            .normalize_or_zero();

            // NOTE: not sure why negative angle
            let angle =
                -direction_vector_normalized.angle_between(Vec2 { x: 1.0, y: 0.0 }) - PI / 2.;

            let velocity = Velocity {
                x: direction_vector_normalized.x * PLAYER_LASER_SPEED,
                y: direction_vector_normalized.y * PLAYER_LASER_SPEED,
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
                        .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                        .insert(CanFly);
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

pub fn player_keyboard_edit_terrain(
    kboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    in_edit_query: Query<(Entity, &InEdit)>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        if kboard.just_pressed(KeyCode::KeyQ) {
            let in_edit = in_edit_query.get(player_entity);
            if in_edit.is_ok() {
                commands.entity(player_entity).remove::<InEdit>();
                //info!("Remove InEdit");
            } else {
                commands.entity(player_entity).insert(InEdit);
                //info!("Insert InEdit");
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
    game_atlases: Res<GameAtlaseLayouts>,
    mut player_state: ResMut<PlayerState>,
) {
    if !player_state.alive {
        let animation = Animation::new(0, 1);

        commands
            .spawn(SpriteSheetBundle {
                transform: Transform::from_xyz(
                    (MAP_SIZE_IN_TILES.0 * TILE_SIZE.0 / 2) as f32,
                    (MAP_SIZE_IN_TILES.1 * TILE_SIZE.1 / 2) as f32,
                    1.0,
                ),
                texture: game_textures.player_animation.clone(),
                atlas: TextureAtlas {
                    layout: game_atlases.player_animation.clone(),
                    index: animation.first_index,
                },
                ..Default::default()
            })
            .insert(PlayerAnimationState::Idle)
            .insert(animation)
            .insert(AnimationTimer::new_from_millis(300))
            .insert(Player)
            .insert(Movable)
            .insert(FacingDirection::from(Vec2::ZERO))
            .insert(SpriteSize::from(EGG_SIZE))
            .insert(Velocity { x: 0., y: 0. })
            .insert(CanWallRide)
            .insert(CanDash);

        player_state.spawned();
    }
}

pub fn player_keyboard_event_system(
    time: Res<Time>,
    mut commands: Commands,
    kboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (Entity, &mut Transform, &mut Velocity),
        (With<Player>, Without<AskingToMove>),
    >,
) {
    if let Ok((entity, transform, mut velocity)) = query.get_single_mut() {
        let delta_time = time.delta().as_secs_f32();

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

        let delta = Vec2::new(velocity.x * delta_time, velocity.y * delta_time) * PLAYER_SPEED;

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

        let extra = Vec2::new(velocity.x, velocity.y).length();
        //let angle = (0.01 + extra) + transform.rotation.to_euler(EulerRot::YXZ).2;

        let angle = transform.rotation.to_euler(EulerRot::YXZ).2;

        commands.spawn(WantsToRotate { entity, angle });
    }
}

pub fn player_modify_map_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_in_edit_query: Query<(Entity, &Transform), (With<Player>, With<InEdit>)>,
    camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut last_mouse: ResMut<LastMouse>,
    map: Res<Map>,
    game_textures: Res<GameTextures>, // DEBUG
) {
    if let Ok((player_entity, _player_tf)) = player_in_edit_query.get_single() {
        if let Ok(camera_tf) = camera_query.get_single() {
            let mouse_position_from_window = match window_query.single().cursor_position() {
                Some(mouse_pos) => {
                    last_mouse.pos = mouse_pos;
                    mouse_pos
                }
                None => last_mouse.pos,
            };
            let win_size_gap = Vec2::new(win_size.w, win_size.h);
            let mouse_position = get_mouse_pos_from_origin(
                mouse_position_from_window,
                win_size_gap,
                camera_tf.translation,
            );
            let mouse_poistion_minus_tile =
                mouse_position + Vec2::new(TILE_SIZE.0 as f32 / 2., TILE_SIZE.1 as f32 / 2.);

            if mouse_button.pressed(MouseButton::Left) {
                if !map.can_enter_tile(&mouse_poistion_minus_tile) {
                    //info!(
                    //    "Entt:{:?}, Spawning. UpdateTile: pos: {:?}, asking to Wall",
                    //    player_entity, mouse_position
                    //);
                    commands.spawn(UpdateTile {
                        from_entity: player_entity,
                        position: mouse_poistion_minus_tile,
                        tiletype: TilesType::Floor,
                    });
                }
            }
        }
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

pub fn player_update_animation(
    mut player_query: Query<
        (
            //&FacingDirection,
            &Velocity,
            //&mut PlayerAnimation,
            &mut TextureAtlas,
            &mut Animation,
        ),
        With<Player>,
    >,
) {
    //if let Ok((facing_direction, velocity, player_animation, mut animation)) =

    if let Ok((velocity, mut texture, mut animation)) = player_query.get_single_mut() {
        info!("Player_update_anim");
        let mut new_animation: Option<Animation> = None;
        if velocity.x == 0. {
            if velocity.y < 0. {
                info!("vel y> 0");
                new_animation = Some(Animation::new(0, 3));
            } else if velocity.y > 0. {
                info!("vel y < 0");
                new_animation = Some(Animation::new(6, 9));
            } else {
                info!("vel y = 0");
                new_animation = Some(Animation::new(0, 1));
            }
        } else if velocity.x > 0. {
            info!("vel x > 0");
            let mut animation = Animation::new(12, 15);
            animation.set_flip(true);
            new_animation = Some(animation);
        } else {
            info!("vel x < 0");
            let mut animation = Animation::new(12, 15);
            animation.set_flip(false);
            new_animation = Some(animation);
        };

        if let Some(new_animation) = new_animation {
            info!("New animation: {:?}", new_animation.first_index);
            if !new_animation.same_index(&animation) {
                info!("Animation Updated");

                texture.index = new_animation.first_index;
                *animation = new_animation;
                info!("Animation {:?}", animation.flip);
            }
        } else {
        }
    }
}
