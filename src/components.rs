use bevy::{math::Vec2, prelude::Component};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

#[derive(Component)]
pub struct SpriteScale(pub Vec2);
impl From<(f32, f32)> for SpriteScale {
    fn from(val: (f32, f32)) -> Self {
        SpriteScale(Vec2::new(val.0, val.1))
    }
}

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct BeingHitted(pub u16);

#[derive(Component)]
pub struct SpawnCoin(pub Vec2);

#[derive(Component)]
pub struct Coin;
