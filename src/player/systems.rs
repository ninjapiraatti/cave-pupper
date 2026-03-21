use bevy::prelude::*;

use crate::constants::{MAX_PLAYERS, PLAYER_COLORS, PLAYER_KEYS, STICK_DEADZONE};

use super::components::{InputSource, Player};
use super::registry::PlayerRegistry;

pub fn player_join(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
    mut registry: ResMut<PlayerRegistry>,
) {
    if registry.player_count() >= MAX_PLAYERS {
        return;
    }

    // Check keyboard joins
    for (key_index, keys) in PLAYER_KEYS.iter().enumerate() {
        let source = InputSource::Keyboard(key_index);
        if registry.is_input_taken(&source) {
            continue;
        }

        // Check if action key (5th key in tuple) is pressed
        if keyboard.just_pressed(keys.4) {
            spawn_player(&mut commands, &mut meshes, &mut materials, &mut registry, source);
        }
    }

    // Check gamepad joins
    for (gamepad_entity, gamepad) in &gamepads {
        let source = InputSource::Gamepad(gamepad_entity);
        if registry.is_input_taken(&source) {
            continue;
        }

        if gamepad.just_pressed(GamepadButton::South)
            || gamepad.just_pressed(GamepadButton::East)
            || gamepad.just_pressed(GamepadButton::West)
            || gamepad.just_pressed(GamepadButton::North)
            || gamepad.just_pressed(GamepadButton::Start)
        {
            spawn_player(&mut commands, &mut meshes, &mut materials, &mut registry, source);
        }
    }
}

fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    registry: &mut ResMut<PlayerRegistry>,
    source: InputSource,
) {
    let player_id = registry.player_count();
    let color = PLAYER_COLORS[player_id];

    // Calculate spawn position (spread players out)
    let angle = (player_id as f32) * std::f32::consts::TAU / 4.0;
    let spawn_distance = 150.0;
    let spawn_pos = Vec3::new(
        angle.cos() * spawn_distance,
        angle.sin() * spawn_distance,
        0.0,
    );

    let entity = commands
        .spawn((
            Player {
                id: player_id,
                input_source: source,
            },
            Mesh2d(meshes.add(Circle::new(30.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
            Transform::from_translation(spawn_pos),
        ))
        .id();

    registry.add_player(entity, source);
    info!("Player {} joined with {:?}", player_id + 1, source);
}

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut players: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    let speed = 300.0;

    for (player, mut transform) in &mut players {
        let mut direction = Vec2::ZERO;

        match player.input_source {
            InputSource::Keyboard(key_index) => {
                let keys = PLAYER_KEYS[key_index];
                if keyboard.pressed(keys.0) {
                    direction.y += 1.0;
                }
                if keyboard.pressed(keys.1) {
                    direction.y -= 1.0;
                }
                if keyboard.pressed(keys.2) {
                    direction.x -= 1.0;
                }
                if keyboard.pressed(keys.3) {
                    direction.x += 1.0;
                }
            }
            InputSource::Gamepad(gamepad_entity) => {
                if let Ok(gamepad) = gamepads.get(gamepad_entity) {
                    let left_stick = gamepad.left_stick();

                    // Apply deadzone to prevent drift
                    if left_stick.length() > STICK_DEADZONE {
                        direction.x = left_stick.x;
                        direction.y = left_stick.y;
                    }

                    // Also support D-pad
                    if gamepad.pressed(GamepadButton::DPadUp) {
                        direction.y += 1.0;
                    }
                    if gamepad.pressed(GamepadButton::DPadDown) {
                        direction.y -= 1.0;
                    }
                    if gamepad.pressed(GamepadButton::DPadLeft) {
                        direction.x -= 1.0;
                    }
                    if gamepad.pressed(GamepadButton::DPadRight) {
                        direction.x += 1.0;
                    }
                }
            }
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            transform.translation.x += direction.x * speed * time.delta_secs();
            transform.translation.y += direction.y * speed * time.delta_secs();
        }
    }
}
