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
    Alive(Entity),
    Dead,
}

/// Tracks state of all player slots
#[derive(Resource)]
pub struct PlayerSlots {
    pub states: [SlotState; MAX_SLOTS],
}

impl Default for PlayerSlots {
    fn default() -> Self {
        Self {
            states: [SlotState::Empty; MAX_SLOTS],
        }
    }
}

impl PlayerSlots {
    pub fn reset(&mut self) {
        self.states = [SlotState::Empty; MAX_SLOTS];
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
