//use std::f32::consts::PI;
//use std::time::Duration;
//
//use bevy::math::Vec2;
//use bevy::window::PrimaryWindow;
//use bevy::{prelude::*, time::common_conditions::on_timer};


use crate::prelude::*;



pub fn player_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut automatic_player_skill_list: ResMut<AtomaticPlayerSkillList>,
    mut player_skill: ResMut<PlayerSkill>,
    time: Res<Time>,
    win_size: Res<WinSize>,
    kboard: Res<Input<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        // get vector velocity
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
        if kboard.pressed(KeyCode::ControlLeft) {
            // probably set here a OPP? single call

            player_skill.timer.tick(time.delta());
            if player_skill.timer.finished() {
                player_skill.timer.reset();
                commands
                    .spawn(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        // TODO: player_y as a part of the SPRITE?
                        transform: Transform::from_xyz(player_position.x, player_position.y, 0.)
                            .with_scale(Vec3::new(PLAYER_LASER_SCALE, PLAYER_LASER_SCALE, 0.))
                            .with_rotation(Quat::from_rotation_z(angle)),
                        ..Default::default()
                    })
                    .insert(Movable)
                    .insert(velocity)
                    .insert(FromPlayer)
                    .insert(Laser)
                    .insert(Damage(PLAYER_DAMAGE))
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE));
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
                        .insert(Laser)
                        .insert(Damage(PLAYER_DAMAGE))
                        .insert(SpriteSize::from(PLAYER_LASER_SIZE));
                }
            }
        }
    }
}

pub fn player_keyboard_dash_system(
    kboard: Res<Input<KeyCode>>,
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

            // Collisioin
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

pub fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut player_state: ResMut<PlayerState>,
) {
    if !player_state.alive {
        commands
            .spawn(SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_scale(Vec3::new(EGG_SCALE, EGG_SCALE, 0.)),
                ..Default::default()
            })
            .insert(Player)
            .insert(Movable)
            .insert(SpriteSize::from(EGG_SIZE))
            .insert(Velocity { x: 0., y: 0. })
            .insert(CanDash);
        player_state.spawned();
    }
}

pub fn player_keyboard_event_system(
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
