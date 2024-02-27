use crate::prelude::*;

pub fn spawn_skill_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
    spawn_skill_query: Query<(Entity, &SpawnSkill)>,
) {
    if let Ok((entity, _spawn_skill)) = spawn_skill_query.get_single() {
        commands
            .spawn(SpriteBundle {
                texture: game_textures.skill.clone(),
                transform: Transform::from_xyz(
                    (-win_size.w / 2.) + 10.,
                    (win_size.h / 2.) - 10.,
                    0.,
                )
                .with_scale(Vec3::new(SKILL_SCALE, SKILL_SCALE, 1.)),
                ..Default::default()
            })
            .insert(UI)
            .insert(SpriteSize::from(SKILL_SIZE));

        commands.entity(entity).despawn();
    }
}
