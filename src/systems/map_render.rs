use bevy::utils::info;

use crate::prelude::*;

pub fn render_map_system(
    mut commands: Commands,
    map: Res<Map>,
    game_textures: Res<GameTextures>,
    game_atlases: Res<GameAtlaseLayouts>,
) {
    for y in 0..MAP_SIZE_IN_TILES.1 {
        for x in 0..MAP_SIZE_IN_TILES.0 {
            match map.tiles[tile_xy_to_map_idx(x, y)] {
                TilesType::Wall => {
                    let index = 75;
                    commands.spawn(SpriteSheetBundle {
                        transform: Transform::from_xyz(
                            (x * TILE_SIZE.0) as f32,
                            (y * TILE_SIZE.1) as f32,
                            0.,
                        ),
                        texture: game_textures.map_texture.clone(),
                        atlas: TextureAtlas {
                            layout: game_atlases.map.clone(),
                            index,
                        },
                        ..Default::default()
                    });
                }
                TilesType::Floor => {
                    let tile_xy = IVec2::new(x, y);
                    let lava_vecinities = get_lava_vecinities(&map, &tile_xy);

                    if let Some(lava_vecinities) = lava_vecinities {
                        let mut index = 76;

                        if lava_vecinities.len() == 1 {
                            if lava_vecinities.contains(&LavaVecinity::NE) {
                                index = 11 * 2 + 5;
                            }
                            if lava_vecinities.contains(&LavaVecinity::NW) {
                                index = 11 * 2 + 6;
                            }
                            if lava_vecinities.contains(&LavaVecinity::SE) {
                                index = 11 * 1 + 5;
                            }
                            if lava_vecinities.contains(&LavaVecinity::SW) {
                                index = 11 * 1 + 6;
                            }
                        } else if lava_vecinities.len() == 2 {
                            if lava_vecinities.contains(&LavaVecinity::NW)
                                && lava_vecinities.contains(&LavaVecinity::NE)
                            {
                                index = 11 * 2 + 8;
                            } else if lava_vecinities.contains(&LavaVecinity::SW)
                                && lava_vecinities.contains(&LavaVecinity::SE)
                            {
                                index = 11 * 1 + 8;
                            } else if lava_vecinities.contains(&LavaVecinity::NW)
                                && lava_vecinities.contains(&LavaVecinity::SE)
                            {
                                index = 11 * 0 + 9;
                            } else if lava_vecinities.contains(&LavaVecinity::NW)
                                && lava_vecinities.contains(&LavaVecinity::SW)
                            {
                                index = 11 * 4 + 6;
                            } else if lava_vecinities.contains(&LavaVecinity::NE)
                                && lava_vecinities.contains(&LavaVecinity::SW)
                            {
                                index = 11 * 1 + 9;
                            } else if lava_vecinities.contains(&LavaVecinity::NE)
                                && lava_vecinities.contains(&LavaVecinity::SE)
                            {
                                index = 11 * 4 + 5;
                            }
                        } else if lava_vecinities.len() == 3 {
                            if lava_vecinities.contains(&LavaVecinity::NE)
                                && lava_vecinities.contains(&LavaVecinity::NW)
                                && lava_vecinities.contains(&LavaVecinity::SW)
                            {
                                index = 11 * 2 + 9;
                            } else if lava_vecinities.contains(&LavaVecinity::NE)
                                && lava_vecinities.contains(&LavaVecinity::NW)
                                && lava_vecinities.contains(&LavaVecinity::SE)
                            {
                                index = 11 * 2 + 10;
                            } else if lava_vecinities.contains(&LavaVecinity::NW)
                                && lava_vecinities.contains(&LavaVecinity::SW)
                                && lava_vecinities.contains(&LavaVecinity::SE)
                            {
                                index = 11 * 3 + 9;
                            } else if lava_vecinities.contains(&LavaVecinity::NE)
                                && lava_vecinities.contains(&LavaVecinity::SW)
                                && lava_vecinities.contains(&LavaVecinity::SE)
                            {
                                index = 11 * 3 + 10;
                            }
                        } else if lava_vecinities.len() == 4 {
                            if lava_vecinities.contains(&LavaVecinity::NE)
                                && lava_vecinities.contains(&LavaVecinity::NW)
                                && lava_vecinities.contains(&LavaVecinity::SE)
                                && lava_vecinities.contains(&LavaVecinity::SW)
                            {
                                index = 11 * 4 + 9;
                            }
                        }

                        if index >= 75 {
                            if lava_vecinities.contains(&LavaVecinity::N) {
                                if lava_vecinities.contains(&LavaVecinity::E)
                                    && lava_vecinities.contains(&LavaVecinity::S)
                                {
                                    index = 11 * 3 + 2;
                                } else if lava_vecinities.contains(&LavaVecinity::W)
                                    && lava_vecinities.contains(&LavaVecinity::S)
                                {
                                    index = 11 * 3 + 0;
                                } else if lava_vecinities.contains(&LavaVecinity::S) {
                                    index = 11 * 3 + 1;
                                } else if lava_vecinities.contains(&LavaVecinity::W)
                                    && lava_vecinities.contains(&LavaVecinity::E)
                                {
                                    index = 11 * 0 + 3;
                                } else if lava_vecinities.contains(&LavaVecinity::W)
                                    && lava_vecinities.contains(&LavaVecinity::S)
                                    && lava_vecinities.contains(&LavaVecinity::E)
                                {
                                    index = 11 * 3 + 3;
                                } else if lava_vecinities.contains(&LavaVecinity::W)
                                    && lava_vecinities.contains(&LavaVecinity::SE)
                                    && !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                {
                                    index = 11 * 0 + 4;
                                } else if !lava_vecinities.contains(&LavaVecinity::W)
                                    && lava_vecinities.contains(&LavaVecinity::SE)
                                    && !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                {
                                    index = 11 * 0 + 5;
                                } else if !lava_vecinities.contains(&LavaVecinity::W)
                                    && lava_vecinities.contains(&LavaVecinity::SW)
                                    && !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                {
                                    index = 11 * 0 + 6;
                                } else if !lava_vecinities.contains(&LavaVecinity::W)
                                    && lava_vecinities.contains(&LavaVecinity::SW)
                                    && lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                {
                                    index = 11 * 0 + 7;
                                } else if !lava_vecinities.contains(&LavaVecinity::W)
                                    && lava_vecinities.contains(&LavaVecinity::SW)
                                    && lava_vecinities.contains(&LavaVecinity::SE)
                                    && !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                {
                                    index = 11 * 0 + 7;
                                } else if !lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::SW)
                                    && !lava_vecinities.contains(&LavaVecinity::SE)
                                    && !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                {
                                    index = 11 * 0 + 1;
                                } else if lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                    && !lava_vecinities.contains(&LavaVecinity::SW)
                                {
                                    index = 2;
                                } else if lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                    && !lava_vecinities.contains(&LavaVecinity::SE)
                                {
                                    index = 0;
                                }
                            } else if lava_vecinities.contains(&LavaVecinity::S) {
                                if lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::NE)
                                {
                                    index = 11 * 2 + 0;
                                } else if lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::NW)
                                {
                                    index = 11 * 2 + 2;
                                } else if lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::E)
                                    && lava_vecinities.contains(&LavaVecinity::NE)
                                {
                                    index = 11 * 3 + 4;
                                } else if lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::W)
                                    && lava_vecinities.contains(&LavaVecinity::NW)
                                {
                                    index = 11 * 3 + 7;
                                } else if !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::W)
                                    && lava_vecinities.contains(&LavaVecinity::NW)
                                    && lava_vecinities.contains(&LavaVecinity::NE)
                                {
                                    index = 11 * 3 + 8;
                                } else if !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::W)
                                    && lava_vecinities.contains(&LavaVecinity::NW)
                                    && !lava_vecinities.contains(&LavaVecinity::NE)
                                {
                                    index = 11 * 3 + 6;
                                } else if !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::NW)
                                    && lava_vecinities.contains(&LavaVecinity::NE)
                                {
                                    index = 11 * 3 + 5;
                                } else if !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::NW)
                                    && !lava_vecinities.contains(&LavaVecinity::NE)
                                    && !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::W)
                                {
                                    index = 11 * 2 + 1;
                                } else if lava_vecinities.contains(&LavaVecinity::E)
                                    && lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                {
                                    index = 11 * 2 + 3;
                                }
                            } else if lava_vecinities.contains(&LavaVecinity::E) {
                                if !lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                    && !lava_vecinities.contains(&LavaVecinity::NW)
                                    && !lava_vecinities.contains(&LavaVecinity::SW)
                                {
                                    index = 11 * 1 + 2;
                                } else if !lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                    && !lava_vecinities.contains(&LavaVecinity::NW)
                                    && lava_vecinities.contains(&LavaVecinity::SW)
                                {
                                    index = 11 * 1 + 7;
                                } else if !lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                    && lava_vecinities.contains(&LavaVecinity::NW)
                                    && !lava_vecinities.contains(&LavaVecinity::SW)
                                {
                                    index = 11 * 2 + 7;
                                } else if !lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                    && lava_vecinities.contains(&LavaVecinity::NW)
                                    && lava_vecinities.contains(&LavaVecinity::SW)
                                {
                                    index = 11 * 4 + 7;
                                } else if lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                {
                                    index = 11 * 1 + 3;
                                } else if lava_vecinities.contains(&LavaVecinity::W)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && lava_vecinities.contains(&LavaVecinity::S)
                                {
                                    index = 11 * 2 + 3;
                                }
                            } else if lava_vecinities.contains(&LavaVecinity::W) {
                                if !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                    && !lava_vecinities.contains(&LavaVecinity::NE)
                                    && !lava_vecinities.contains(&LavaVecinity::SE)
                                {
                                    index = 11 * 1 + 0;
                                } else if !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                    && lava_vecinities.contains(&LavaVecinity::NE)
                                    && lava_vecinities.contains(&LavaVecinity::SE)
                                {
                                    index = 11 * 4 + 4;
                                } else if !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                    && lava_vecinities.contains(&LavaVecinity::NE)
                                    && !lava_vecinities.contains(&LavaVecinity::SE)
                                {
                                    index = 11 * 2 + 4;
                                } else if !lava_vecinities.contains(&LavaVecinity::E)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                    && !lava_vecinities.contains(&LavaVecinity::NE)
                                    && lava_vecinities.contains(&LavaVecinity::SE)
                                {
                                    index = 11 * 1 + 4;
                                } else if lava_vecinities.contains(&LavaVecinity::E)
                                    && lava_vecinities.contains(&LavaVecinity::N)
                                    && !lava_vecinities.contains(&LavaVecinity::SW)
                                    && !lava_vecinities.contains(&LavaVecinity::SE)
                                    && !lava_vecinities.contains(&LavaVecinity::S)
                                {
                                    index = 11 * 0 + 3;
                                } else if lava_vecinities.contains(&LavaVecinity::E)
                                    && lava_vecinities.contains(&LavaVecinity::S)
                                    && !lava_vecinities.contains(&LavaVecinity::NW)
                                    && !lava_vecinities.contains(&LavaVecinity::NE)
                                    && !lava_vecinities.contains(&LavaVecinity::N)
                                {
                                    index = 11 * 2 + 3;
                                }
                            }
                        }

                        commands.spawn(SpriteSheetBundle {
                            transform: Transform::from_xyz(
                                (x * TILE_SIZE.0) as f32,
                                (y * TILE_SIZE.1) as f32,
                                0.,
                            ),
                            texture: game_textures.map_texture.clone(),
                            atlas: TextureAtlas {
                                layout: game_atlases.map.clone(),
                                index,
                            },
                            ..Default::default()
                        });
                    } else {
                        commands.spawn(SpriteBundle {
                            transform: Transform::from_xyz(
                                (x * TILE_SIZE.0) as f32,
                                (y * TILE_SIZE.1) as f32,
                                0.,
                            ),
                            texture: game_textures.floor.clone(),
                            ..Default::default()
                        });
                    }
                }
            };
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

