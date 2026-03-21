mod systems;

use bevy::prelude::*;

use crate::state::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), systems::setup_menu)
            .add_systems(OnExit(GameState::Menu), systems::cleanup_menu)
            .add_systems(Update, systems::menu_input.run_if(in_state(GameState::Menu)));
    }
}
