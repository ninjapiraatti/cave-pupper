mod actions;
mod roster;

pub use actions::*;
pub use roster::*;

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
