use crate::prelude::*;

use super::super::map::NUM_TILES;

const STAGGER_DISTANCE: usize = 1000;
const DESIRED_NUMBER_FLOOR: usize = NUM_TILES as usize / 3;

use super::MapArchitect;

pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self) -> MapBuilder {
        let mut mb = MapBuilder { map: Map::new() };

        let mut rng = thread_rng();
        // start all wall
        mb.fill(TilesType::Lava);
        let center = IVec2::new(MAP_SIZE_IN_TILES.0 / 2, MAP_SIZE_IN_TILES.1 / 2);
        self.drunkard(&center, &mut mb.map);
        while mb
            .map
            .tiles
            .iter()
            .filter(|t| **t == TilesType::Floor)
            .count()
            < DESIRED_NUMBER_FLOOR
        {
            let start = IVec2::new(
                rng.gen_range(0..MAP_SIZE_IN_TILES.0 / 2),
                rng.gen_range(0..MAP_SIZE_IN_TILES.1 / 2),
            );
            self.drunkard(&start, &mut mb.map);
        }

        println!("build finished");

        mb
    }
}

impl DrunkardsWalkArchitect {
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

            if !map.in_bound(&drunkard_position.as_vec2()) {
                break;
            }

            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}
