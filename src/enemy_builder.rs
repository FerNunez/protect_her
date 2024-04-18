use crate::prelude::*;
use crate::systems::enemy::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, enemy_spawn_system)
            .add_systems(
                Update,
                (
                    //TODO: make this change in time
                    //enemy_spawn_system.run_if(on_timer(Duration::from_secs_f32(
                    //    1. / ENEMY_SPAWN_RATE_PER_MIN,
                    //))),
                    enemy_spawn_system.run_if(on_timer(Duration::from_secs(WAVE_TIMER))),
                    enemy_target_player.run_if(resource_exists::<PlayerState>),
                ),
            );
    }
}

//pub struct Wave{
//    pub level: i32,
//    pub number_spawn: i32,
//    pub game_timestamp: Duration
//}
//impl Default for Wave{
//
//    fn default() -> Self {
//        Self {
//            level: MAX_WAVE_LEVEL,
//            number_spawn: 0,
//            game_timestamp: Duration::new(10,0),
//
//        }
//    }
//}
//
//pub struct Waves{
//    pub wave: Vec<Wave>,
//}
//

//pub fn run_if_time_spawn(
//    time: Res<Time>,
//    wave_level: Res<WaveLevel>,
//)
//{
//    let game_duration = time.elapsed();
//    if (game_duration.as_secs() > )
//{
//    }
//    // spanw level^2 * monster
//
//}
