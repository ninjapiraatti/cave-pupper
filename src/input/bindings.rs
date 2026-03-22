use bevy::prelude::*;

/// Maximum player slots (keyboard only, 2 keys each)
pub const MAX_SLOTS: usize = 8;

/// Two-key binding for a single player slot
#[derive(Clone, Copy, Debug)]
pub struct TwoKeyBinding {
    pub key_a: KeyCode,
    pub key_b: KeyCode,
}

impl TwoKeyBinding {
    pub const fn new(key_a: KeyCode, key_b: KeyCode) -> Self {
        Self { key_a, key_b }
    }
}

/// Input bindings for all player slots
#[derive(Resource, Clone)]
pub struct InputBindings {
    pub slots: [TwoKeyBinding; MAX_SLOTS],
}

impl Default for InputBindings {
    fn default() -> Self {
        Self {
            slots: [
                TwoKeyBinding::new(KeyCode::KeyQ, KeyCode::KeyW),
                TwoKeyBinding::new(KeyCode::KeyE, KeyCode::KeyR),
                TwoKeyBinding::new(KeyCode::KeyT, KeyCode::KeyY),
                TwoKeyBinding::new(KeyCode::KeyU, KeyCode::KeyI),
                TwoKeyBinding::new(KeyCode::KeyO, KeyCode::KeyP),
                TwoKeyBinding::new(KeyCode::KeyA, KeyCode::KeyS),
                TwoKeyBinding::new(KeyCode::KeyD, KeyCode::KeyF),
                TwoKeyBinding::new(KeyCode::KeyG, KeyCode::KeyH),
            ],
        }
    }
}

impl InputBindings {
    pub fn get(&self, slot: usize) -> &TwoKeyBinding {
        &self.slots[slot]
    }
}

/// Input state for a single player slot this frame
#[derive(Clone, Copy, Default, Debug)]
pub struct PlayerInput {
    pub key_a_pressed: bool,
    pub key_a_just_pressed: bool,
    pub key_b_pressed: bool,
    pub key_b_just_pressed: bool,
}

impl PlayerInput {
    /// Returns true if either key was just pressed (for join/respawn detection)
    pub fn any_just_pressed(&self) -> bool {
        self.key_a_just_pressed || self.key_b_just_pressed
    }
}

/// Input states for all player slots
#[derive(Resource, Default)]
pub struct PlayerInputs {
    inputs: [PlayerInput; MAX_SLOTS],
}

impl PlayerInputs {
    pub fn get(&self, slot: usize) -> &PlayerInput {
        &self.inputs[slot]
    }

    pub fn update(&mut self, keyboard: &ButtonInput<KeyCode>, bindings: &InputBindings) {
        for (slot, input) in self.inputs.iter_mut().enumerate() {
            let binding = bindings.get(slot);

            input.key_a_pressed = keyboard.pressed(binding.key_a);
            input.key_a_just_pressed = keyboard.just_pressed(binding.key_a);
            input.key_b_pressed = keyboard.pressed(binding.key_b);
            input.key_b_just_pressed = keyboard.just_pressed(binding.key_b);
        }
    }
}
