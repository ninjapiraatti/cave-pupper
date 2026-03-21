mod components;
mod registry;
mod systems;

pub use components::Player;
pub use registry::PlayerRegistry;

use bevy::prelude::*;

use crate::state::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerRegistry>().add_systems(
            Update,
            (systems::player_join, systems::player_movement).run_if(in_state(GameState::Playing)),
        );
    }
}
