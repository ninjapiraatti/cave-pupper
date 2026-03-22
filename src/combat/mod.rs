mod components;

pub use components::*;

use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, _app: &mut App) {
        // Combat systems will be added later
        // For now, just basic health tracking
    }
}
