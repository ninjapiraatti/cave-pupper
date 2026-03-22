use bevy::prelude::*;

use crate::input::InputBindings;
use crate::state::GameState;

#[derive(Component)]
pub struct GameUI;

pub fn setup_game(mut commands: Commands, bindings: Res<InputBindings>) {
    commands
        .spawn((
            GameUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Press your keys to join! (ESC = menu)"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // Show first 4 bindings
                    for slot in 0..4 {
                        let binding = bindings.get(slot);
                        parent.spawn((
                            Text::new(format!(
                                "P{}: {:?}/{:?}",
                                slot + 1,
                                binding.key_a,
                                binding.key_b
                            )),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.6, 0.6, 0.6)),
                        ));
                    }
                });
        });
}

pub fn cleanup_game(mut commands: Commands, game_ui: Query<Entity, With<GameUI>>) {
    for entity in &game_ui {
        commands.entity(entity).despawn();
    }
}

pub fn back_to_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
