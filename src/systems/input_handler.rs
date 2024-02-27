use crate::prelude::*;

pub fn user_mouse_handler_zoom_event_system(
    mut scroll_evr: EventReader<MouseWheel>,
    mut game_state: ResMut<GameState>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                game_state.zoom += ev.y / 10.;
            }
            MouseScrollUnit::Pixel => {}
        }
    }

    if game_state.zoom <= 0.0 {
        game_state.zoom = 0.1;
    }
}
