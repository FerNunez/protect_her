use crate::prelude::*;

pub fn zoom_system(
    game_state: ResMut<GameState>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for mut projection in camera_query.iter_mut() {
        projection.scale = game_state.zoom;
    }
}

pub fn move_camera_system(
    win_size: Res<WinSize>,
    mut camera_query: Query<
        (&mut Transform, &OrthographicProjection),
        (With<Camera>, Without<Player>),
    >,
    player_query: Query<(&Transform, &SpriteSize), With<Player>>,
) {
    if let Ok((player_tf, player_size)) = player_query.get_single() {
        let player_left = player_tf.translation.x - player_size.0.x * player_tf.scale.x;
        let player_right = player_tf.translation.x + player_size.0.x * player_tf.scale.x;

        let player_top = player_tf.translation.y + player_size.0.y * player_tf.scale.y;
        let player_bottom = player_tf.translation.y - player_size.0.y * player_tf.scale.y;

        if let Ok((mut camera_tf, ortho_projection)) = camera_query.get_single_mut() {
            println!("ortho_projection min: {}", ortho_projection.area.min);
            println!("ortho_projection max: {}", ortho_projection.area.max);
            println!("ortho_projection scale: {}", ortho_projection.scale);
            println!(
                "ortho_projection viewport_origin: {}",
                ortho_projection.viewport_origin
            );

            let win_w = ortho_projection.area.max.x - ortho_projection.area.min.x;
            let win_h = ortho_projection.area.max.y - ortho_projection.area.min.y;

            let camera_margin = Vec2::new(
                win_w * CAMERA_MARGIN_RATIO.0,
                win_h * CAMERA_MARGIN_RATIO.1,
            );

            println!("camera_margin: {}", camera_margin);
            let camera_left = camera_tf.translation.x - win_size.w / 2.;
            let camera_right = camera_tf.translation.x + win_size.w / 2.;

            let camera_top = camera_tf.translation.y + win_size.h / 2.;
            let camera_bottom = camera_tf.translation.y - win_size.h / 2.;

            if player_left < camera_left + camera_margin.x {
                camera_tf.translation.x -= camera_left + camera_margin.x - player_left;
            } else if player_right > camera_right - camera_margin.x {
                camera_tf.translation.x -= camera_right - camera_margin.x - player_right;
            }
            if player_top > camera_top - camera_margin.y {
                camera_tf.translation.y -= camera_top - camera_margin.y - player_top;
            } else if player_bottom < camera_bottom + camera_margin.y {
                camera_tf.translation.y -= camera_bottom + camera_margin.y - player_bottom;
            }

            println!("camera_pos: {}", camera_tf.translation);
        }
    }
}
