mod components;

pub use components::*;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::state::GameState;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_arena)
            .add_systems(OnExit(GameState::Playing), cleanup_arena);
    }
}

fn spawn_arena(mut commands: Commands) {
    // Ground platform
    commands.spawn((
        ArenaElement,
        Platform,
        RigidBody::Static,
        Collider::rectangle(800.0, 40.0),
        Transform::from_xyz(0.0, -200.0, 0.0),
        Sprite {
            color: Color::srgb(0.4, 0.4, 0.4),
            custom_size: Some(Vec2::new(800.0, 40.0)),
            ..default()
        },
    ));

    // Left platform
    commands.spawn((
        ArenaElement,
        Platform,
        RigidBody::Static,
        Collider::rectangle(200.0, 20.0),
        Transform::from_xyz(-250.0, -50.0, 0.0),
        Sprite {
            color: Color::srgb(0.4, 0.4, 0.4),
            custom_size: Some(Vec2::new(200.0, 20.0)),
            ..default()
        },
    ));

    // Right platform
    commands.spawn((
        ArenaElement,
        Platform,
        RigidBody::Static,
        Collider::rectangle(200.0, 20.0),
        Transform::from_xyz(250.0, -50.0, 0.0),
        Sprite {
            color: Color::srgb(0.4, 0.4, 0.4),
            custom_size: Some(Vec2::new(200.0, 20.0)),
            ..default()
        },
    ));

    // Top platform
    commands.spawn((
        ArenaElement,
        Platform,
        RigidBody::Static,
        Collider::rectangle(150.0, 20.0),
        Transform::from_xyz(0.0, 100.0, 0.0),
        Sprite {
            color: Color::srgb(0.4, 0.4, 0.4),
            custom_size: Some(Vec2::new(150.0, 20.0)),
            ..default()
        },
    ));

    // Death zone (bottom of screen)
    commands.spawn((
        ArenaElement,
        DeathZone,
        Collider::rectangle(2000.0, 100.0),
        Transform::from_xyz(0.0, -450.0, 0.0),
        Sensor,
    ));

    // Spawn points
    commands.spawn((ArenaElement, SpawnPoint, Transform::from_xyz(-300.0, -100.0, 0.0)));
    commands.spawn((ArenaElement, SpawnPoint, Transform::from_xyz(-100.0, -100.0, 0.0)));
    commands.spawn((ArenaElement, SpawnPoint, Transform::from_xyz(100.0, -100.0, 0.0)));
    commands.spawn((ArenaElement, SpawnPoint, Transform::from_xyz(300.0, -100.0, 0.0)));
}

fn cleanup_arena(mut commands: Commands, query: Query<Entity, With<ArenaElement>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
