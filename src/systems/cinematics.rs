use crate::prelude::*;

pub fn movable_system(mut query: Query<(&Velocity, &mut Transform), With<Movable>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
