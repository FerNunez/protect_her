
use crate::prelude::*;

use crate::systems::player::*;

impl Default for PlayerState {
    fn default() -> Self {
        Self { alive: false }
    }
}

impl PlayerState {
    //fn dead(&mut self) {
    //    self.alive = false;
    //}
    pub fn spawned(&mut self) {
        self.alive = true;
    }
}

// Player
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_spawn_system.run_if(on_timer(Duration::from_secs_f64(0.1))),
                player_keyboard_event_system,
                player_fire_system,
                player_keyboard_dash_system,
                player_laser_hit_enemy_system,
                //player_dash_system,
            ),
        );
    }
}
