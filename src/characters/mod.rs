mod actions;

pub use actions::*;

use bevy::prelude::*;

use crate::state::GameState;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            actions::execute_actions.run_if(in_state(GameState::Playing)),
        );
    }
}
