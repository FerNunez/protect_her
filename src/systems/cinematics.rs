use crate::prelude::*;

pub fn movable_system(
    mut query_movable: Query<(Entity, &Velocity, &mut Transform), With<Movable>>,
    query_player: Query<&Player>,
) {
    for (entity, velocity, mut transform) in query_movable.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if query_player.get_component::<Player>(entity).is_ok() {
            transform.rotate_z(0.05 + 0.1 * (velocity.x.abs() + velocity.y.abs()) / 2.);
        }
    }
}
