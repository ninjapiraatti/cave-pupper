use bevy::prelude::*;

use crate::input::MAX_SLOTS;

#[derive(Component)]
pub struct Player {
    pub slot: usize,
}

/// State of a player slot
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SlotState {
    #[default]
    Empty,
    WaitingToSpawn,
    /// Previewing character before spawn (character_index, remaining_time)
    Previewing(usize, f32),
    Alive(Entity),
    Dead,
}

/// Tracks state of all player slots
#[derive(Resource)]
pub struct PlayerSlots {
    pub states: [SlotState; MAX_SLOTS],
    /// Player names (placeholders for now)
    pub names: [String; MAX_SLOTS],
}

impl Default for PlayerSlots {
    fn default() -> Self {
        Self {
            states: [SlotState::Empty; MAX_SLOTS],
            names: std::array::from_fn(|i| format!("Player {}", i + 1)),
        }
    }
}

impl PlayerSlots {
    pub fn reset(&mut self) {
        self.states = [SlotState::Empty; MAX_SLOTS];
    }

    pub fn reset_names(&mut self) {
        self.names = std::array::from_fn(|i| format!("Player {}", i + 1));
    }

    pub fn get(&self, slot: usize) -> SlotState {
        self.states[slot]
    }

    pub fn set(&mut self, slot: usize, state: SlotState) {
        self.states[slot] = state;
    }

    pub fn active_count(&self) -> usize {
        self.states
            .iter()
            .filter(|s| !matches!(s, SlotState::Empty))
            .count()
    }
}
