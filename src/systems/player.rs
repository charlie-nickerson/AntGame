use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;

pub fn handle_input(
    mut query: Query<(&mut Velocity, &mut Sprite), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut velocity, mut sprite) in &mut query {
        // Jump
        if input.just_pressed(KeyCode::Space) {
            velocity.0.y = JUMP_FORCE;
        }

        // Horizontal movement
        if input.pressed(KeyCode::KeyA) {
            velocity.0.x = -PLAYER_SPEED;
            sprite.flip_x = true;
        } else if input.pressed(KeyCode::KeyD) {
            velocity.0.x = PLAYER_SPEED;
            sprite.flip_x = false;
        } else {
            velocity.0.x = 0.0;
        }
    }
}
