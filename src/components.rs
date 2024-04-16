use bevy::{ecs::entity::Entity, math::Vec2, prelude::Component, time::Timer};

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
pub struct Projectile;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
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

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component)]
pub struct SpawnSkill(pub Vec2);

#[derive(Component)]
pub struct UI;

#[derive(Component)]
pub struct WithReloadtime(pub f32);

#[derive(Component)]
pub struct CoinText;

#[derive(Component)]
pub struct CanDash;

#[derive(Component)]
pub struct Dash {
    //pub start_time: Option<Time>,
    pub timer: Timer,
    //pub duration: u32,
    pub velocity_offset: f32,
}

#[derive(Component)]
pub struct WantsToAccelerate(pub f32);

#[derive(Component)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Vec2,
}

#[derive(Component)]
pub struct WantsToRotate {
    pub entity: Entity,
    pub angle: f32,
}

#[derive(Component)]
pub struct AskingToMove;

#[derive(Component)]
pub struct AskingToRotate;

#[derive(Component)]
pub struct WantsToMoveInDirecion{
    pub entity: Entity,
}

#[derive(Component)]
pub struct CanWallRide;


#[derive(Component)]
pub struct Collide{
    pub from: Entity,
    pub to: Entity,
    pub pos: Vec2,
}

#[derive(Component)]
pub struct HasCollided;

#[derive(Component)]
pub struct CanFly;
