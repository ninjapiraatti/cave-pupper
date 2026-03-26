mod definitions;

pub use definitions::*;

use bevy::prelude::*;

/// Platform definition
#[derive(Clone, Debug)]
pub struct PlatformDef {
    pub pos: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub movable: Option<MovableDef>,
    pub crumbling: Option<CrumblingDef>,
}

impl Default for PlatformDef {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            size: Vec2::new(100.0, 20.0),
            color: Color::srgb(0.4, 0.4, 0.4),
            movable: None,
            crumbling: None,
        }
    }
}

impl PlatformDef {
    pub fn moving(mut self, waypoints: Vec<Vec2>, speed: f32) -> Self {
        self.movable = Some(MovableDef { waypoints, speed });
        self
    }

    pub fn crumbling(mut self, stand_time: f32, delay: f32, respawn_time: f32) -> Self {
        self.crumbling = Some(CrumblingDef { stand_time, delay, respawn_time });
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

#[derive(Clone, Debug)]
pub struct MovableDef {
    pub waypoints: Vec<Vec2>,
    pub speed: f32,
}

#[derive(Clone, Debug)]
pub struct CrumblingDef {
    pub stand_time: f32,
    pub delay: f32,
    pub respawn_time: f32,
}

/// Hazard types
#[derive(Clone, Debug)]
pub enum HazardKind {
    Spike,
}

#[derive(Clone, Debug)]
pub struct HazardDef {
    pub pos: Vec2,
    pub kind: HazardKind,
}

/// Complete level definition
#[derive(Clone, Debug)]
pub struct LevelDef {
    pub name: &'static str,
    pub spawn_points: Vec<Vec2>,
    pub platforms: Vec<PlatformDef>,
    pub hazards: Vec<HazardDef>,
    pub death_zone_y: f32,
}

// Helper functions for ergonomic level building

pub fn platform(pos: Vec2, size: Vec2) -> PlatformDef {
    PlatformDef {
        pos,
        size,
        ..default()
    }
}

pub fn spike(pos: Vec2) -> HazardDef {
    HazardDef {
        pos,
        kind: HazardKind::Spike,
    }
}

/// Get all available levels
pub fn all_levels() -> Vec<LevelDef> {
    vec![
        level_arena(),
        level_platforms(),
        level_danger_zone(),
    ]
}
