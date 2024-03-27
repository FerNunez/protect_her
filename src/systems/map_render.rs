use crate::prelude::*;

pub fn render_map_system(mut commands: Commands, _map: Res<Map>, game_textures: Res<GameTextures>) {
    for y in (0..MAP_SIZE.1).step_by((TILE_SIZE.1 * TILE_SCALE) as usize) {
        for x in (0..MAP_SIZE.0).step_by((TILE_SIZE.0 * TILE_SCALE) as usize) {
            //let a_type = map.tiles[pos_to_map_idx(x as f32, y as f32)];

            commands.spawn(SpriteBundle {
                transform: Transform::from_xyz(x as f32, y as f32, 0.)
                    .with_scale(Vec3::new(TILE_SCALE as f32, TILE_SCALE as f32, 1.)),
                texture: game_textures.floor.clone(),
                ..Default::default()
            });
        }
    }
}
