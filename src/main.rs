mod constants;
mod game;
mod menu;
mod player;
mod state;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Emo Nemo - Couch Game".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<state::GameState>()
        .add_plugins((menu::MenuPlugin, game::GamePlugin, player::PlayerPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
