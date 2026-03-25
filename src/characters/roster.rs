use super::actions::{Action, Character, SpriteConfig};
use bevy::prelude::*;

/// All available characters in the game
pub fn all_characters() -> Vec<Character> {
    vec![
        // Basic mover - simple left/right, double-tap to jump
        Character {
            name: "Walker".to_string(),
            action_a: Action::MoveLeft,
            action_b: Action::MoveRight,
            double_tap_action_a: Some(Action::Jump),
            double_tap_action_b: Some(Action::Jump),
            move_speed: 250.0,
            jump_force: 400.0,
            sprite: Some(SpriteConfig::new(
                "characters/character_test01.png",
                UVec2::new(64, 110),
                4,   // columns
                1,   // rows
                4,   // frame count
                0.1, // frame time
            )),
        },
        // Jumper - can jump and move right
        Character {
            name: "Hopper".to_string(),
            action_a: Action::Jump,
            action_b: Action::MoveRight,
            double_tap_action_a: None,
            double_tap_action_b: None,
            move_speed: 200.0,
            jump_force: 500.0,
            sprite: None,
        },
        // Reverse jumper - move left and jump
        Character {
            name: "Lefty".to_string(),
            action_a: Action::MoveLeft,
            action_b: Action::Jump,
            double_tap_action_a: None,
            double_tap_action_b: None,
            move_speed: 200.0,
            jump_force: 500.0,
            sprite: None,
        },
        // Speed demon - fast but can only go right and jump
        Character {
            name: "Speedy".to_string(),
            action_a: Action::Jump,
            action_b: Action::MoveRight,
            double_tap_action_a: None,
            double_tap_action_b: None,
            move_speed: 400.0,
            jump_force: 350.0,
            sprite: None,
        },
        // Slow tank - slow but high jump
        Character {
            name: "Tanky".to_string(),
            action_a: Action::MoveLeft,
            action_b: Action::MoveRight,
            double_tap_action_a: None,
            double_tap_action_b: None,
            move_speed: 150.0,
            jump_force: 600.0,
            sprite: None,
        },
    ]
}

/// Get a character for a given slot (cycles through roster)
pub fn character_for_slot(slot: usize) -> Character {
    let roster = all_characters();
    roster[slot % roster.len()].clone()
}
