use bevy::prelude::*;

/// Marker for all arena elements (for cleanup)
#[derive(Component)]
pub struct ArenaElement;

/// A solid platform players can stand on
#[derive(Component)]
pub struct Platform;

/// Kills players on contact
#[derive(Component)]
pub struct DeathZone;

/// Where players spawn
#[derive(Component)]
pub struct SpawnPoint;
