
use crate::prelude::*;

pub fn spawn_coin_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &SpawnCoin)>,
) {
    for (entity, spawn_coin) in query.iter() {
        let x = spawn_coin.0.x;
        let y = spawn_coin.0.y;

        let sprite_size = (COIN_SCALE * COIN_SIZE.0, COIN_SCALE * COIN_SIZE.1);
        commands
            .spawn(SpriteBundle {
                texture: game_textures.coin.clone(),
                transform: Transform::from_xyz(x, y, 0.0)
                    .with_scale(Vec3::new(COIN_SCALE, COIN_SCALE, 0.)),
                ..Default::default()
            })
            .insert(SpriteSize::from(sprite_size))
            // TODO: fix this. create muiltiplication (f32, f"2)*f32
            .insert(Coin);

        commands.entity(entity).despawn();
    }
}
