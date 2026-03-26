use avian2d::prelude::*;
use bevy::prelude::*;

use crate::arena::{DeathZone, SpawnPoint};
use crate::characters::{AnimationPlayer, FacingDirection, Grounded};
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

/// Preview duration in seconds
const PREVIEW_DURATION: f32 = 1.5;

pub fn handle_join_respawn(
    inputs: Res<PlayerInputs>,
    mut slots: ResMut<PlayerSlots>,
) {
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
                    // Pick random character and start preview
                    let roster = crate::characters::all_characters();
                    let char_index = rand::random_range(0..roster.len());
                    slots.set(slot, SlotState::Previewing(char_index, PREVIEW_DURATION));
                    info!("Player {} previewing character: {}", slot, roster[char_index].name);
                }
            }
            SlotState::Previewing(_, _) | SlotState::Alive(_) => {}
        }
    }
}

/// Updates preview timers and spawns players when preview is done
pub fn update_previews(
    mut commands: Commands,
    mut slots: ResMut<PlayerSlots>,
    spawn_points: Query<&Transform, With<SpawnPoint>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
) {
    let spawn_positions: Vec<Vec2> = spawn_points
        .iter()
        .map(|t| t.translation.truncate())
        .collect();

    if spawn_positions.is_empty() {
        return;
    }

    for slot in 0..8 {
        if let SlotState::Previewing(char_index, remaining) = slots.get(slot) {
            let new_remaining = remaining - time.delta_secs();

            if new_remaining <= 0.0 {
                // Preview done, spawn the player
                let spawn_pos = spawn_positions[slot % spawn_positions.len()];
                let entity = spawn_player(
                    &mut commands,
                    slot,
                    char_index,
                    spawn_pos,
                    &asset_server,
                    &mut texture_atlas_layouts,
                );
                slots.set(slot, SlotState::Alive(entity));
                info!("Player {} spawned", slot);
            } else {
                slots.set(slot, SlotState::Previewing(char_index, new_remaining));
            }
        }
    }
}

fn spawn_player(
    commands: &mut Commands,
    slot: usize,
    character_index: usize,
    position: Vec2,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> Entity {
    let color = SLOT_COLORS[slot];
    let roster = crate::characters::all_characters();
    let character = roster[character_index % roster.len()].clone();
    info!("Spawning {} for slot {}", character.name, slot);

    // Capsule collider: radius 10.5 (21 wide), length 70 (total height ~91)
    let collider_radius = 10.5;
    let collider_length = 70.0; // Increased by 50% from ~47
    let collider_total_height = collider_length + collider_radius * 2.0;

    let mut entity_commands = commands.spawn((
        Player { slot },
        Health::new(100),
        character.clone(),
        Grounded(false),
        RigidBody::Dynamic,
        Collider::capsule(collider_radius, collider_length),
        CollidingEntities::default(),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        Friction::new(0.3),
        Restitution::new(0.0),
        // Spawn with collider bottom at the spawn point
        Transform::from_translation(position.extend(0.0) + Vec3::Y * (collider_total_height / 2.0)),
    ));

    // Add sprite - either from atlas or fallback colored box
    // Use custom anchor to align sprite bottom with collider bottom
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

        // Custom anchor: sprite bottom should be at collider bottom (entity.y - collider_height/2)
        // Anchor Y of -0.5 = bottom at entity, we need bottom at entity - collider_height/2
        // So we need anchor Y = -0.5 + (collider_height/2) / sprite_height
        let sprite_height = sprite_config.tile_size.y as f32;
        let anchor_y = -0.5 + (collider_total_height / 2.0) / sprite_height;

        entity_commands.insert((
            Sprite {
                image: texture,
                texture_atlas: Some(TextureAtlas {
                    layout: layout_handle,
                    index: 0,
                }),
                ..default()
            },
            bevy::sprite::Anchor(Vec2::new(0.0, anchor_y)),
            AnimationPlayer::new(sprite_config.frame_time, sprite_config.frame_count),
            FacingDirection::default(),
        ));
    } else {
        // Fallback colored box matches collider size
        entity_commands.insert(Sprite {
            color,
            custom_size: Some(Vec2::new(collider_radius * 2.0, collider_total_height)),
            ..default()
        });
    }

    entity_commands.id()
}

pub fn update_grounded(
    mut query: Query<(Entity, &mut Grounded)>,
    collisions: Collisions,
) {
    for (entity, mut grounded) in &mut query {
        grounded.0 = false;

        // Check all contacts for this entity
        for contacts in collisions.collisions_with(entity) {
            for manifold in contacts.manifolds.iter() {
                // Normal points from body1 to body2
                // If we're body1, a ground contact has normal pointing down (we're above)
                // If we're body2, a ground contact has normal pointing up (we're below the other)
                let normal = if contacts.body1 == Some(entity) {
                    -manifold.normal
                } else {
                    manifold.normal
                };

                // Ground contact if normal points mostly up (we're standing on something)
                if normal.y > 0.5 {
                    grounded.0 = true;
                    return;
                }
            }
        }
    }
}

pub fn apply_friction(
    inputs: Res<PlayerInputs>,
    mut query: Query<(&Player, &Grounded, &mut LinearVelocity)>,
) {
    for (player, grounded, mut velocity) in &mut query {
        let input = inputs.get(player.slot);

        // Only apply horizontal friction when grounded and not pressing movement keys
        if grounded.0 && !input.key_a_pressed && !input.key_b_pressed {
            velocity.x *= 0.85;
            if velocity.x.abs() < 1.0 {
                velocity.x = 0.0;
            }
        }
    }
}

/// Prevents players from sticking to walls by detecting wall contacts
pub fn handle_wall_contacts(
    mut query: Query<(Entity, &Grounded, &mut LinearVelocity)>,
    collisions: Collisions,
) {
    for (entity, grounded, mut velocity) in &mut query {
        if grounded.0 {
            continue; // Don't interfere when grounded
        }

        for contacts in collisions.collisions_with(entity) {
            for manifold in contacts.manifolds.iter() {
                let normal = if contacts.body1 == Some(entity) {
                    -manifold.normal
                } else {
                    manifold.normal
                };

                // Wall contact if normal is mostly horizontal
                if normal.y.abs() < 0.3 {
                    // Cancel velocity toward the wall
                    if normal.x > 0.0 && velocity.x < 0.0 {
                        // Wall is to the left, player moving left
                        velocity.x = 0.0;
                    } else if normal.x < 0.0 && velocity.x > 0.0 {
                        // Wall is to the right, player moving right
                        velocity.x = 0.0;
                    }
                }
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
