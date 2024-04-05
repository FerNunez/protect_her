use crate::prelude::*;

use self::drunkards::DrunkardsWalkArchitect;
mod drunkards;

trait MapArchitect {
    fn new(&mut self) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: Map,
}

impl MapBuilder {
    pub fn new() -> Self {
        let mut architect = DrunkardsWalkArchitect {};
        architect.new()
    }

    pub fn fill(&mut self, tile_type: TilesType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile_type)
    }
}
