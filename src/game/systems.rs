use bevy::prelude::*;

use crate::player::{Player, PlayerRegistry};
use crate::state::GameState;

#[derive(Component)]
pub struct GameUI;

pub fn setup_game(mut commands: Commands, mut registry: ResMut<PlayerRegistry>) {
    // Clear player registry
    registry.players.clear();

    // Spawn game UI
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
            // Instructions at top
            parent
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Press your action key to join! (ESC to return to menu)"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn cleanup_game(
    mut commands: Commands,
    game_ui: Query<Entity, With<GameUI>>,
    players: Query<Entity, With<Player>>,
) {
    for entity in &game_ui {
        commands.entity(entity).despawn();
    }
    for entity in &players {
        commands.entity(entity).despawn();
    }
}

pub fn back_to_menu(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
