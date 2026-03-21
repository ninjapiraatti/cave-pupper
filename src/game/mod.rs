mod systems;

use bevy::prelude::*;

use crate::state::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), systems::setup_game)
            .add_systems(OnExit(GameState::Playing), systems::cleanup_game)
            .add_systems(
                Update,
                systems::back_to_menu.run_if(in_state(GameState::Playing)),
            );
    }
}
