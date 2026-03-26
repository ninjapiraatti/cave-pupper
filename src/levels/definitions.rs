use bevy::prelude::*;
use super::{LevelDef, platform, spike};

/// Classic arena - simple platforms
pub fn level_arena() -> LevelDef {
    LevelDef {
        name: "Arena",
        spawn_points: vec![
            Vec2::new(-100.0, 200.0),
            Vec2::new(0.0, 200.0),
            Vec2::new(100.0, 200.0),
        ],
        platforms: vec![
            // Ground
            platform(Vec2::new(0.0, -200.0), Vec2::new(800.0, 40.0)),
            // Left platform
            platform(Vec2::new(-250.0, -50.0), Vec2::new(200.0, 20.0)),
            // Right platform
            platform(Vec2::new(250.0, -50.0), Vec2::new(200.0, 20.0)),
            // Top platform
            platform(Vec2::new(0.0, 100.0), Vec2::new(150.0, 20.0)),
        ],
        hazards: vec![],
        death_zone_y: -450.0,
    }
}

/// Platforms level - more vertical, moving platforms
pub fn level_platforms() -> LevelDef {
    LevelDef {
        name: "Platforms",
        spawn_points: vec![
            Vec2::new(-150.0, 250.0),
            Vec2::new(150.0, 250.0),
        ],
        platforms: vec![
            // Small ground platforms on sides
            platform(Vec2::new(-300.0, -200.0), Vec2::new(200.0, 40.0)),
            platform(Vec2::new(300.0, -200.0), Vec2::new(200.0, 40.0)),
            // Middle gap with moving platform
            platform(Vec2::new(0.0, -200.0), Vec2::new(120.0, 20.0))
                .moving(vec![Vec2::new(-150.0, -200.0), Vec2::new(150.0, -200.0)], 100.0)
                .color(Color::srgb(0.3, 0.5, 0.3)),
            // Stepping stones going up
            platform(Vec2::new(-200.0, -80.0), Vec2::new(100.0, 15.0)),
            platform(Vec2::new(0.0, 0.0), Vec2::new(100.0, 15.0)),
            platform(Vec2::new(200.0, 80.0), Vec2::new(100.0, 15.0)),
            // Top platform
            platform(Vec2::new(0.0, 160.0), Vec2::new(200.0, 20.0)),
        ],
        hazards: vec![],
        death_zone_y: -450.0,
    }
}

/// Danger zone - lots of hazards and crumbling platforms
pub fn level_danger_zone() -> LevelDef {
    LevelDef {
        name: "Danger Zone",
        spawn_points: vec![
            Vec2::new(-200.0, 200.0),
            Vec2::new(200.0, 200.0),
        ],
        platforms: vec![
            // Narrow ground with spikes
            platform(Vec2::new(-250.0, -200.0), Vec2::new(150.0, 40.0)),
            platform(Vec2::new(250.0, -200.0), Vec2::new(150.0, 40.0)),
            // Crumbling bridges
            platform(Vec2::new(-80.0, -200.0), Vec2::new(80.0, 20.0))
                .crumbling(0.8, 0.3, 4.0)
                .color(Color::srgb(0.6, 0.4, 0.2)),
            platform(Vec2::new(80.0, -200.0), Vec2::new(80.0, 20.0))
                .crumbling(0.8, 0.3, 4.0)
                .color(Color::srgb(0.6, 0.4, 0.2)),
            // Upper platforms
            platform(Vec2::new(-150.0, -50.0), Vec2::new(120.0, 15.0)),
            platform(Vec2::new(150.0, -50.0), Vec2::new(120.0, 15.0)),
            // Moving platform at top
            platform(Vec2::new(0.0, 80.0), Vec2::new(100.0, 15.0))
                .moving(vec![Vec2::new(-180.0, 80.0), Vec2::new(180.0, 80.0)], 120.0)
                .color(Color::srgb(0.3, 0.5, 0.3)),
        ],
        hazards: vec![
            // Spikes in the middle gap
            spike(Vec2::new(-30.0, -165.0)),
            spike(Vec2::new(0.0, -165.0)),
            spike(Vec2::new(30.0, -165.0)),
        ],
        death_zone_y: -450.0,
    }
}