fn get_lava_vecinities(map: &Map, tile_pos: &IVec2) -> Option<HashSet<LavaVecinity>> {
    let mut lava_vecinity_hash: HashSet<LavaVecinity> = HashSet::new();

    for i in -1..=1 {
        for j in -1..=1 {
            let tuple = (i, j);
            let position = IVec2::new(tile_pos.x + i, tile_pos.y + j);

            if map.tile_in_bound(&position) {
                let tile_type = map.tiles[tile_xy_to_map_idx(position.x, position.y)];

                if tile_type == TilesType::Wall {
                    match tuple {
                        (-1, 1) => {
                            lava_vecinity_hash.insert(LavaVecinity::NW);
                        }
                        (0, 1) => {
                            lava_vecinity_hash.insert(LavaVecinity::N);
                        }
                        (1, 1) => {
                            lava_vecinity_hash.insert(LavaVecinity::NE);
                        }
                        (-1, 0) => {
                            lava_vecinity_hash.insert(LavaVecinity::W);
                        }
                        (1, 0) => {
                            lava_vecinity_hash.insert(LavaVecinity::E);
                        }
                        (-1, -1) => {
                            lava_vecinity_hash.insert(LavaVecinity::SW);
                        }
                        (0, -1) => {
                            lava_vecinity_hash.insert(LavaVecinity::S);
                        }
                        (1, -1) => {
                            lava_vecinity_hash.insert(LavaVecinity::SE);
                        }
                        (_, _) => (),
                    }
                }
            }
        }
    }

    if lava_vecinity_hash.is_empty() {
        None
    } else {
        Some(lava_vecinity_hash)
    }
}

