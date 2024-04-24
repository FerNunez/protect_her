use crate::prelude::*;

pub fn render_map_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    game_textures: Res<GameTextures>,
    game_atlases: Res<GameAtlaseLayouts>,
) {
    for y in 0..MAP_SIZE_IN_TILES.1 {
        for x in 0..MAP_SIZE_IN_TILES.0 {
            match map.tiles[tile_xy_to_map_idx(x, y)] {
                TilesType::Lava => {
                    let index = 75;
                    let id = commands
                        .spawn(SpriteSheetBundle {
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
                        })
                        .id();
                    map.ids[tile_xy_to_map_idx(x, y)] = Some(id);
                }
                TilesType::Floor => {
                    let tile_xy = IVec2::new(x, y);
                    let lava_vecinities = get_lava_vecinities(&map, &tile_xy);

                    if let Some(lava_vecinities) = lava_vecinities {
                        let index = get_index_from_vecinity(&lava_vecinities);
                        let id = commands
                            .spawn(SpriteSheetBundle {
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
                            })
                            .id();
                        map.ids[tile_xy_to_map_idx(x, y)] = Some(id);
                    } else {
                        let id = commands
                            .spawn(SpriteBundle {
                                transform: Transform::from_xyz(
                                    (x * TILE_SIZE.0) as f32,
                                    (y * TILE_SIZE.1) as f32,
                                    0.,
                                ),
                                texture: game_textures.floor.clone(),
                                ..Default::default()
                            })
                            .id();
                        map.ids[tile_xy_to_map_idx(x, y)] = Some(id);
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

pub fn get_index_from_vecinity(lava_vecinities: &HashSet<CardinalDirections>) -> usize {
    let mut index = 76;

    if lava_vecinities.len() == 1 {
        if lava_vecinities.contains(&CardinalDirections::NE) {
            index = 11 * 2 + 5;
        }
        if lava_vecinities.contains(&CardinalDirections::NW) {
            index = 11 * 2 + 6;
        }
        if lava_vecinities.contains(&CardinalDirections::SE) {
            index = 11 * 1 + 5;
        }
        if lava_vecinities.contains(&CardinalDirections::SW) {
            index = 11 * 1 + 6;
        }
    } else if lava_vecinities.len() == 2 {
        if lava_vecinities.contains(&CardinalDirections::NW)
            && lava_vecinities.contains(&CardinalDirections::NE)
        {
            index = 11 * 2 + 8;
        } else if lava_vecinities.contains(&CardinalDirections::SW)
            && lava_vecinities.contains(&CardinalDirections::SE)
        {
            index = 11 * 1 + 8;
        } else if lava_vecinities.contains(&CardinalDirections::NW)
            && lava_vecinities.contains(&CardinalDirections::SE)
        {
            index = 11 * 0 + 9;
        } else if lava_vecinities.contains(&CardinalDirections::NW)
            && lava_vecinities.contains(&CardinalDirections::SW)
        {
            index = 11 * 4 + 6;
        } else if lava_vecinities.contains(&CardinalDirections::NE)
            && lava_vecinities.contains(&CardinalDirections::SW)
        {
            index = 11 * 1 + 9;
        } else if lava_vecinities.contains(&CardinalDirections::NE)
            && lava_vecinities.contains(&CardinalDirections::SE)
        {
            index = 11 * 4 + 5;
        }
    } else if lava_vecinities.len() == 3 {
        if lava_vecinities.contains(&CardinalDirections::NE)
            && lava_vecinities.contains(&CardinalDirections::NW)
            && lava_vecinities.contains(&CardinalDirections::SW)
        {
            index = 11 * 2 + 9;
        } else if lava_vecinities.contains(&CardinalDirections::NE)
            && lava_vecinities.contains(&CardinalDirections::NW)
            && lava_vecinities.contains(&CardinalDirections::SE)
        {
            index = 11 * 2 + 10;
        } else if lava_vecinities.contains(&CardinalDirections::NW)
            && lava_vecinities.contains(&CardinalDirections::SW)
            && lava_vecinities.contains(&CardinalDirections::SE)
        {
            index = 11 * 3 + 9;
        } else if lava_vecinities.contains(&CardinalDirections::NE)
            && lava_vecinities.contains(&CardinalDirections::SW)
            && lava_vecinities.contains(&CardinalDirections::SE)
        {
            index = 11 * 3 + 10;
        }
    } else if lava_vecinities.len() == 4 {
        if lava_vecinities.contains(&CardinalDirections::NE)
            && lava_vecinities.contains(&CardinalDirections::NW)
            && lava_vecinities.contains(&CardinalDirections::SE)
            && lava_vecinities.contains(&CardinalDirections::SW)
        {
            index = 11 * 4 + 9;
        }
    } else if lava_vecinities.len() >= 8 {
        let mut rng = thread_rng();
        match rng.gen_range(0..=2) {
            0 => index = 11 * 3 + 3,
            1 => index = 11 * 4 + 3,
            _ => index = 11 * 4 + 2,
        }
    }

    if index > 75 {
        if lava_vecinities.contains(&CardinalDirections::N) {
            if lava_vecinities.contains(&CardinalDirections::E)
                && lava_vecinities.contains(&CardinalDirections::S)
            {
                index = 11 * 3 + 2;
            } else if lava_vecinities.contains(&CardinalDirections::W)
                && lava_vecinities.contains(&CardinalDirections::S)
            {
                index = 11 * 3 + 0;
            } else if lava_vecinities.contains(&CardinalDirections::S) {
                index = 11 * 3 + 1;
            } else if lava_vecinities.contains(&CardinalDirections::W)
                && lava_vecinities.contains(&CardinalDirections::E)
            {
                index = 11 * 0 + 3;
            } else if lava_vecinities.contains(&CardinalDirections::W)
                && lava_vecinities.contains(&CardinalDirections::S)
                && lava_vecinities.contains(&CardinalDirections::E)
            {
                index = 11 * 3 + 3;
            } else if lava_vecinities.contains(&CardinalDirections::W)
                && lava_vecinities.contains(&CardinalDirections::SE)
                && !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::S)
            {
                index = 11 * 0 + 4;
            } else if !lava_vecinities.contains(&CardinalDirections::W)
                && lava_vecinities.contains(&CardinalDirections::SE)
                && !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::S)
            {
                index = 11 * 0 + 5;
            } else if !lava_vecinities.contains(&CardinalDirections::W)
                && lava_vecinities.contains(&CardinalDirections::SW)
                && !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::S)
            {
                index = 11 * 0 + 6;
            } else if !lava_vecinities.contains(&CardinalDirections::W)
                && lava_vecinities.contains(&CardinalDirections::SW)
                && lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::S)
            {
                index = 11 * 0 + 7;
            } else if !lava_vecinities.contains(&CardinalDirections::W)
                && lava_vecinities.contains(&CardinalDirections::SW)
                && lava_vecinities.contains(&CardinalDirections::SE)
                && !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::S)
            {
                index = 11 * 0 + 7;
            } else if !lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::SW)
                && !lava_vecinities.contains(&CardinalDirections::SE)
                && !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::S)
            {
                index = 11 * 0 + 1;
            } else if lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::S)
                && !lava_vecinities.contains(&CardinalDirections::SW)
            {
                index = 2;
            } else if lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::S)
                && !lava_vecinities.contains(&CardinalDirections::SE)
            {
                index = 0;
            }
        } else if lava_vecinities.contains(&CardinalDirections::S) {
            if lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::NE)
            {
                index = 11 * 2 + 0;
            } else if lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::NW)
            {
                index = 11 * 2 + 2;
            } else if lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::E)
                && lava_vecinities.contains(&CardinalDirections::NE)
            {
                index = 11 * 3 + 4;
            } else if lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::W)
                && lava_vecinities.contains(&CardinalDirections::NW)
            {
                index = 11 * 3 + 7;
            } else if !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::W)
                && lava_vecinities.contains(&CardinalDirections::NW)
                && lava_vecinities.contains(&CardinalDirections::NE)
            {
                index = 11 * 3 + 8;
            } else if !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::W)
                && lava_vecinities.contains(&CardinalDirections::NW)
                && !lava_vecinities.contains(&CardinalDirections::NE)
            {
                index = 11 * 3 + 6;
            } else if !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::NW)
                && lava_vecinities.contains(&CardinalDirections::NE)
            {
                index = 11 * 3 + 5;
            } else if !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::NW)
                && !lava_vecinities.contains(&CardinalDirections::NE)
                && !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::W)
            {
                index = 11 * 2 + 1;
            } else if lava_vecinities.contains(&CardinalDirections::E)
                && lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::N)
            {
                index = 11 * 2 + 3;
            }
        } else if lava_vecinities.contains(&CardinalDirections::E) {
            if !lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::S)
                && !lava_vecinities.contains(&CardinalDirections::NW)
                && !lava_vecinities.contains(&CardinalDirections::SW)
            {
                index = 11 * 1 + 2;
            } else if !lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::S)
                && !lava_vecinities.contains(&CardinalDirections::NW)
                && lava_vecinities.contains(&CardinalDirections::SW)
            {
                index = 11 * 1 + 7;
            } else if !lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::S)
                && lava_vecinities.contains(&CardinalDirections::NW)
                && !lava_vecinities.contains(&CardinalDirections::SW)
            {
                index = 11 * 2 + 7;
            } else if !lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::S)
                && lava_vecinities.contains(&CardinalDirections::NW)
                && lava_vecinities.contains(&CardinalDirections::SW)
            {
                index = 11 * 4 + 7;
            } else if lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::S)
            {
                index = 11 * 1 + 3;
            } else if lava_vecinities.contains(&CardinalDirections::W)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && lava_vecinities.contains(&CardinalDirections::S)
            {
                index = 11 * 2 + 3;
            }
        } else if lava_vecinities.contains(&CardinalDirections::W) {
            if !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::S)
                && !lava_vecinities.contains(&CardinalDirections::NE)
                && !lava_vecinities.contains(&CardinalDirections::SE)
            {
                index = 11 * 1 + 0;
            } else if !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::S)
                && lava_vecinities.contains(&CardinalDirections::NE)
                && lava_vecinities.contains(&CardinalDirections::SE)
            {
                index = 11 * 4 + 4;
            } else if !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::S)
                && lava_vecinities.contains(&CardinalDirections::NE)
                && !lava_vecinities.contains(&CardinalDirections::SE)
            {
                index = 11 * 2 + 4;
            } else if !lava_vecinities.contains(&CardinalDirections::E)
                && !lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::S)
                && !lava_vecinities.contains(&CardinalDirections::NE)
                && lava_vecinities.contains(&CardinalDirections::SE)
            {
                index = 11 * 1 + 4;
            } else if lava_vecinities.contains(&CardinalDirections::E)
                && lava_vecinities.contains(&CardinalDirections::N)
                && !lava_vecinities.contains(&CardinalDirections::SW)
                && !lava_vecinities.contains(&CardinalDirections::SE)
                && !lava_vecinities.contains(&CardinalDirections::S)
            {
                index = 11 * 0 + 3;
            } else if lava_vecinities.contains(&CardinalDirections::E)
                && lava_vecinities.contains(&CardinalDirections::S)
                && !lava_vecinities.contains(&CardinalDirections::NW)
                && !lava_vecinities.contains(&CardinalDirections::NE)
                && !lava_vecinities.contains(&CardinalDirections::N)
            {
                index = 11 * 2 + 3;
            }
        }
    }
    index
}

