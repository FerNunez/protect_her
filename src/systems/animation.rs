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

