use avian2d::prelude::*;
use bevy::prelude::*;

use crate::input::PlayerInputs;
use crate::player::Player;

/// Available actions that can be assigned to keys
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    MoveLeft,
    MoveRight,
    Jump,
    // Future actions can be added here:
    // Shoot,
    // Dash,
    // Block,
    // etc.
}

/// Character definition - what actions are bound to each key
#[derive(Component, Clone)]
pub struct Character {
    pub name: String,
    pub action_a: Action,
    pub action_b: Action,
    pub move_speed: f32,
    pub jump_force: f32,
}

impl Character {
    /// Basic mover character - just walks left and right
    pub fn basic_mover() -> Self {
        Self {
            name: "Mover".to_string(),
            action_a: Action::MoveLeft,
            action_b: Action::MoveRight,
            move_speed: 250.0,
            jump_force: 400.0,
        }
    }

    /// Jumper character - can jump and move right
    pub fn jumper() -> Self {
        Self {
            name: "Jumper".to_string(),
            action_a: Action::Jump,
            action_b: Action::MoveRight,
            move_speed: 250.0,
            jump_force: 500.0,
        }
    }
}

/// Tracks if player is on the ground
#[derive(Component)]
pub struct Grounded(pub bool);

/// Execute character actions based on input
pub fn execute_actions(
    inputs: Res<PlayerInputs>,
    mut query: Query<(&Player, &Character, &mut LinearVelocity, &Grounded)>,
) {
    for (player, character, mut velocity, grounded) in &mut query {
        let input = inputs.get(player.slot);

        // Execute action A
        if input.key_a_pressed || input.key_a_just_pressed {
            execute_action(
                character.action_a,
                character,
                &mut velocity,
                grounded.0,
                input.key_a_just_pressed,
            );
        }

        // Execute action B
        if input.key_b_pressed || input.key_b_just_pressed {
            execute_action(
                character.action_b,
                character,
                &mut velocity,
                grounded.0,
                input.key_b_just_pressed,
            );
        }
    }
}

fn execute_action(
    action: Action,
    character: &Character,
    velocity: &mut LinearVelocity,
    grounded: bool,
    just_pressed: bool,
) {
    match action {
        Action::MoveLeft => {
            velocity.x = -character.move_speed;
        }
        Action::MoveRight => {
            velocity.x = character.move_speed;
        }
        Action::Jump => {
            if just_pressed && grounded {
                velocity.y = character.jump_force;
            }
        }
    }
}
