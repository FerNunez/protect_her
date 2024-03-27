use crate::prelude::*;

const NUM_TILES: i32 = (MAP_SIZE.0 * MAP_SIZE.1) / (TILE_SIZE.0 * TILE_SIZE.1);

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

    pub fn in_bound(&self, pos: &IVec2) -> bool {
        pos.x >= 0 && pos.x <= MAP_SIZE.0 && pos.y >= 0 && pos.y <= MAP_SIZE.1
    }

    pub fn can_enter_tile(&self, pos: &Vec2) -> bool {
        self.in_bound(&pos.as_ivec2())
            && self.tiles[pos_to_map_idx(pos.x, pos.y)] == TilesType::Floor
    }
}

pub fn pos_to_map_idx(x: f32, y: f32) -> usize {
    ((y as i32 * MAP_SIZE.0) + x as i32) as usize
}

#[test]
fn not_in_bound() {
    let map = Map::new();
    let pos_right = IVec2::new(MAP_SIZE.0 + 1, MAP_SIZE.1);
    let pos_left = IVec2::new(-1, MAP_SIZE.1);
    let pos_up = IVec2::new(MAP_SIZE.0, -1);
    let pos_bottom = IVec2::new(MAP_SIZE.0, MAP_SIZE.1 + 1);
    assert!(!map.in_bound(&pos_right));
    assert!(!map.in_bound(&pos_left));
    assert!(!map.in_bound(&pos_bottom));
    assert!(!map.in_bound(&pos_up));
}

#[test]
fn is_in_bound() {
    let map = Map::new();
    let pos_inside = IVec2::new(MAP_SIZE.0 / 2, MAP_SIZE.1 / 2);
    assert!(map.in_bound(&pos_inside));
}

#[test]
fn test_map_index() {
    assert_eq!(pos_to_map_idx(0.2, 0.0), 0);
    assert_eq!(pos_to_map_idx(1.2, 0.0), 1);
    assert_eq!(pos_to_map_idx(0.9, 1.0) as i32, MAP_SIZE.0);
    assert_eq!(pos_to_map_idx(1.9, 1.0) as i32, MAP_SIZE.0 + 1);
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
