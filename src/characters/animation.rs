use bevy::prelude::*;

use crate::player::Player;
use avian2d::prelude::*;

/// Which animation row to play
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AnimationState {
    #[default]
    Idle,
    WalkLeft,
    WalkRight,
}

impl AnimationState {
    /// Get the starting frame index for this animation (row * 10)
    pub fn start_frame(&self) -> usize {
        match self {
            AnimationState::Idle => 0,
            AnimationState::WalkLeft => 10,
            AnimationState::WalkRight => 20,
        }
    }
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

/// Update animation state based on velocity
pub fn update_animation_state(
    mut query: Query<(&LinearVelocity, &mut AnimationPlayer), With<Player>>,
) {
    for (velocity, mut anim) in &mut query {
        let new_state = if velocity.x < -10.0 {
            AnimationState::WalkLeft
        } else if velocity.x > 10.0 {
            AnimationState::WalkRight
        } else {
            AnimationState::Idle
        };

        if anim.state != new_state {
            anim.state = new_state;
            anim.frame = 0;
            anim.timer.reset();
        }
    }
}

/// Cycle through animation frames
pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut AnimationPlayer, &mut Sprite)>,
) {
    for (mut anim, mut sprite) in &mut query {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished() {
            anim.frame = (anim.frame + 1) % anim.frame_count;
        }

        // Set the atlas index: row start + current frame
        if let Some(ref mut atlas) = sprite.texture_atlas {
            atlas.index = anim.state.start_frame() + anim.frame;
        }
    }
}
