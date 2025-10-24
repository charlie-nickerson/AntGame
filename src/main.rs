mod components;
mod constants;
mod resources;
mod systems;

use bevy::prelude::*;
use components::*;
use constants::*;
use resources::*;
use systems::animation::*;
use systems::physics::*;
use systems::player::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Ant Game".to_string(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_input,
                apply_gravity,
                update_animation_state,
                switch_sprite_sheet,
                apply_velocity,
                handle_collision,
                animate_sprite,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Camera
    commands.spawn(Camera2d);

    // Spawn Background
    commands.spawn((
        Sprite::from_image(asset_server.load("background.png")),
        Transform::from_xyz(0., 0., -10.), // Have it behind the foreground
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("gun.png")),
        Transform::from_xyz(0., 0., 0.),
        Velocity(Vec2::ZERO),
        Grounded(false),
        Physics,
    ));

    // Enemy placeholder
    commands.spawn((
        Sprite::from_image(asset_server.load("Sprite-0001.png")),
        Transform::from_xyz(100., 0., 0.),
    ));

    // Create animation layouts
    let idle_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 128), 4, 1, None, None);
    let walking_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 128), 4, 1, None, None);

    // Store animation resources
    commands.insert_resource(PlayerAnimations {
        idle_texture: asset_server.load("idle_ant.png"),
        idle_layout: texture_atlas_layouts.add(idle_layout),
        walking_texture: asset_server.load("walking_ant.png"),
        walking_layout: texture_atlas_layouts.add(walking_layout),
    });

    // Spawn player
    commands.spawn((
        Sprite::from_atlas_image(
            asset_server.load("idle_ant.png"),
            TextureAtlas {
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::new(64, 128),
                    4,
                    1,
                    None,
                    None,
                )),
                index: 0,
            },
        ),
        Transform::from_xyz(0., 0., 0.),
        AnimationIndices { first: 0, last: 3 },
        AnimationTimer(Timer::from_seconds(
            ANIMATION_FRAME_TIME,
            TimerMode::Repeating,
        )),
        AnimationState::Idle,
        Velocity(Vec2::ZERO),
        Grounded(false),
        Player,
        Physics,
    ));

    // Ground
    commands.spawn((
        Sprite::from_image(asset_server.load("ground.png")),
        CollisionType::Solid,
        Transform::from_xyz(0., -100., 0.),
    ));
}
