use crate::prelude::*;

pub fn render_map_system(mut commands: Commands, map: Res<Map>, game_textures: Res<GameTextures>) {
    for y in 0..MAP_SIZE_IN_TILES.1 {
        for x in 0..MAP_SIZE_IN_TILES.0 {
            let texture = match map.tiles[tile_xy_to_map_idx(x, y)] {
                TilesType::Wall => game_textures.wall.clone(),
                TilesType::Floor => game_textures.floor.clone(),
            };

            commands.spawn(SpriteBundle {
                transform: Transform::from_xyz(
                    (x * TILE_SIZE.0) as f32,
                    (y * TILE_SIZE.1) as f32,
                    0.,
                )
                .with_scale(Vec3::new(TILE_SCALE as f32, TILE_SCALE as f32, 1.)),
                texture,
                ..Default::default()
            });
        }
    }

    //Debug
    commands.spawn(SpriteBundle {
        texture: game_textures.pixel.clone(),
        transform: Transform::from_xyz(
            (MAP_SIZE_IN_TILES.0 * TILE_SIZE.0 / 2) as f32,
            (MAP_SIZE_IN_TILES.1 * TILE_SIZE.1 / 2) as f32,
            2.,
        ),
        ..Default::default()
    });
}
