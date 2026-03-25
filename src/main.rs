mod arena;
mod characters;
mod combat;
mod game;
mod input;
mod menu;
mod player;
mod state;

use avian2d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cave Pupper - Brawl".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default().with_length_unit(100.0))
        .insert_resource(Gravity(Vec2::NEG_Y * 980.0))
        .init_state::<state::GameState>()
        .add_plugins((
            input::InputPlugin,
            menu::MenuPlugin,
            game::GamePlugin,
            arena::ArenaPlugin,
            player::PlayerPlugin,
            characters::CharacterPlugin,
            combat::CombatPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d::default());

    // Create gradient background (light salmon pink to orange, top to bottom)
    let gradient = create_gradient_texture(
        Color::srgb(1.0, 0.9, 0.85), // Light salmon pink (top)
        Color::srgb(1.0, 0.8, 0.7),  // Orange (bottom)
        256,
    );
    let gradient_handle = images.add(gradient);

    commands.spawn((
        Sprite {
            image: gradient_handle,
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -100.0), // Behind everything
    ));
}

/// Create a vertical gradient texture
fn create_gradient_texture(top: Color, bottom: Color, height: u32) -> Image {
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

    let top_srgba = top.to_srgba();
    let bottom_srgba = bottom.to_srgba();

    let mut data = Vec::with_capacity((height * 4) as usize);

    for y in 0..height {
        let t = y as f32 / (height - 1) as f32;
        let r = (top_srgba.red * (1.0 - t) + bottom_srgba.red * t) * 255.0;
        let g = (top_srgba.green * (1.0 - t) + bottom_srgba.green * t) * 255.0;
        let b = (top_srgba.blue * (1.0 - t) + bottom_srgba.blue * t) * 255.0;
        data.extend_from_slice(&[r as u8, g as u8, b as u8, 255]);
    }

    Image::new(
        Extent3d {
            width: 1,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        default(),
    )
}
