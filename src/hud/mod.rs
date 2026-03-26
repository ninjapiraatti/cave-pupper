use bevy::prelude::*;

use crate::characters::all_characters;
use crate::input::MAX_SLOTS;
use crate::player::{PlayerSlots, SlotState};
use crate::state::GameState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_hud)
            .add_systems(OnExit(GameState::Playing), cleanup_hud)
            .add_systems(
                Update,
                update_hud.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
struct HudRoot;

#[derive(Component)]
struct PlayerSlotUi {
    slot: usize,
}

#[derive(Component)]
struct PlayerNameText {
    slot: usize,
}

#[derive(Component)]
struct PlayerCharacterImage {
    slot: usize,
    /// Currently displayed character index (to avoid reloading)
    current_char: Option<usize>,
}

/// Player colors for each slot
const SLOT_COLORS: [Color; 8] = [
    Color::srgb(0.2, 0.6, 1.0),  // Blue
    Color::srgb(1.0, 0.3, 0.3),  // Red
    Color::srgb(0.3, 1.0, 0.3),  // Green
    Color::srgb(1.0, 1.0, 0.3),  // Yellow
    Color::srgb(1.0, 0.3, 1.0),  // Magenta
    Color::srgb(0.3, 1.0, 1.0),  // Cyan
    Color::srgb(1.0, 0.6, 0.2),  // Orange
    Color::srgb(0.6, 0.3, 1.0),  // Purple
];

fn spawn_hud(mut commands: Commands) {
    // Root container at top of screen
    commands
        .spawn((
            HudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
        ))
        .with_children(|parent| {
            // Spawn 8 player slots
            for slot in 0..MAX_SLOTS {
                parent
                    .spawn((
                        PlayerSlotUi { slot },
                        Node {
                            width: Val::Percent(12.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                    ))
                    .with_children(|slot_parent| {
                        // Character image container
                        slot_parent.spawn((
                            PlayerCharacterImage { slot, current_char: None },
                            ImageNode::default(),
                            Node {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                margin: UiRect::bottom(Val::Px(4.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.5)),
                        ));

                        // Player name
                        slot_parent.spawn((
                            PlayerNameText { slot },
                            Text::new(format!("P{}", slot + 1)),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });
}

fn update_hud(
    slots: Res<PlayerSlots>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut slot_query: Query<(&PlayerSlotUi, &mut BackgroundColor)>,
    mut name_query: Query<(&PlayerNameText, &mut Text, &mut TextColor)>,
    mut image_query: Query<(
        &mut PlayerCharacterImage,
        &mut BackgroundColor,
        &mut ImageNode,
    ), Without<PlayerSlotUi>>,
) {
    let roster = all_characters();

    for (slot_ui, mut bg) in &mut slot_query {
        let state = slots.get(slot_ui.slot);
        let is_active = !matches!(state, SlotState::Empty);

        // Highlight active slots
        *bg = if is_active {
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.5))
        } else {
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0))
        };
    }

    for (name_ui, mut text, mut text_color) in &mut name_query {
        let state = slots.get(name_ui.slot);
        let name = &slots.names[name_ui.slot];

        // Update name text
        **text = name.clone();

        // Color based on state
        *text_color = match state {
            SlotState::Empty => TextColor(Color::srgba(0.5, 0.5, 0.5, 0.5)),
            SlotState::WaitingToSpawn => TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
            SlotState::Previewing(_, _) => TextColor(Color::srgba(1.0, 1.0, 0.5, 1.0)), // Yellow flash
            SlotState::Alive(_) => TextColor(SLOT_COLORS[name_ui.slot]),
            SlotState::Dead => TextColor(Color::srgba(0.8, 0.2, 0.2, 0.8)),
        };
    }

    for (mut image_ui, mut bg, mut image_node) in &mut image_query {
        let state = slots.get(image_ui.slot);
        let slot_color = SLOT_COLORS[image_ui.slot];

        match state {
            SlotState::Empty => {
                *bg = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.3));
                image_node.image = Handle::default();
                image_ui.current_char = None;
            }
            SlotState::WaitingToSpawn => {
                *bg = BackgroundColor(slot_color.with_alpha(0.5));
                image_node.image = Handle::default();
                image_ui.current_char = None;
            }
            SlotState::Previewing(char_index, remaining) => {
                let char_def = &roster[char_index % roster.len()];

                // Load sprite if character changed
                if image_ui.current_char != Some(char_index) {
                    if let Some(ref sprite_config) = char_def.sprite {
                        // Load the sprite texture
                        let texture = asset_server.load(&sprite_config.path);
                        image_node.image = texture;

                        // Set up texture atlas for first frame
                        let layout = TextureAtlasLayout::from_grid(
                            sprite_config.tile_size,
                            sprite_config.columns,
                            sprite_config.rows,
                            None,
                            None,
                        );
                        let layout_handle = texture_atlas_layouts.add(layout);
                        image_node.texture_atlas = Some(TextureAtlas {
                            layout: layout_handle,
                            index: 0, // First frame
                        });

                        *bg = BackgroundColor(Color::NONE);
                    } else {
                        // No sprite, use colored background
                        image_node.image = Handle::default();
                        image_node.texture_atlas = None;
                    }
                    image_ui.current_char = Some(char_index);
                }

                // Flash effect for characters without sprites
                if char_def.sprite.is_none() {
                    let flash = (remaining * 8.0).sin().abs();
                    *bg = BackgroundColor(slot_color.with_alpha(0.5 + flash * 0.5));
                }
            }
            SlotState::Alive(_) => {
                // Keep showing the character sprite/color
                if image_ui.current_char.is_none() {
                    *bg = BackgroundColor(slot_color);
                }
            }
            SlotState::Dead => {
                *bg = BackgroundColor(slot_color.with_alpha(0.3));
                image_node.image = Handle::default();
                image_ui.current_char = None;
            }
        }
    }
}

fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
