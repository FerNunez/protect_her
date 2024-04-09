use crate::prelude::*;

pub const NUM_TILES: i32 = MAP_SIZE_IN_TILES.0 * MAP_SIZE_IN_TILES.1;

#[derive(Copy, Clone, PartialEq)]
pub enum TilesType {
    Wall,
    Floor,
}

#[derive(Resource)]
pub struct Map {
    pub tiles: Vec<TilesType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TilesType::Floor; NUM_TILES as usize],
        }
    }

    pub fn in_bound(&self, pos: &Vec2) -> bool {
        pos.x >= 0.0
            && pos.x <= (MAP_SIZE_IN_TILES.0 * TILE_SIZE.0) as f32
            && pos.y >= 0.0
            && pos.y <= (MAP_SIZE_IN_TILES.1 * TILE_SIZE.1) as f32
    }

    pub fn can_enter_tile(&self, pos: &Vec2) -> bool {
        self.in_bound(&pos) && self.tiles[pos_to_map_idx(pos.x, pos.y)] == TilesType::Floor
    }

    pub fn tile_can_enter_tile(&self, pos: &Vec2) -> bool {

       // let gap = 26./2.;
       // self.in_bound(&pos)
       //     && self.tiles[pos_to_map_idx(pos.x - gap, pos.y - gap)] == TilesType::Floor
       //     && self.tiles[pos_to_map_idx(pos.x - gap, pos.y + gap)] == TilesType::Floor
       //     && self.tiles[pos_to_map_idx(pos.x + gap, pos.y - gap)] == TilesType::Floor
       //     && self.tiles[pos_to_map_idx(pos.x + gap, pos.y + gap)] == TilesType::Floor

            // declaed to right
        self.in_bound(&pos)
            && self.tiles[pos_to_map_idx(pos.x, pos.y)] == TilesType::Floor
            && self.tiles[pos_to_map_idx(pos.x + 26., pos.y)] == TilesType::Floor
            && self.tiles[pos_to_map_idx(pos.x, pos.y + 26.)] == TilesType::Floor
            && self.tiles[pos_to_map_idx(pos.x + 26., pos.y + 26.)] == TilesType::Floor
    }
}
pub fn pos_to_map_idx(x: f32, y: f32) -> usize {
    let (x, y) = (x.round() as i32, y.round() as i32);
    tile_xy_to_map_idx(x / TILE_SIZE.0, y / TILE_SIZE.1)
}

pub fn tile_xy_to_map_idx(map_x: i32, map_y: i32) -> usize {
    //info!("a tile: {map_x},{map_y}");
    ((map_y * MAP_SIZE_IN_TILES.0) + map_x) as usize
}

#[test]
fn not_in_bound() {
    let map = Map::new();

    let pos_right = Vec2::new((MAP_SIZE_IN_TILES.0 * TILE_SIZE.0 + 1) as f32, 0.);
    assert!(!map.in_bound(&pos_right));

    let pos_left = Vec2::new(-1., 0.);
    assert!(!map.in_bound(&pos_left));

    let pos_up = Vec2::new(0., -1.);
    assert!(!map.in_bound(&pos_up));

    let pos_bottom = Vec2::new(0., (MAP_SIZE_IN_TILES.1 * TILE_SIZE.1) as f32);
    assert!(!map.in_bound(&pos_bottom));
}

#[test]
fn int_divide_int() {
    let num: i32 = 32;
    let dem: i32 = 33;
    assert_eq!(num / dem, 0);
    assert_eq!(dem / num, 1);
}

#[test]
fn error() {
    let x = 128;
    let y = 0;

    let idx = pos_to_map_idx(x as f32, y as f32);
    assert_eq!(idx, 128 / 16);
    let x = 0;
    let y = 128;
    let idx = pos_to_map_idx(x as f32, y as f32);
    assert_eq!(idx, 128 / 16)
}

#[test]
fn is_in_bound() {
    let map = Map::new();
    let pos_inside = IVec2::new(MAP_SIZE_IN_TILES.0 / 2, MAP_SIZE_IN_TILES.1 / 2);
    assert!(map.in_bound(&pos_inside));
}

#[test]
fn test_map_index() {
    assert_eq!(pos_to_map_idx(0.2, 0.0), 0);
    assert_eq!(pos_to_map_idx(TILE_SIZE.0 as f32 + 1., 0.0), 1);
}

#[test]
fn test_can_enter_tile() {
    let map = Map::new();
    let pos = Vec2::new(0.0, 0.0);
    assert!(map.can_enter_tile(&pos));
}

#[test]
fn test_cannot_enter_tile() {
    let mut map = Map::new();
    map.tiles[0] = TilesType::Wall;
    let pos = Vec2::new(0.0, 0.0);
    assert!(!map.can_enter_tile(&pos));
}
