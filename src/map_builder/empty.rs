use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self) -> MapBuilder {
        let mut mb = MapBuilder { map: Map::new() };

        let center = IVec2::new(2, 2);

        mb.map.tiles[tile_xy_to_map_idx(center.x, center.y)] = TilesType::Lava;
        mb.map.tiles[tile_xy_to_map_idx(center.x + 2, center.y)] = TilesType::Lava;

        mb.map.tiles[tile_xy_to_map_idx(center.x, center.y + 2)] = TilesType::Lava;
        mb.map.tiles[tile_xy_to_map_idx(center.x + 2, center.y + 2)] = TilesType::Lava;
        mb.map.tiles[tile_xy_to_map_idx(center.x , center.y + 1)] = TilesType::Lava;
        mb.map.tiles[tile_xy_to_map_idx(5 , 3)] = TilesType::Lava;
        mb.map.tiles[tile_xy_to_map_idx(1 , 3)] = TilesType::Lava;
        mb.map.tiles[tile_xy_to_map_idx(6 , 1)] = TilesType::Lava;
        mb.map.tiles[tile_xy_to_map_idx(0 , 1)] = TilesType::Lava;
        mb.map.tiles[tile_xy_to_map_idx(0 , 5)] = TilesType::Lava;
        mb.map.tiles[tile_xy_to_map_idx(5 , 5)] = TilesType::Lava;

        mb.map.tiles[tile_xy_to_map_idx(7 , 5)] = TilesType::Lava;
        mb.map.tiles[tile_xy_to_map_idx(6 , 6)] = TilesType::Lava;
        
        mb.map.tiles[tile_xy_to_map_idx(10 , 6)] = TilesType::Lava;
        mb.map.tiles[tile_xy_to_map_idx(11 , 5)] = TilesType::Lava;
        mb
    }
}
