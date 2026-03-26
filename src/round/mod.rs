use bevy::prelude::*;

use crate::arena::{spawn_level, ArenaElement, CurrentLevel};
use crate::levels::all_levels;
use crate::player::{Player, PlayerSlots, SlotState};
use crate::state::GameState;

pub struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoundState>()
            .add_systems(OnEnter(GameState::Playing), reset_round)
            .add_systems(
                Update,
                (check_arm_round, check_winner)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

/// Tracks the state of the current round
#[derive(Resource, Default)]
pub struct RoundState {
    /// True once 2+ players have been alive at the same time
    pub armed: bool,
    /// Brief delay before transitioning to prevent instant transitions
    pub transition_timer: Option<f32>,
}

fn reset_round(mut round_state: ResMut<RoundState>) {
    round_state.armed = false;
    round_state.transition_timer = None;
}

/// Arms the round when 2+ players are alive
fn check_arm_round(mut round_state: ResMut<RoundState>, slots: Res<PlayerSlots>) {
    if round_state.armed {
        return;
    }

    let alive_count = slots
        .states
        .iter()
        .filter(|s| matches!(s, SlotState::Alive(_)))
        .count();

    if alive_count >= 2 {
        round_state.armed = true;
        info!("Round armed! {} players alive", alive_count);
    }
}

/// Checks for a winner (1 player left when armed) and transitions to next level
fn check_winner(
    mut commands: Commands,
    mut round_state: ResMut<RoundState>,
    mut current_level: ResMut<CurrentLevel>,
    mut slots: ResMut<PlayerSlots>,
    players: Query<Entity, With<Player>>,
    arena_elements: Query<Entity, With<ArenaElement>>,
    time: Res<Time>,
) {
    if !round_state.armed {
        return;
    }

    let alive_count = slots
        .states
        .iter()
        .filter(|s| matches!(s, SlotState::Alive(_)))
        .count();

    // Start transition timer when only 1 player left
    if alive_count <= 1 && round_state.transition_timer.is_none() {
        round_state.transition_timer = Some(1.5); // 1.5 second delay
        info!("Winner! Transitioning to next level...");
    }

    // Count down and transition
    if let Some(ref mut timer) = round_state.transition_timer {
        *timer -= time.delta_secs();

        if *timer <= 0.0 {
            // Advance to next level
            current_level.next();
            let levels = all_levels();
            info!(
                "Loading level {}: {}",
                current_level.index % levels.len(),
                current_level.get_level().name
            );

            // Cleanup current arena
            for entity in &arena_elements {
                commands.entity(entity).despawn();
            }

            // Cleanup players
            for entity in &players {
                commands.entity(entity).despawn();
            }

            // Reset player slots to preserve who's playing but mark as dead for respawn
            for slot in slots.states.iter_mut() {
                if !matches!(slot, SlotState::Empty) {
                    *slot = SlotState::Dead;
                }
            }

            // Spawn new level
            let level = current_level.get_level();
            spawn_level(&mut commands, &level);

            // Reset round state
            round_state.armed = false;
            round_state.transition_timer = None;
        }
    }
}
