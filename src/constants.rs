use bevy::prelude::*;

/// Maximum number of players supported
pub const MAX_PLAYERS: usize = 4;

/// Gamepad stick deadzone (ignore values below this threshold)
pub const STICK_DEADZONE: f32 = 0.15;

/// Player colors for each player slot
pub const PLAYER_COLORS: [Color; MAX_PLAYERS] = [
    Color::srgb(0.2, 0.6, 1.0),  // Blue - Player 1
    Color::srgb(1.0, 0.3, 0.3),  // Red - Player 2
    Color::srgb(0.3, 1.0, 0.3),  // Green - Player 3
    Color::srgb(1.0, 1.0, 0.3),  // Yellow - Player 4
];

/// Keyboard bindings for each player (Up, Down, Left, Right, Action)
pub const PLAYER_KEYS: [(KeyCode, KeyCode, KeyCode, KeyCode, KeyCode); MAX_PLAYERS] = [
    // Player 1: WASD + Space
    (KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD, KeyCode::Space),
    // Player 2: Arrow keys + Enter
    (KeyCode::ArrowUp, KeyCode::ArrowDown, KeyCode::ArrowLeft, KeyCode::ArrowRight, KeyCode::Enter),
    // Player 3: IJKL + U
    (KeyCode::KeyI, KeyCode::KeyK, KeyCode::KeyJ, KeyCode::KeyL, KeyCode::KeyU),
    // Player 4: Numpad 8456 + 0
    (KeyCode::Numpad8, KeyCode::Numpad5, KeyCode::Numpad4, KeyCode::Numpad6, KeyCode::Numpad0),
];
