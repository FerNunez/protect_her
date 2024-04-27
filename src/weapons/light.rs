use crate::get_mouse_pos_from_origin;
use crate::prelude::*;

//pub struct LightSaber{
//    size:
//}



pub fn light_spawn(
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
