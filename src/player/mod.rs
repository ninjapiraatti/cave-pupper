mod components;
mod systems;

pub use components::{Player, PlayerSlots, SlotState};

use bevy::prelude::*;

use crate::state::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSlots>()
            .add_systems(OnEnter(GameState::Playing), systems::reset_slots)
            .add_systems(
                Update,
                (
                    systems::handle_join_respawn,
                    systems::update_previews,
                    systems::update_grounded,
                    systems::handle_wall_contacts,
                    systems::apply_friction,
                    systems::check_death_zone,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), systems::cleanup_players);
    }
}
