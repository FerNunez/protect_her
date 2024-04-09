use crate::prelude::*;

pub fn movable_system(
    mut commands: Commands,
    map: Res<Map>,
    query_wants_to_move: Query<(Entity, &WantsToMove)>,
    mut query_transform: Query<&mut Transform>,
    query_velocity: Query<&Velocity>,
    query_wall_ride: Query<&CanWallRide>,
    query_has_collided: Query<&HasCollided>,
) {
    //println!("calling movable_system");
    for (entity, wants_to_move) in query_wants_to_move.iter() {
        let transform = query_transform.get_mut(wants_to_move.entity);
        let velocity = query_velocity.get(wants_to_move.entity);
        //let _can_wall_ride = query_wall_ride.get_component::<CanWallRide>(wants_to_move.entity);
        let player_entity = wants_to_move.entity;

        if let Ok(mut transform) = transform {
            if map.in_bound(&wants_to_move.destination)
                && map.tile_can_enter_tile(&wants_to_move.destination)
            {
                transform.translation.x = wants_to_move.destination.x;
                transform.translation.y = wants_to_move.destination.y;
            } else {
                let has_collided_already =
                    query_has_collided.get(wants_to_move.entity);

                if has_collided_already.is_err() {
                    commands.spawn(Collide {
                        from: wants_to_move.entity,
                        to: wants_to_move.entity,
                        pos: wants_to_move.destination,
                    });
                    commands.entity(wants_to_move.entity).insert(HasCollided{});
                }

                if let Ok(velocity) = velocity {
                    //info!("there is velocity ");
                    // if can_wall_ride.is_err() {
                    //     continue;
                    // }

                    if velocity.x != 0.
                        && map.tile_can_enter_tile(&Vec2::new(
                            wants_to_move.destination.x,
                            transform.translation.y,
                        ))
                    {
                        transform.translation.x = wants_to_move.destination.x;
                    }
                    if velocity.y != 0.
                        && map.tile_can_enter_tile(&Vec2::new(
                            transform.translation.x,
                            wants_to_move.destination.y,
                        ))
                    {
                        transform.translation.y = wants_to_move.destination.y;
                    }
                }
            }
        }

        //println!(
        //    " deleing wantstomove: {player_entity:?} destination: {},{}",
        //    destination.x, destination.y
        //);
        commands.entity(entity).despawn();
        commands.entity(player_entity).remove::<AskingToMove>();

        // if query_player.get_component::<Player>(entity).is_ok() {
        //
        //transform.rotate_z(0.01 + 0.1 * (velocity.x.abs() + velocity.y.abs()) / 2.);
        // }
    }
    //for (entity, wants_to_rotate) in query_wants_to_rotate.iter() {
    //    let transform = query_transform.get_component_mut::<Transform>(wants_to_rotate.entity);
    //    if let Ok(mut transform) = transform {
    //        transform.rotate_z(wants_to_rotate.angle);
    //    }
    //    commands.entity(entity).despawn();
    //}
}

pub fn rotable_system(
    mut commands: Commands,
    query_wants_to_rotate: Query<(Entity, &WantsToRotate)>,
    mut query_transform: Query<&mut Transform>,
    query_velocity: Query<&Velocity>,
) {
    for (entity, wants_to_rotate) in query_wants_to_rotate.iter() {
        let transform = query_transform.get_mut(wants_to_rotate.entity);
        let velocity = query_velocity.get(wants_to_rotate.entity);

        let player_entity = wants_to_rotate.entity;

        if let Ok(mut transform) = transform {
            let extra = if let Ok(velocity) = velocity {
                0.1 * ((velocity.x.abs() + velocity.y.abs()) / 2.)
            } else {
                0.0
            };

            transform.rotate_z(0.01 + extra);
        }

        //println!(
        //    " deleing wantstomove: {player_entity:?} destination: {},{}",
        //    destination.x, destination.y
        //);
        commands.entity(entity).despawn();
        commands.entity(player_entity).remove::<WantsToRotate>();

        //transform.rotate_z(0.01 + 0.1 * (velocity.x.abs() + velocity.y.abs()) / 2.);
    }
}
