use crate::prelude::*;

pub fn projectile_movement_system(
    mut commands: Commands,
    projectile_query: Query<
        (Entity, &mut Transform, &mut Velocity),
        (With<Projectile>, With<Movable>),
    >,
) {
    for (entity, transform, velocity) in projectile_query.iter() {
        let delta = Vec2::new(
            velocity.x * TIME_STEP * BASE_SPEED,
            velocity.y * TIME_STEP * BASE_SPEED,
        );

        if delta.x != 0. || delta.y != 0. {
            let destination = Vec2::new(
                transform.translation.x + delta.x,
                transform.translation.y + delta.y,
            );
            commands.spawn(WantsToMove {
                entity,
                destination,
            });
        }
    }
}

pub fn despawn_projectile_system(
    mut commands: Commands,
    collide_query: Query<(Entity, &Collide)>,
    projectile_query: Query<&Projectile>,
    enemy_query: Query<(Entity, &Enemy)>,
) {
    for (entity, collide) in collide_query.iter() {

        if projectile_query
            .get(collide.from)
            .is_ok()
        {

            if enemy_query.get(collide.to).is_ok(){
                commands.entity(collide.to).despawn_recursive();

            }
            commands.entity(collide.from).despawn();
            commands.entity(entity).despawn();
        }
    }
}
