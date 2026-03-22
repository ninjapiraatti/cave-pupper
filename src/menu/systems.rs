use bevy::prelude::*;

use crate::input::InputBindings;
use crate::state::GameState;

#[derive(Component)]
pub struct MenuUI;

pub fn setup_menu(mut commands: Commands, bindings: Res<InputBindings>) {
    commands
        .spawn((
            MenuUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(15.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("CAVE PUPPER"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Subtitle
            parent.spawn((
                Text::new("2-Key Brawler"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            // Spacer
            parent.spawn(Node {
                height: Val::Px(30.0),
                ..default()
            });

            // Controls header
            parent.spawn((
                Text::new("CONTROLS"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.3)),
            ));

            // Player bindings
            let colors = [
                Color::srgb(0.2, 0.6, 1.0),
                Color::srgb(1.0, 0.3, 0.3),
                Color::srgb(0.3, 1.0, 0.3),
                Color::srgb(1.0, 1.0, 0.3),
            ];

            for (slot, color) in colors.iter().enumerate() {
                let binding = bindings.get(slot);
                parent.spawn((
                    Text::new(format!(
                        "Player {}: {:?} = Left, {:?} = Right",
                        slot + 1,
                        binding.key_a,
                        binding.key_b
                    )),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(*color),
                ));
            }

            // More slots hint
            parent.spawn((
                Text::new("(4 more slots: A/S, D/F, G/H, O/P)"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));

            // Spacer
            parent.spawn(Node {
                height: Val::Px(30.0),
                ..default()
            });

            // Instructions
            parent.spawn((
                Text::new("Press SPACE to start, then press your keys to join!"),
                TextFont {
                    font_size: 24.0,
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
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
}
