use crate::prelude::*;
use crate::systems::enemy::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(PreUpdate, enemy_debug_spawn_system)
            .add_systems(
                Update,
                (
                    //TODO: make this change in time
                    //enemy_spawn_system.run_if(on_timer(Duration::from_secs_f32(
                    //    1. / ENEMY_SPAWN_RATE_PER_MIN,
                    //))),
                    enemy_spawn_system.run_if(on_timer(Duration::from_secs(WAVE_TIMER))),
                    enemy_target_egg.run_if(resource_exists::<PlayerState>),
                    enemy_speed_scaling_system,
                ),
            );
    }
}