fn get_lava_vecinities(map: &Map, tile_pos: &IVec2) -> Option<HashSet<CardinalDirections>> {
    let mut lava_vecinity_hash: HashSet<CardinalDirections> = HashSet::new();

    for i in -1..=1 {
        for j in -1..=1 {
            let tuple = (i, j);
            let position = IVec2::new(tile_pos.x + i, tile_pos.y + j);

            if map.tile_in_bound(&position) {
                let tile_type = map.tiles[tile_xy_to_map_idx(position.x, position.y)];

                if tile_type == TilesType::Lava {
                    match tuple {
                        (-1, 1) => {
                            lava_vecinity_hash.insert(CardinalDirections::NW);
                        }
                        (0, 1) => {
                            lava_vecinity_hash.insert(CardinalDirections::N);
                        }
                        (1, 1) => {
                            lava_vecinity_hash.insert(CardinalDirections::NE);
                        }
                        (-1, 0) => {
                            lava_vecinity_hash.insert(CardinalDirections::W);
                        }
                        (1, 0) => {
                            lava_vecinity_hash.insert(CardinalDirections::E);
                        }
                        (-1, -1) => {
                            lava_vecinity_hash.insert(CardinalDirections::SW);
                        }
                        (0, -1) => {
                            lava_vecinity_hash.insert(CardinalDirections::S);
                        }
                        (1, -1) => {
                            lava_vecinity_hash.insert(CardinalDirections::SE);
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
//
pub fn update_map_at_position(
    mut atlas_query: Query<&mut TextureAtlas>,
    pos_list: &Vec<Vec2>,
    map: &mut Map,
) {
    for pos in pos_list {
        let (x, y) = (pos.x.round() as i32, pos.y.round() as i32);
        let tile = IVec2::new(x / TILE_SIZE.0, y / TILE_SIZE.1);

        let tile_type = map.tiles[tile_xy_to_map_idx(tile.x, tile.y)];
        //info!("{:?} ", tile_type);

        for y in tile.y - 1..=tile.y + 1 {
            for x in tile.x - 1..=tile.x + 1 {
                let tile_type = map.tiles[tile_xy_to_map_idx(x, y)];
                //info!("{x:?},{y:?} has tile type {:?} ", tile_type);
                if tile_type != TilesType::Lava && map.tile_in_bound(&IVec2::new(x, y)) {
                    let lava_vecinities = get_lava_vecinities(&map, &IVec2::new(x, y));
                    let index = match lava_vecinities {
                        Some(lava_vecinities) => get_index_from_vecinity(&lava_vecinities),
                        None => 11 * 6 + 10,
                    };

                    let id = map.ids[tile_xy_to_map_idx(x, y)];
                    if id.is_some() {
                        if let Ok(mut atlas) = atlas_query.get_mut(id.unwrap()) {
                            atlas.index = index;
                        }
                    }
                }
            }
        }
    }
}

pub fn update_render_map_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    update_tile_query: Query<(Entity, &UpdateTile)>,
    atlas_query: Query<&mut TextureAtlas>,
) {
    let mut tiles_to_be_updated: Vec<Vec2> = Vec::new();

    for (msg_entity, tile_to_update) in update_tile_query.iter() {
        //info!(
        //    "Processing msg: {:?} with tile to update at{:?}",
        //    msg_entity, tile_to_update.position
        //);
        if let Some(tile_type) = map.get_tile_from_position(&tile_to_update.position) {
            if tile_type != tile_to_update.tiletype {
                map.tiles[pos_to_map_idx(tile_to_update.position.x, tile_to_update.position.y)] =
                    tile_to_update.tiletype;
            }
            tiles_to_be_updated.push(tile_to_update.position);
        }
        commands.entity(msg_entity).despawn();
    }
    update_map_at_position(atlas_query, &tiles_to_be_updated, &mut map);
}
