mod components;
mod systems;

pub use components::*;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::state::GameState;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_arena)
            .add_systems(OnExit(GameState::Playing), cleanup_arena)
            .add_systems(
                Update,
                (
                    systems::move_movables,
                    systems::move_players_on_platforms.after(systems::move_movables),
                    systems::handle_deadly_contact,
                    systems::despawn_destroyed,
                    systems::init_crumbling,
                    systems::detect_crumbling_standing,
                    systems::update_crumbling,
                )
                    .run_if(in_state(GameState::Playing)),
            );
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

    // Spawn points - centered, above the highest platform (top platform is at y=100)
    let spawn_y = 200.0; // Well above the top platform
    commands.spawn((ArenaElement, SpawnPoint, Transform::from_xyz(-100.0, spawn_y, 0.0)));
    commands.spawn((ArenaElement, SpawnPoint, Transform::from_xyz(0.0, spawn_y, 0.0)));
    commands.spawn((ArenaElement, SpawnPoint, Transform::from_xyz(100.0, spawn_y, 0.0)));
    commands.spawn((ArenaElement, SpawnPoint, Transform::from_xyz(0.0, spawn_y, 0.0))); // Extra center spawn

    // Example: Deadly spike - triangle centered on origin
    commands.spawn((
        ArenaElement,
        Deadly,
        RigidBody::Static,
        Collider::triangle(Vec2::new(-20.0, -15.0), Vec2::new(20.0, -15.0), Vec2::new(0.0, 15.0)),
        Transform::from_xyz(0.0, -165.0, 0.0), // Adjusted so spike sits on ground
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(40.0, 30.0)),
            ..default()
        },
    ));

    // Example: Moving platform
    commands.spawn((
        ArenaElement,
        Platform,
        Movable {
            waypoints: vec![Vec2::new(-350.0, 50.0), Vec2::new(350.0, 50.0)],
            speed: 80.0,
            current_index: 0,
            forward: true,
            current_velocity: Vec2::ZERO,
        },
        RigidBody::Kinematic,
        Collider::rectangle(100.0, 15.0),
        Transform::from_xyz(-350.0, 50.0, 0.0),
        Sprite {
            color: Color::srgb(0.3, 0.5, 0.3),
            custom_size: Some(Vec2::new(100.0, 15.0)),
            ..default()
        },
    ));

    // Example: Crumbling platform
    commands.spawn((
        ArenaElement,
        Platform,
        Crumbling {
            stand_time: 1.0,   // Must stand for 1 second
            delay: 0.3,        // Then 0.3s warning before crumble
            respawn_time: 3.0,
        },
        RigidBody::Static,
        Collider::rectangle(80.0, 15.0),
        Transform::from_xyz(0.0, -80.0, 0.0),
        Sprite {
            color: Color::srgb(0.6, 0.4, 0.2),
            custom_size: Some(Vec2::new(80.0, 15.0)),
            ..default()
        },
    ));
}

fn cleanup_arena(mut commands: Commands, query: Query<Entity, With<ArenaElement>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
