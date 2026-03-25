mod actions;
mod animation;
mod roster;

pub use actions::{Action, Character, Grounded, SpriteConfig, execute_actions};
pub use animation::{AnimationPlayer, FacingDirection};
pub use roster::*;

use bevy::prelude::*;

use crate::state::GameState;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                actions::execute_actions,
                animation::update_animation_state,
                animation::animate_sprites,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}
