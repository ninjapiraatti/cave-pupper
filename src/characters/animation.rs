use bevy::prelude::*;

use crate::player::Player;
use avian2d::prelude::*;

/// Which animation to play
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AnimationState {
    #[default]
    Idle,
    Walking,
}

/// Which direction the character is facing (persists when idle)
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FacingDirection {
    #[default]
    Right,
    Left,
}

/// Tracks animation playback
#[derive(Component)]
pub struct AnimationPlayer {
    pub state: AnimationState,
    pub frame: usize,
    pub timer: Timer,
    pub frame_count: usize,
}

impl AnimationPlayer {
    pub fn new(frame_time: f32, frame_count: usize) -> Self {
        Self {
            state: AnimationState::Idle,
            frame: 0,
            timer: Timer::from_seconds(frame_time, TimerMode::Repeating),
            frame_count,
        }
    }
}

/// Update animation state and facing direction based on velocity
pub fn update_animation_state(
    mut query: Query<(&LinearVelocity, &mut AnimationPlayer, &mut FacingDirection), With<Player>>,
) {
    for (velocity, mut anim, mut facing) in &mut query {
        let is_walking = velocity.x.abs() > 10.0;
        let new_state = if is_walking {
            AnimationState::Walking
        } else {
            AnimationState::Idle
        };

        // Update facing direction when moving
        if velocity.x < -10.0 {
            *facing = FacingDirection::Left;
        } else if velocity.x > 10.0 {
            *facing = FacingDirection::Right;
        }
        // When idle, facing direction stays the same

        if anim.state != new_state {
            anim.state = new_state;
            anim.frame = 0;
            anim.timer.reset();
        }
    }
}

/// Cycle through animation frames and handle sprite flipping
pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut AnimationPlayer, &FacingDirection, &mut Sprite)>,
) {
    for (mut anim, facing, mut sprite) in &mut query {
        // Handle sprite flipping based on facing direction
        sprite.flip_x = *facing == FacingDirection::Left;

        match anim.state {
            AnimationState::Idle => {
                // Show first frame when idle
                if let Some(ref mut atlas) = sprite.texture_atlas {
                    atlas.index = 0;
                }
            }
            AnimationState::Walking => {
                // Cycle through walk animation
                anim.timer.tick(time.delta());

                if anim.timer.just_finished() {
                    anim.frame = (anim.frame + 1) % anim.frame_count;
                }

                if let Some(ref mut atlas) = sprite.texture_atlas {
                    atlas.index = anim.frame;
                }
            }
        }
    }
}
