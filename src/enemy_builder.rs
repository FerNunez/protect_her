use crate::prelude::*;
use crate::systems::enemy::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                enemy_spawn_system.run_if(on_timer(Duration::from_secs_f32(
                    1. / ENEMY_SPAWN_RATE_PER_MIN,
                ))),
                //enemy_spawn_system.run_if(on_timer(Duration::from_secs_f32( 5.))),
                enemy_target_player.run_if(resource_exists::<PlayerState>),
            ),
        );
    }
}
