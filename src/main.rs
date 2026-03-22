mod arena;
mod characters;
mod combat;
mod input;
mod game;
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
