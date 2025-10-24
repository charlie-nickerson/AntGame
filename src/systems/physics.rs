use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;

pub fn apply_gravity(mut query: Query<&mut Velocity, With<Physics>>, time: Res<Time>) {
    for mut velocity in &mut query {
        velocity.0.y += GRAVITY * time.delta_secs();
        velocity.0.y = velocity.0.y.max(MAX_FALL_SPEED);
    }
}

pub fn handle_collision(
    mut player_query: Query<(&mut Velocity, &mut Transform, &Sprite, &mut Grounded), With<Physics>>,
    obstacle_query: Query<(&Sprite, &Transform, &CollisionType), Without<Physics>>,
    images: Res<Assets<Image>>,
    layouts: Res<Assets<TextureAtlasLayout>>,
) {
    for (mut velocity, mut player_transform, player_sprite, mut grounded) in &mut player_query {
        grounded.0 = false;

        let Some(player_size) = get_sprite_size(player_sprite, &images, &layouts) else {
            continue;
        };

        let player_half_width = player_size.x / 2.0;
        let player_half_height = player_size.y / 2.0;

        for (obstacle_sprite, obstacle_transform, collision_type) in &obstacle_query {
            let Some(obstacle_size) = get_sprite_size(obstacle_sprite, &images, &layouts) else {
                continue;
            };

            let obstacle_half_width = obstacle_size.x / 2.0;
            let obstacle_half_height = obstacle_size.y / 2.0;

            // Calculate positions
            let player_pos = player_transform.translation;
            let obstacle_pos = obstacle_transform.translation;

            // Calculate overlap on each axis
            let Some((overlap_x, overlap_y)) = check_overlap(
                obstacle_half_width,
                player_half_width,
                obstacle_half_height,
                player_half_height,
                player_pos,
                obstacle_pos,
            ) else {
                continue;
            };

            // Resolve collision on the axis with SMALLEST penetration
            if overlap_x < overlap_y {
                // Horizontal collision (left or right side)

                // Check if this collision type allows side collision
                if matches!(collision_type, CollisionType::Wall | CollisionType::Solid) {
                    if player_pos.x < obstacle_pos.x {
                        // Colliding from the left
                        player_transform.translation.x -= overlap_x;
                    } else {
                        // Colliding from the right
                        player_transform.translation.x += overlap_x;
                    }
                    velocity.0.x = 0.0;
                }
            } else {
                // Vertical collision (top or bottom)
                if player_pos.y < obstacle_pos.y {
                    // Colliding from below (hitting ceiling)
                    if matches!(
                        collision_type,
                        CollisionType::Ceiling | CollisionType::Solid
                    ) {
                        player_transform.translation.y -= overlap_y;
                        velocity.0.y = 0.0;
                    }
                } else {
                    // Colliding from above (landing on ground)
                    if matches!(collision_type, CollisionType::Ground | CollisionType::Solid) {
                        player_transform.translation.y += overlap_y;
                        velocity.0.y = 0.0;
                        grounded.0 = true;
                    }
                }
            }
        }
    }
}

pub fn apply_velocity(
    mut query: Query<(&Velocity, &mut Transform), With<Physics>>,
    time: Res<Time>,
) {
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}

fn check_overlap(
    half_width_a: f32,
    half_width_b: f32,
    half_height_a: f32,
    half_height_b: f32,
    pos_a: Vec3,
    pos_b: Vec3,
) -> Option<(f32, f32)> {
    let overlap_x = (half_width_a + half_width_b) - (pos_a.x - pos_b.x).abs();
    let overlap_y = (half_height_a + half_height_b) - (pos_a.y - pos_b.y).abs();

    if overlap_x > 0.0 && overlap_y > 0.0 {
        Some((overlap_x, overlap_y))
    } else {
        None
    }
}

fn get_sprite_size(
    sprite: &Sprite,
    images: &Assets<Image>,
    layouts: &Assets<TextureAtlasLayout>,
) -> Option<Vec2> {
    if let Some(custom) = sprite.custom_size {
        return Some(custom);
    }

    if let Some(atlas) = &sprite.texture_atlas {
        if let Some(layout) = layouts.get(&atlas.layout) {
            let rect = layout.textures[atlas.index];
            return Some(rect.size().as_vec2());
        }
    }

    images.get(&sprite.image).map(|img| img.size().as_vec2())
}
