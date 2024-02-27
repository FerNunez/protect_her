use crate::prelude::*;
use crate::systems::enemy::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                enemy_spawn_system.run_if(on_timer(Duration::from_secs_f64(1.))),
                enemy_target_player.run_if(resource_exists::<PlayerState>()),
            ),
        );
    }
}
