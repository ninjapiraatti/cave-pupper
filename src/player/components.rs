use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub id: usize,
    pub input_source: InputSource,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputSource {
    Keyboard(usize), // Index into PLAYER_KEYS
    Gamepad(Entity), // Gamepad entity
}
