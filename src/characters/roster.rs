use super::actions::{Action, Character};

/// All available characters in the game
pub fn all_characters() -> Vec<Character> {
    vec![
        // Basic mover - simple left/right
        Character {
            name: "Walker".to_string(),
            action_a: Action::MoveLeft,
            action_b: Action::MoveRight,
            move_speed: 250.0,
            jump_force: 400.0,
        },
        // Jumper - can jump and move right
        Character {
            name: "Hopper".to_string(),
            action_a: Action::Jump,
            action_b: Action::MoveRight,
            move_speed: 200.0,
            jump_force: 500.0,
        },
        // Reverse jumper - move left and jump
        Character {
            name: "Lefty".to_string(),
            action_a: Action::MoveLeft,
            action_b: Action::Jump,
            move_speed: 200.0,
            jump_force: 500.0,
        },
        // Speed demon - fast but can only go right and jump
        Character {
            name: "Speedy".to_string(),
            action_a: Action::Jump,
            action_b: Action::MoveRight,
            move_speed: 400.0,
            jump_force: 350.0,
        },
        // Slow tank - slow but high jump
        Character {
            name: "Tanky".to_string(),
            action_a: Action::MoveLeft,
            action_b: Action::MoveRight,
            move_speed: 150.0,
            jump_force: 600.0,
        },
    ]
}

/// Get a character for a given slot (cycles through roster)
pub fn character_for_slot(slot: usize) -> Character {
    let roster = all_characters();
    roster[slot % roster.len()].clone()
}
