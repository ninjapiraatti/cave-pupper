use bevy::prelude::*;

/// Marker for all arena elements (for cleanup)
#[derive(Component)]
pub struct ArenaElement;

/// A solid platform players can stand on
#[derive(Component)]
pub struct Platform;

/// Kills players on contact (sensor-based, like bottom of screen)
#[derive(Component)]
pub struct DeathZone;

/// Where players spawn
#[derive(Component)]
pub struct SpawnPoint;

// === Composable Level Object Properties ===

/// Kills players on contact (for physical objects like spikes)
#[derive(Component)]
pub struct Deadly;

/// Can be destroyed. When health reaches 0, entity is despawned.
#[derive(Component)]
pub struct Destructible {
    pub health: f32,
    pub max_health: f32,
}

impl Destructible {
    pub fn new(health: f32) -> Self {
        Self {
            health,
            max_health: health,
        }
    }

    pub fn damage(&mut self, amount: f32) -> bool {
        self.health = (self.health - amount).max(0.0);
        self.health <= 0.0
    }
}

/// Moves along a path of waypoints
#[derive(Component)]
pub struct Movable {
    pub waypoints: Vec<Vec2>,
    pub speed: f32,
    pub current_index: usize,
    pub forward: bool, // ping-pong direction
    pub current_velocity: Vec2, // Tracks current movement for passenger transfer
}

impl Movable {
    pub fn new(waypoints: Vec<Vec2>, speed: f32) -> Self {
        Self {
            waypoints,
            speed,
            current_index: 0,
            forward: true,
            current_velocity: Vec2::ZERO,
        }
    }
}

/// Temporarily disappears when stood on, then reappears
#[derive(Component)]
pub struct Crumbling {
    pub stand_time: f32,   // How long player must stand before crumbling starts
    pub delay: f32,        // Time before crumbling after stand_time reached
    pub respawn_time: f32, // Time to respawn after crumbling
}

/// Tracks crumbling state
#[derive(Component)]
pub struct CrumblingState {
    pub standing_timer: f32, // How long player has been standing
    pub crumble_timer: f32,  // Countdown to crumble after triggered
    pub triggered: bool,     // Stand time reached, now counting down to crumble
    pub crumbled: bool,
}
