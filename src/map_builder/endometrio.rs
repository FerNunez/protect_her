use crate::prelude::*;

use super::super::map::NUM_TILES;

const STAGGER_DISTANCE: usize = 1000;
const DESIRED_NUMBER_FLOOR_PER_ISLAND: usize = NUM_TILES as usize / (3 * NUM_ISLAND as usize);
const NUM_ISLAND: i32 = 50;
use super::MapArchitect;

pub struct EndometrioArchitect {
    pub num_islands: usize,
}

impl MapArchitect for EndometrioArchitect {
    fn new(&mut self) -> MapBuilder {
        let mut mb = MapBuilder { map: Map::new() };
        //
        mb.fill(TilesType::Lava);
        mb.circle(
            IVec2::new(50, MAP_SIZE_IN_TILES.1 - 50) ,
            50,
            TilesType::Floor,
        );
        mb.circle(
            IVec2::new(MAP_SIZE_IN_TILES.0 - 50, MAP_SIZE_IN_TILES.1 - 50),
            50,
            TilesType::Floor,
        );
        mb.circle(IVec2::new(MAP_SIZE_IN_TILES.0 / 2, 1), 20, TilesType::Floor);
        let num_final_islands = NUM_ISLAND + 3;
        let division = (
            MAP_SIZE_IN_TILES.0 / num_final_islands,
            MAP_SIZE_IN_TILES.1 / num_final_islands,
        );
        let mut rng = thread_rng();
        let island_position: Vec<IVec2> = (0..num_final_islands)
            .map(|p| division.0.max(division.1) * p)
            .map(|i| {
                //info!("converted: {:?}", i);
                let range = if i < num_final_islands / 2 {
                    0..MAP_SIZE_IN_TILES.1 / 2 - 1
                } else {
                    MAP_SIZE_IN_TILES.1 / 2..MAP_SIZE_IN_TILES.1 - 1
                };
                let j = rng.gen_range(range);
                IVec2::new(i, j)
            })
            .collect();
        // start all wall
        island_position.iter().for_each(|center| {
            self.drunkard(&center, &mut mb.map);
            while mb
                .map
                .tiles
                .iter()
                .filter(|t| **t == TilesType::Floor)
                .count()
                < DESIRED_NUMBER_FLOOR_PER_ISLAND
            {
                let start = IVec2::new(
                    rng.gen_range(0..MAP_SIZE_IN_TILES.0 / 2),
                    rng.gen_range(0..MAP_SIZE_IN_TILES.1 / 2),
                );

                info!("start: {:?}", start);
                self.drunkard(&start, &mut mb.map);
            }
        });
        println!("build finished");

        mb
    }
}

impl EndometrioArchitect {
    fn drunkard(&mut self, start: &IVec2, map: &mut Map) {
        let mut drunkard_position = start.clone();
        let mut distance_staggered = 0;
        let mut rng = thread_rng();

        loop {
            let drunk_idx = tile_xy_to_map_idx(drunkard_position.x, drunkard_position.y);

            map.tiles[drunk_idx] = TilesType::Floor;

            match rng.gen_range(0..4) {
                0 => drunkard_position.x += 1,
                1 => drunkard_position.x -= 1,
                2 => drunkard_position.y += 1,
                _ => drunkard_position.y -= 1,
            }

            if !map.tile_in_bound(&drunkard_position) {
                break;
            }

            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}
