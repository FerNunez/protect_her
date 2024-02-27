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
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<(&Transform, &SpriteSize), With<Player>>,
) {
    if let Ok((player_tf, player_size)) = player_query.get_single() {
        let player_left = player_tf.translation.x - player_size.0.x * player_tf.scale.x;
        let player_right = player_tf.translation.x + player_size.0.x * player_tf.scale.x;

        let player_top = player_tf.translation.y + player_size.0.y * player_tf.scale.y;
        let player_bottom = player_tf.translation.y - player_size.0.y * player_tf.scale.y;

        if let Ok(mut camera_tf) = camera_query.get_single_mut() {
            let camera_left = camera_tf.translation.x - win_size.w / 2.;
            let camera_right = camera_tf.translation.x + win_size.w / 2.;

            let camera_top = camera_tf.translation.y + win_size.h / 2.;
            let camera_bottom = camera_tf.translation.y - win_size.h / 2.;

            if player_left < camera_left + CAMERA_WINDOWS_MARGIN {
                camera_tf.translation.x -= camera_left + CAMERA_WINDOWS_MARGIN - player_left;
            } else if player_right > camera_right - CAMERA_WINDOWS_MARGIN {
                camera_tf.translation.x -= camera_right - CAMERA_WINDOWS_MARGIN - player_right;
            }
            if player_top > camera_top - CAMERA_WINDOWS_MARGIN {
                camera_tf.translation.y -= camera_top - CAMERA_WINDOWS_MARGIN - player_top;
            } else if player_bottom < camera_bottom + CAMERA_WINDOWS_MARGIN {
                camera_tf.translation.y -= camera_bottom + CAMERA_WINDOWS_MARGIN - player_bottom;
            }
        }
    }
}
