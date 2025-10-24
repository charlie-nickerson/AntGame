use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn update_animation_state(
    mut query: Query<(&Velocity, &mut AnimationState, &mut AnimationIndices), With<Player>>,
) {
    for (velocity, mut state, mut indices) in &mut query {
        let new_state = if velocity.0.x.abs() > 0.1 {
            AnimationState::Walking
        } else {
            AnimationState::Idle
        };

        if *state != new_state {
            *state = new_state;

            match *state {
                AnimationState::Idle => {
                    indices.first = 0;
                    indices.last = 3;
                }
                AnimationState::Walking => {
                    indices.first = 0;
                    indices.last = 3;
                }
            }
        }
    }
}

pub fn switch_sprite_sheet(
    mut query: Query<(&AnimationState, &mut Sprite), (Changed<AnimationState>, With<Player>)>,
    animations: Res<PlayerAnimations>,
) {
    for (state, mut sprite) in &mut query {
        match state {
            AnimationState::Idle => {
                sprite.image = animations.idle_texture.clone();
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.layout = animations.idle_layout.clone();
                    atlas.index = 0;
                }
            }
            AnimationState::Walking => {
                sprite.image = animations.walking_texture.clone();
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.layout = animations.walking_layout.clone();
                    atlas.index = 0;
                }
            }
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index >= indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
