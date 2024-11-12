use crate::prelude::*;

mod drunkards;
mod empty;
mod endometrio;

use self::{drunkards::DrunkardsWalkArchitect, endometrio::EndometrioArchitect};
//use self::empty::EmptyArchitect;
trait MapArchitect {
    fn new(&mut self) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: Map,
}

impl MapBuilder {
    pub fn new() -> Self {
        //let mut architect = DrunkardsWalkArchitect {};
        //       let mut architect = EmptyArchitect {};

        let mut architect = EndometrioArchitect { num_islands: 5 };
        architect.new()
    }

    pub fn fill(&mut self, tile_type: TilesType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile_type)
    }

    pub fn circle(&mut self, center: IVec2, radius: i32, tile_type: TilesType) {
        self.map
            .tiles
            .iter_mut()
            .enumerate()
            .for_each(|(index, t)| {
                let x = index as i32 % MAP_SIZE_IN_TILES.0;
                let y = index as i32 / MAP_SIZE_IN_TILES.0;
                let diff_x = x - center.x;
                let diff_y = y - center.y;

                if diff_x * diff_x + diff_y * diff_y < radius * radius {
                    *t = tile_type;
                }
            });
    }
    pub fn rectangle(&mut self, center: IVec2, width: i32, height: i32, tile_type: TilesType) {
        self.map
            .tiles
            .iter_mut()
            .enumerate()
            .for_each(|(index, t)| {
                let x = index as i32 % MAP_SIZE_IN_TILES.0;
                let y = index as i32 / MAP_SIZE_IN_TILES.0;
                let diff_x = x - center.x;
                let diff_y = y - center.y;
                info!("diff_y: {:?}", diff_y);

                if diff_x - width/2 < 0 && diff_y < height{
                    *t = tile_type;
                }
            });
    }
}
