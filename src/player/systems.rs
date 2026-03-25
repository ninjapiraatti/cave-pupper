use avian2d::prelude::*;
use bevy::prelude::*;

use crate::arena::{DeathZone, SpawnPoint};
use crate::characters::{character_for_slot, AnimationPlayer, FacingDirection, Grounded};
use crate::combat::Health;
use crate::input::PlayerInputs;

use super::components::{Player, PlayerSlots, SlotState};

/// Player colors for each slot (used as fallback and tint)
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

pub fn reset_slots(mut slots: ResMut<PlayerSlots>) {
    slots.reset();
}

pub fn handle_join_respawn(
    mut commands: Commands,
    inputs: Res<PlayerInputs>,
    mut slots: ResMut<PlayerSlots>,
    spawn_points: Query<&Transform, With<SpawnPoint>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let spawn_positions: Vec<Vec2> = spawn_points
        .iter()
        .map(|t| t.translation.truncate())
        .collect();

    if spawn_positions.is_empty() {
        return;
    }

    for slot in 0..8 {
        let input = inputs.get(slot);
        let state = slots.get(slot);

        match state {
            SlotState::Empty => {
                if input.any_just_pressed() {
                    slots.set(slot, SlotState::WaitingToSpawn);
                    info!("Slot {} claimed, press again to spawn", slot);
                }
            }
            SlotState::WaitingToSpawn | SlotState::Dead => {
                if input.any_just_pressed() {
                    let spawn_pos = spawn_positions[slot % spawn_positions.len()];
                    let entity = spawn_player(
                        &mut commands,
                        slot,
                        spawn_pos,
                        &asset_server,
                        &mut texture_atlas_layouts,
                    );
                    slots.set(slot, SlotState::Alive(entity));
                    info!("Player {} spawned", slot);
                }
            }
            SlotState::Alive(_) => {}
        }
    }
}

fn spawn_player(
    commands: &mut Commands,
    slot: usize,
    position: Vec2,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> Entity {
    let color = SLOT_COLORS[slot];
    let character = character_for_slot(slot);
    info!("Spawning {} for slot {}", character.name, slot);

    let mut entity_commands = commands.spawn((
        Player { slot },
        Health::new(100),
        character.clone(),
        Grounded(false),
        RigidBody::Dynamic,
        Collider::rectangle(30.0, 40.0),
        CollidingEntities::default(),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        Friction::new(0.3),
        Restitution::new(0.0),
        Transform::from_translation(position.extend(0.0)),
    ));

    // Add sprite - either from atlas or fallback colored box
    if let Some(ref sprite_config) = character.sprite {
        let texture = asset_server.load(&sprite_config.path);
        let layout = TextureAtlasLayout::from_grid(
            sprite_config.tile_size,
            sprite_config.columns,
            sprite_config.rows,
            None,
            None,
        );
        let layout_handle = texture_atlas_layouts.add(layout);

        entity_commands.insert((
            Sprite {
                image: texture,
                texture_atlas: Some(TextureAtlas {
                    layout: layout_handle,
                    index: 0,
                }),
                ..default()
            },
            AnimationPlayer::new(sprite_config.frame_time, sprite_config.frame_count),
            FacingDirection::default(),
        ));
    } else {
        entity_commands.insert(Sprite {
            color,
            custom_size: Some(Vec2::new(30.0, 40.0)),
            ..default()
        });
    }

    entity_commands.id()
}

pub fn update_grounded(
    mut query: Query<(Entity, &mut Grounded, &CollidingEntities)>,
    transforms: Query<&Transform>,
) {
    for (entity, mut grounded, colliding) in &mut query {
        let player_y = transforms
            .get(entity)
            .map(|t| t.translation.y)
            .unwrap_or(0.0);

        grounded.0 = colliding.iter().any(|other| {
            if let Ok(other_tf) = transforms.get(*other) {
                other_tf.translation.y < player_y - 10.0
            } else {
                false
            }
        });
    }
}

pub fn apply_friction(
    inputs: Res<PlayerInputs>,
    mut query: Query<(&Player, &mut LinearVelocity)>,
) {
    for (player, mut velocity) in &mut query {
        let input = inputs.get(player.slot);

        if !input.key_a_pressed && !input.key_b_pressed {
            velocity.x *= 0.85;
            if velocity.x.abs() < 1.0 {
                velocity.x = 0.0;
            }
        }
    }
}

pub fn check_death_zone(
    mut commands: Commands,
    mut slots: ResMut<PlayerSlots>,
    collisions: Collisions,
    players: Query<&Player>,
    death_zones: Query<Entity, With<DeathZone>>,
) {
    for death_zone in &death_zones {
        for player_entity in collisions.entities_colliding_with(death_zone) {
            if let Ok(player) = players.get(player_entity) {
                info!("Player {} fell into death zone", player.slot);
                slots.set(player.slot, SlotState::Dead);
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn cleanup_players(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
