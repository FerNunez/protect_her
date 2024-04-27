use crate::prelude::*;

impl Default for EggState {
    fn default() -> Self {
        Self { alive: false }
    }
}

impl EggState {
    //fn dead(&mut self) {
    //    self.alive = false;
    //}
    pub fn spawned(&mut self) {
        self.alive = true;
    }
}

pub fn egg_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut egg_state: ResMut<EggState>,
) {
    let mut rng = thread_rng();
    let pull_force = Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..1.0));
    let eye = commands
        .spawn(SpriteBundle {
            texture: game_textures.egg_eye.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.03),

            ..Default::default()
        })
        .insert(PullForce(Vec2::new(pull_force.x * 10., pull_force.y * 10.)))
        .insert(Embrion)
        .id();
    let yellow = commands
        .spawn(SpriteBundle {
            texture: game_textures.egg_yellow.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.01),

            ..Default::default()
        })
        .insert(RotationSpeed(-0.2))
        .id();
    //let blue = commands
    //    .spawn(SpriteBundle {
    //        texture: game_textures.egg_blue.clone(),
    //        transform: Transform::from_xyz(0.0, 0.0, -0.01),

    //        ..Default::default()
    //    })
    //    .id();

    //tentacles
    let tentacles = commands
        .spawn(SpriteBundle {
            texture: game_textures.egg_tentacles.clone(),
            transform: Transform::from_xyz(
                (MAP_SIZE_IN_TILES.0 * TILE_SIZE.0 / 2) as f32,
                (MAP_SIZE_IN_TILES.1 * TILE_SIZE.1 / 2) as f32,
                1.01,
            ),

            ..Default::default()
        })
        .insert(Egg)
        .insert(Movable)
        .insert(SpriteSize::from((128., 128.)))
        .insert(Velocity { x: 0., y: 0. })
        .insert(Target(Vec2::new(0.0, 0.0)))
        .id();

    commands.entity(tentacles).push_children(&[eye, yellow]);
    egg_state.spawned();
}

pub fn pull_force_system(
    time: Res<Time>,
    mut pull_force_query: Query<(&mut PullForce, &mut Transform)>,
) {
    for (mut pull_force, mut transform) in pull_force_query.iter_mut() {
        let distant_from_center = 10.;

        let has_to_inverse: (bool, bool) =
            if transform.translation.xy().length() > distant_from_center {
                if transform.translation.x * pull_force.0.x < 0.
                    && transform.translation.y * pull_force.0.y < 0.
                {
                    (false, false)
                } else if transform.translation.x.abs() > transform.translation.y.abs()
                    && transform.translation.x * pull_force.0.x > 0.
                {
                    (true, false)
                } else if transform.translation.y.abs() > transform.translation.x.abs()
                    && transform.translation.y * pull_force.0.y > 0.
                {
                    (false, true)
                } else {
                    (false, false)
                }
            } else {
                (false, false)
            };

        //info!("psotion?: ({:?})", transform.translation.xy());

        //info!("has to invert?: ({:?})", has_to_inverse);
        //info!("pull force?: ({:?})", pull_force.0);
        if has_to_inverse.0 || has_to_inverse.1 {
            let mut rng = thread_rng();

            pull_force.0 = Vec2::new(
                rng.gen_range(0.1..=10.0) * pull_force.0.x.signum(),
                rng.gen_range(0.1..=10.0) * pull_force.0.y.signum(),
            );
        }
        if has_to_inverse.0 {
            pull_force.0.x = -pull_force.0.x;
        }
        if has_to_inverse.1 {
            pull_force.0.y = -pull_force.0.y;
        }
        transform.translation.x += time.delta().as_secs_f32() * pull_force.0.x;
        transform.translation.y += time.delta().as_secs_f32() * pull_force.0.y;
    }
}

pub fn rotation_egg_parts(
    time: Res<Time>,
    mut egg_part_query: Query<(&RotationSpeed, &mut Transform)>,
) {
    for (rotation_speed, mut transformation) in egg_part_query.iter_mut() {
        transformation.rotate_z(rotation_speed.0 * time.delta_seconds());
    }
}

pub fn attraction_velocity_egg(
    time: Res<Time>,
    mut egg_part_query: Query<(&RotationSpeed, &mut Transform)>,
) {
    for (rotation_speed, mut transformation) in egg_part_query.iter_mut() {
        transformation.rotate_z(rotation_speed.0 * time.delta_seconds());
    }
}

pub fn egg_target_move(
    time: Res<Time>,
    mut commands: Commands,
    mut egg_query: Query<(Entity, &Target, &mut Velocity, &mut Transform), Without<Player>>,
) {
    for (egg_entity, target, mut egg_velocity, egg_transform) in egg_query.iter_mut() {
        //let (destination, angle) = egg.bicycle_model(egg_transform, player_transform.translation.xy());
        let direction_vector = egg_transform.translation.xy() - target.0;
        let angle = egg_transform.rotation.to_euler(EulerRot::XYZ).2 + 0.1 * PI / 180.0;

        egg_velocity.x = direction_vector.normalize_or_zero().x * EGG_SPEED;
        egg_velocity.y = -direction_vector.normalize_or_zero().y * EGG_SPEED;
        let delta_time = time.delta().as_secs_f32();

        let delta = Vec2::new(egg_velocity.x * delta_time, egg_velocity.y * delta_time);

        if delta.x != 0. || delta.y != 0. {
            let destination = Vec2::new(
                egg_transform.translation.x + delta.x,
                egg_transform.translation.y + delta.y,
            );

            commands.spawn(WantsToMove {
                entity: egg_entity,
                destination,
            });
        }

        //egg_transform.rotation = Quat::from_rotation_z(angle );
        commands.spawn(WantsToRotate {
            entity: egg_entity,
            angle,
        });
    }
}
