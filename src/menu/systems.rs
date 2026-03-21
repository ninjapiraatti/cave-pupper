use bevy::prelude::*;

use crate::constants::PLAYER_COLORS;
use crate::state::GameState;

#[derive(Component)]
pub struct MenuUI;

pub fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            MenuUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("EMO NEMO"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Subtitle
            parent.spawn((
                Text::new("Couch Game Template"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            // Spacer
            parent.spawn(Node {
                height: Val::Px(40.0),
                ..default()
            });

            // Controls info
            parent.spawn((
                Text::new("CONTROLS"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.3)),
            ));

            // Player 1
            parent.spawn((
                Text::new("Player 1: WASD + Space"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(PLAYER_COLORS[0]),
            ));

            // Player 2
            parent.spawn((
                Text::new("Player 2: Arrow Keys + Enter"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(PLAYER_COLORS[1]),
            ));

            // Player 3
            parent.spawn((
                Text::new("Player 3: IJKL + U"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(PLAYER_COLORS[2]),
            ));

            // Player 4
            parent.spawn((
                Text::new("Player 4: Numpad 8456 + 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(PLAYER_COLORS[3]),
            ));

            // Gamepad info
            parent.spawn((
                Text::new("Gamepads: Any button to join, Left stick to move"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            // Spacer
            parent.spawn(Node {
                height: Val::Px(40.0),
                ..default()
            });

            // Start instruction
            parent.spawn((
                Text::new("Press SPACE or any gamepad button to start"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Check keyboard
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
        return;
    }

    // Check gamepads
    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::South)
            || gamepad.just_pressed(GamepadButton::Start)
        {
            next_state.set(GameState::Playing);
            return;
        }
    }
}
