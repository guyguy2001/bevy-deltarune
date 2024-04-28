use bevy::prelude::*;

pub fn get_input_direction(input: &ButtonInput<KeyCode>) -> Vec3 {
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        direction += Vec3::Y;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        direction += -Vec3::Y;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        direction += Vec3::X;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        direction += -Vec3::X;
    }
    direction.normalize_or_zero()
}
