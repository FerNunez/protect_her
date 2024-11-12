use crate::prelude::*;

use bevy::input::mouse::MouseScrollUnit;
pub fn user_mouse_handler_zoom_event_system(
    mut scroll_evr: EventReader<MouseWheel>,
    mut game_state: ResMut<GameState>,
) {

    if !scroll_evr.is_empty(){
        let result = scroll_evr.read_with_id();
        for ev in result.into_iter(){
                game_state.zoom += ev.0.y / 10.;
            println!("game_state.zoom = {}", game_state.zoom );
        }

    }
//    for ev in scroll_evr.iter(){
//        match ev.unit {
//            MouseScrollUnit::Line => {
//                game_state.zoom += ev.y / 10.;
//            }
//            MouseScrollUnit::Pixel => {}
//        }
//    }

    if game_state.zoom <= 0.0 {
        game_state.zoom = 0.1;
    }
}
