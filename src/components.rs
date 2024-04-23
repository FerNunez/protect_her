use bevy::transform::components;

use crate::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy {
    pub life_duration: Duration,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            life_duration: Duration::ZERO,
        }
    }

    pub fn bicycle_model(&self, origin: &Transform, destination: Vec2) -> (Vec2, f32) {
        let direction_vector = Vec2::new(
            destination.x - origin.translation.x,
            -(destination.y - origin.translation.y),
        );
        //let *self.life_duration.as_secs_f32()/10.;
        let angle = direction_vector.angle_between(Vec2 { x: 0.0, y: -1.0 }) + PI / 2.;

        (Vec2::new(0., 0.), 0.0)
    }

    pub fn perfect_model(&self, origin: &Transform, destination: &Vec2) -> (Vec2, f32) {
        let direction_vector = Vec2::new(
            destination.x - origin.translation.x,
            -(destination.y - origin.translation.y),
        );

        //let angle = Vec2::new(1.0, 0.0).angle_between(direction_vector);
        // NOTE: THIS PI IS CAUSE OF THE sperm was inverted :(
        let angle = direction_vector.angle_between(Vec2::new(1.0, 0.)) + PI;

        (direction_vector, angle)
    }
}

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
pub struct FacingDirection(pub Vec2);

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
pub struct WantsToMoveInDirecion {
    pub entity: Entity,
}

#[derive(Component)]
pub struct CanWallRide;

#[derive(Component)]
pub struct Collide {
    pub from: Entity,
    pub to: Entity,
    pub pos: Vec2,
}

#[derive(Component)]
pub struct HasCollided;

#[derive(Component)]
pub struct CanFly;

impl Animation {
    pub fn new(first_index: usize, last_index: usize, timer: Timer) -> Self {
        Self {
            first_index,
            last_index,
            timer,
        }
    }
}
impl Default for Animation {
    fn default() -> Self {
        Self::new(0, 0, Timer::from_seconds(100.0, TimerMode::Once))
    }
}

#[derive(Component)]
pub struct InEdit;

#[derive(Component)]
pub struct UpdateTile {
    pub from_entity: Entity,
    pub position: Vec2,
    pub tiletype: TilesType,
}

#[derive(Component)]
pub struct Animation {
    pub first_index: usize,
    pub last_index: usize,
    pub timer: Timer,
}
#[derive(Component)]
pub enum PlayerAnimation {
    MovingDown(Animation),
    MovingUp(Animation),
    MovingSide(Animation),
    Idle(Animation),
}
