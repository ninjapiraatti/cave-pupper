use avian2d::prelude::*;
use bevy::prelude::*;

use super::{Crumbling, CrumblingState, Deadly, Destructible, Movable};
use crate::player::{Player, SlotState};

/// Moves entities along their waypoint paths (ping-pong style)
pub fn move_movables(mut query: Query<(&mut Transform, &mut Movable)>, time: Res<Time>) {
    for (mut transform, mut movable) in &mut query {
        if movable.waypoints.len() < 2 {
            movable.current_velocity = Vec2::ZERO;
            continue;
        }

        let target = movable.waypoints[movable.current_index];
        let current = transform.translation.truncate();
        let direction = target - current;
        let distance = direction.length();

        if distance < 1.0 {
            // Reached waypoint, move to next
            movable.current_velocity = Vec2::ZERO;
            if movable.forward {
                if movable.current_index + 1 >= movable.waypoints.len() {
                    movable.forward = false;
                    movable.current_index -= 1;
                } else {
                    movable.current_index += 1;
                }
            } else if movable.current_index == 0 {
                movable.forward = true;
                movable.current_index += 1;
            } else {
                movable.current_index -= 1;
            }
        } else {
            let velocity = direction.normalize() * movable.speed;
            movable.current_velocity = velocity;
            let movement = velocity * time.delta_secs();
            if movement.length() > distance {
                transform.translation.x = target.x;
                transform.translation.y = target.y;
            } else {
                transform.translation.x += movement.x;
                transform.translation.y += movement.y;
            }
        }
    }
}

/// Moves players standing on moving platforms
pub fn move_players_on_platforms(
    platform_query: Query<(Entity, &Transform, &Movable, &Collider)>,
    mut player_query: Query<(&mut Transform, &CollidingEntities), (With<Player>, Without<Movable>)>,
    time: Res<Time>,
) {
    for (mut player_transform, colliding) in &mut player_query {
        for (platform_entity, platform_transform, movable, _collider) in &platform_query {
            if colliding.contains(&platform_entity) {
                // Check if player is above the platform (standing on it, not beside it)
                let player_y = player_transform.translation.y;
                let platform_y = platform_transform.translation.y;

                // Player's bottom should be near platform's top
                if player_y > platform_y {
                    // Apply platform velocity to player
                    let movement = movable.current_velocity * time.delta_secs();
                    player_transform.translation.x += movement.x;
                    player_transform.translation.y += movement.y;
                }
            }
        }
    }
}

/// Handles deadly object contact with players
pub fn handle_deadly_contact(
    mut commands: Commands,
    deadly_query: Query<Entity, With<Deadly>>,
    player_query: Query<(Entity, &Player), With<Player>>,
    collisions: Collisions,
    mut slot_states: ResMut<crate::player::PlayerSlots>,
) {
    for deadly_entity in &deadly_query {
        for (player_entity, player) in &player_query {
            if collisions.contains(deadly_entity, player_entity) {
                slot_states.set(player.slot, SlotState::Dead);
                commands.entity(player_entity).despawn();
            }
        }
    }
}

/// Despawns destructibles that have run out of health
pub fn despawn_destroyed(
    mut commands: Commands,
    query: Query<(Entity, &Destructible)>,
) {
    for (entity, destructible) in &query {
        if destructible.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Initializes crumbling state when Crumbling component is added
pub fn init_crumbling(
    mut commands: Commands,
    query: Query<(Entity, &Crumbling), Without<CrumblingState>>,
) {
    for (entity, _) in &query {
        commands.entity(entity).insert(CrumblingState {
            standing_timer: 0.0,
            crumble_timer: 0.0,
            triggered: false,
            crumbled: false,
        });
    }
}

/// Detects when players are standing on crumbling platforms and updates timers
pub fn detect_crumbling_standing(
    crumbling_query: Query<Entity, (With<Crumbling>, With<CrumblingState>)>,
    player_query: Query<Entity, With<Player>>,
    collisions: Collisions,
    mut states: Query<(&Crumbling, &mut CrumblingState)>,
    time: Res<Time>,
) {
    for crumbling_entity in &crumbling_query {
        let mut player_standing = false;

        // Check if any player is standing on this platform
        for player_entity in &player_query {
            if let Some(contacts) = collisions.get(crumbling_entity, player_entity) {
                for manifold in contacts.manifolds.iter() {
                    // Check if player is above platform (standing on it)
                    // Normal should point from platform toward player (upward)
                    let normal = if contacts.body1 == Some(crumbling_entity) {
                        manifold.normal
                    } else {
                        -manifold.normal
                    };

                    if normal.y > 0.5 {
                        player_standing = true;
                        break;
                    }
                }
            }
            if player_standing {
                break;
            }
        }

        if let Ok((crumbling, mut state)) = states.get_mut(crumbling_entity) {
            if state.crumbled {
                continue;
            }

            if player_standing {
                if !state.triggered {
                    // Player standing, accumulate standing time
                    state.standing_timer += time.delta_secs();
                    if state.standing_timer >= crumbling.stand_time {
                        state.triggered = true;
                    }
                }
            } else if !state.triggered {
                // Player left before triggering, reset standing timer
                state.standing_timer = 0.0;
            }
        }
    }
}

/// Updates crumbling timers and handles crumble/respawn
pub fn update_crumbling(
    mut commands: Commands,
    mut query: Query<(Entity, &Crumbling, &mut CrumblingState, &mut Visibility)>,
    time: Res<Time>,
) {
    for (entity, crumbling, mut state, mut visibility) in &mut query {
        if state.triggered && !state.crumbled {
            state.crumble_timer += time.delta_secs();
            if state.crumble_timer >= crumbling.delay {
                state.crumbled = true;
                state.crumble_timer = 0.0;
                *visibility = Visibility::Hidden;
                // Make collider a sensor so players fall through
                commands.entity(entity).insert(Sensor);
            }
        } else if state.crumbled {
            state.crumble_timer += time.delta_secs();
            if state.crumble_timer >= crumbling.respawn_time {
                state.crumbled = false;
                state.triggered = false;
                state.standing_timer = 0.0;
                state.crumble_timer = 0.0;
                *visibility = Visibility::Inherited;
                // Remove sensor to make solid again
                commands.entity(entity).remove::<Sensor>();
            }
        }
    }
}
