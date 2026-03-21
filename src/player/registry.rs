use bevy::prelude::*;

use super::components::InputSource;

#[derive(Resource, Default)]
pub struct PlayerRegistry {
    pub players: Vec<(Entity, InputSource)>,
}

impl PlayerRegistry {
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    pub fn is_input_taken(&self, source: &InputSource) -> bool {
        self.players.iter().any(|(_, s)| s == source)
    }

    pub fn add_player(&mut self, entity: Entity, source: InputSource) {
        self.players.push((entity, source));
    }
}
