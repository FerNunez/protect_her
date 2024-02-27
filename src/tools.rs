use bevy::math::Vec3;

pub fn collide_circular(a_pos: Vec3, a_radius: f32, b_pos: Vec3, b_radius: f32) -> bool {
    let distance = a_pos.distance(b_pos);

    let minimum = (a_radius + b_radius);

    if distance <= minimum {
        println!("distance: {}", distance);
        println!("minimum: {}", minimum);
        return true;
    }
    return false;
}
