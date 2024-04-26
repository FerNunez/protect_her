use crate::prelude::*;

pub fn animate_being_hitted(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BeingHitted, &mut Sprite)>,
) {
    for (entity, mut frame_hitted, mut sprite) in query.iter_mut() {
        frame_hitted.0 += 1;
        sprite.color.set_a(0.2);

        if frame_hitted.0 >= FRAMES_HITTED {
            commands.entity(entity).remove::<BeingHitted>();
            sprite.color.set_a(1.);
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut AnimationTimer, &mut Sprite, &mut TextureAtlas)>,
) {
    for ( animation, mut animation_timer, mut sprite, mut atlas) in &mut query {
        animation_timer.timer.tick(time.delta());
        if animation_timer.timer.just_finished() {
            atlas.index = if atlas.index == animation.last_index {
                animation.first_index
            } else {
                atlas.index + 1
            };
        }

        sprite.flip_x = animation.flip;
    }
}
