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

/// Double-tap detection threshold in seconds
pub const DOUBLE_TAP_THRESHOLD: f32 = 0.3;

/// Input state for a single player slot this frame
#[derive(Clone, Copy, Default, Debug)]
pub struct PlayerInput {
    pub key_a_pressed: bool,
    pub key_a_just_pressed: bool,
    pub key_a_double_tapped: bool,
    pub key_b_pressed: bool,
    pub key_b_just_pressed: bool,
    pub key_b_double_tapped: bool,
}

/// Tracks timing for double-tap detection
#[derive(Clone, Copy, Default, Debug)]
pub struct DoubleTapState {
    pub last_press_a: f32,
    pub last_press_b: f32,
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
    double_tap_states: [DoubleTapState; MAX_SLOTS],
}

impl PlayerInputs {
    pub fn get(&self, slot: usize) -> &PlayerInput {
        &self.inputs[slot]
    }

    pub fn update(&mut self, keyboard: &ButtonInput<KeyCode>, bindings: &InputBindings, time: f32) {
        for (slot, input) in self.inputs.iter_mut().enumerate() {
            let binding = bindings.get(slot);
            let tap_state = &mut self.double_tap_states[slot];

            input.key_a_pressed = keyboard.pressed(binding.key_a);
            input.key_a_just_pressed = keyboard.just_pressed(binding.key_a);
            input.key_b_pressed = keyboard.pressed(binding.key_b);
            input.key_b_just_pressed = keyboard.just_pressed(binding.key_b);

            // Double-tap detection for key A
            input.key_a_double_tapped = false;
            if input.key_a_just_pressed {
                if time - tap_state.last_press_a <= DOUBLE_TAP_THRESHOLD {
                    input.key_a_double_tapped = true;
                    tap_state.last_press_a = 0.0; // Reset to prevent triple-tap
                } else {
                    tap_state.last_press_a = time;
                }
            }

            // Double-tap detection for key B
            input.key_b_double_tapped = false;
            if input.key_b_just_pressed {
                if time - tap_state.last_press_b <= DOUBLE_TAP_THRESHOLD {
                    input.key_b_double_tapped = true;
                    tap_state.last_press_b = 0.0; // Reset to prevent triple-tap
                } else {
                    tap_state.last_press_b = time;
                }
            }
        }
    }
}
