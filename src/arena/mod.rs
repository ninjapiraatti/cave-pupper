mod components;
mod systems;

pub use components::*;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::levels::{all_levels, HazardKind, LevelDef};
use crate::state::GameState;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>()
            .add_systems(OnEnter(GameState::Playing), spawn_arena)
            .add_systems(OnExit(GameState::Playing), (cleanup_arena, reset_level))
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

/// Tracks the current level index
#[derive(Resource, Default)]
pub struct CurrentLevel {
    pub index: usize,
}

impl CurrentLevel {
    pub fn get_level(&self) -> LevelDef {
        let levels = all_levels();
        levels[self.index % levels.len()].clone()
    }

    pub fn next(&mut self) {
        self.index += 1;
    }
}

fn spawn_arena(mut commands: Commands, current_level: Res<CurrentLevel>) {
    let level = current_level.get_level();
    spawn_level(&mut commands, &level);
}

pub fn spawn_level(commands: &mut Commands, level: &LevelDef) {
    info!("Spawning level: {}", level.name);

    // Spawn platforms
    for platform_def in &level.platforms {
        let mut entity = commands.spawn((
            ArenaElement,
            Platform,
            Collider::rectangle(platform_def.size.x, platform_def.size.y),
            Transform::from_xyz(platform_def.pos.x, platform_def.pos.y, 0.0),
            Sprite {
                color: platform_def.color,
                custom_size: Some(platform_def.size),
                ..default()
            },
        ));

        // Add movable if defined
        if let Some(ref movable) = platform_def.movable {
            entity.insert((
                RigidBody::Kinematic,
                Movable {
                    waypoints: movable.waypoints.clone(),
                    speed: movable.speed,
                    current_index: 0,
                    forward: true,
                    current_velocity: Vec2::ZERO,
                },
            ));
        } else {
            entity.insert(RigidBody::Static);
        }

        // Add crumbling if defined
        if let Some(ref crumbling) = platform_def.crumbling {
            entity.insert(Crumbling {
                stand_time: crumbling.stand_time,
                delay: crumbling.delay,
                respawn_time: crumbling.respawn_time,
            });
        }
    }

    // Spawn hazards
    for hazard in &level.hazards {
        match hazard.kind {
            HazardKind::Spike => {
                commands.spawn((
                    ArenaElement,
                    Deadly,
                    RigidBody::Static,
                    Collider::triangle(
                        Vec2::new(-20.0, -15.0),
                        Vec2::new(20.0, -15.0),
                        Vec2::new(0.0, 15.0),
                    ),
                    Transform::from_xyz(hazard.pos.x, hazard.pos.y, 0.0),
                    Sprite {
                        color: Color::srgb(0.8, 0.2, 0.2),
                        custom_size: Some(Vec2::new(40.0, 30.0)),
                        ..default()
                    },
                ));
            }
        }
    }

    // Spawn death zone
    commands.spawn((
        ArenaElement,
        DeathZone,
        Collider::rectangle(2000.0, 100.0),
        Transform::from_xyz(0.0, level.death_zone_y, 0.0),
        Sensor,
    ));

    // Spawn points
    for spawn_pos in &level.spawn_points {
        commands.spawn((
            ArenaElement,
            SpawnPoint,
            Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.0),
        ));
    }
}

pub fn cleanup_arena(mut commands: Commands, query: Query<Entity, With<ArenaElement>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn reset_level(mut current_level: ResMut<CurrentLevel>) {
    current_level.index = 0;
}
