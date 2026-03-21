use bevy::prelude::*;

// Maximum number of players supported
const MAX_PLAYERS: usize = 4;

// Gamepad stick deadzone (ignore values below this threshold)
const STICK_DEADZONE: f32 = 0.15;

// Player colors for each player slot
const PLAYER_COLORS: [Color; MAX_PLAYERS] = [
    Color::srgb(0.2, 0.6, 1.0),  // Blue - Player 1
    Color::srgb(1.0, 0.3, 0.3),  // Red - Player 2
    Color::srgb(0.3, 1.0, 0.3),  // Green - Player 3
    Color::srgb(1.0, 1.0, 0.3),  // Yellow - Player 4
];

// Keyboard bindings for each player
const PLAYER_KEYS: [(KeyCode, KeyCode, KeyCode, KeyCode, KeyCode); MAX_PLAYERS] = [
    // Player 1: WASD + Space
    (KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD, KeyCode::Space),
    // Player 2: Arrow keys + Enter
    (KeyCode::ArrowUp, KeyCode::ArrowDown, KeyCode::ArrowLeft, KeyCode::ArrowRight, KeyCode::Enter),
    // Player 3: IJKL + U
    (KeyCode::KeyI, KeyCode::KeyK, KeyCode::KeyJ, KeyCode::KeyL, KeyCode::KeyU),
    // Player 4: Numpad 8456 + 0
    (KeyCode::Numpad8, KeyCode::Numpad5, KeyCode::Numpad4, KeyCode::Numpad6, KeyCode::Numpad0),
];

// Game states
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
}

// Component to mark a player entity
#[derive(Component)]
struct Player {
    id: usize,
    input_source: InputSource,
}

// Input source for a player
#[derive(Clone, Copy, Debug, PartialEq)]
enum InputSource {
    Keyboard(usize), // Index into PLAYER_KEYS
    Gamepad(Entity), // Gamepad entity
}

// Resource to track which input sources are taken
#[derive(Resource, Default)]
struct PlayerRegistry {
    players: Vec<(Entity, InputSource)>,
}

impl PlayerRegistry {
    fn player_count(&self) -> usize {
        self.players.len()
    }

    fn is_input_taken(&self, source: &InputSource) -> bool {
        self.players.iter().any(|(_, s)| s == source)
    }

    fn add_player(&mut self, entity: Entity, source: InputSource) {
        self.players.push((entity, source));
    }

    fn get_player_id(&self, entity: Entity) -> Option<usize> {
        self.players.iter().position(|(e, _)| *e == entity)
    }
}

// Marker for menu UI
#[derive(Component)]
struct MenuUI;

// Marker for game UI
#[derive(Component)]
struct GameUI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Emo Nemo - Couch Game".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .init_resource::<PlayerRegistry>()
        .add_systems(Startup, setup)
        // Menu systems
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
        // Game systems
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(OnExit(GameState::Playing), cleanup_game)
        .add_systems(
            Update,
            (
                player_join,
                player_movement,
                back_to_menu,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

// ============ MENU SYSTEMS ============

fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            MenuUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("EMO NEMO"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Subtitle
            parent.spawn((
                Text::new("Couch Game Template"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            // Spacer
            parent.spawn(Node {
                height: Val::Px(40.0),
                ..default()
            });

            // Controls info
            parent.spawn((
                Text::new("CONTROLS"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.3)),
            ));

            // Player 1
            parent.spawn((
                Text::new("Player 1: WASD + Space"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(PLAYER_COLORS[0]),
            ));

            // Player 2
            parent.spawn((
                Text::new("Player 2: Arrow Keys + Enter"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(PLAYER_COLORS[1]),
            ));

            // Player 3
            parent.spawn((
                Text::new("Player 3: IJKL + U"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(PLAYER_COLORS[2]),
            ));

            // Player 4
            parent.spawn((
                Text::new("Player 4: Numpad 8456 + 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(PLAYER_COLORS[3]),
            ));

            // Gamepad info
            parent.spawn((
                Text::new("Gamepads: Any button to join, Left stick to move"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            // Spacer
            parent.spawn(Node {
                height: Val::Px(40.0),
                ..default()
            });

            // Start instruction
            parent.spawn((
                Text::new("Press SPACE or any gamepad button to start"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Check keyboard
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
        return;
    }

    // Check gamepads
    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::South)
            || gamepad.just_pressed(GamepadButton::Start)
        {
            next_state.set(GameState::Playing);
            return;
        }
    }
}

// ============ GAME SYSTEMS ============

fn setup_game(mut commands: Commands, mut registry: ResMut<PlayerRegistry>) {
    // Clear player registry
    registry.players.clear();

    // Spawn game UI
    commands
        .spawn((
            GameUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Instructions at top
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Press your action key to join! (ESC to return to menu)"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

fn cleanup_game(
    mut commands: Commands,
    game_ui: Query<Entity, With<GameUI>>,
    players: Query<Entity, With<Player>>,
) {
    for entity in &game_ui {
        commands.entity(entity).despawn();
    }
    for entity in &players {
        commands.entity(entity).despawn();
    }
}

fn player_join(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
    mut registry: ResMut<PlayerRegistry>,
) {
    if registry.player_count() >= MAX_PLAYERS {
        return;
    }

    // Check keyboard joins
    for (key_index, keys) in PLAYER_KEYS.iter().enumerate() {
        let source = InputSource::Keyboard(key_index);
        if registry.is_input_taken(&source) {
            continue;
        }

        // Check if action key (5th key in tuple) is pressed
        if keyboard.just_pressed(keys.4) {
            spawn_player(&mut commands, &mut meshes, &mut materials, &mut registry, source);
        }
    }

    // Check gamepad joins
    for (gamepad_entity, gamepad) in &gamepads {
        let source = InputSource::Gamepad(gamepad_entity);
        if registry.is_input_taken(&source) {
            continue;
        }

        if gamepad.just_pressed(GamepadButton::South)
            || gamepad.just_pressed(GamepadButton::East)
            || gamepad.just_pressed(GamepadButton::West)
            || gamepad.just_pressed(GamepadButton::North)
            || gamepad.just_pressed(GamepadButton::Start)
        {
            spawn_player(&mut commands, &mut meshes, &mut materials, &mut registry, source);
        }
    }
}

fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    registry: &mut ResMut<PlayerRegistry>,
    source: InputSource,
) {
    let player_id = registry.player_count();
    let color = PLAYER_COLORS[player_id];

    // Calculate spawn position (spread players out)
    let angle = (player_id as f32) * std::f32::consts::TAU / 4.0;
    let spawn_distance = 150.0;
    let spawn_pos = Vec3::new(
        angle.cos() * spawn_distance,
        angle.sin() * spawn_distance,
        0.0,
    );

    let entity = commands
        .spawn((
            Player {
                id: player_id,
                input_source: source,
            },
            Mesh2d(meshes.add(Circle::new(30.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
            Transform::from_translation(spawn_pos),
        ))
        .id();

    registry.add_player(entity, source);
    info!("Player {} joined with {:?}", player_id + 1, source);
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut players: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    let speed = 300.0;

    for (player, mut transform) in &mut players {
        let mut direction = Vec2::ZERO;

        match player.input_source {
            InputSource::Keyboard(key_index) => {
                let keys = PLAYER_KEYS[key_index];
                if keyboard.pressed(keys.0) {
                    direction.y += 1.0;
                }
                if keyboard.pressed(keys.1) {
                    direction.y -= 1.0;
                }
                if keyboard.pressed(keys.2) {
                    direction.x -= 1.0;
                }
                if keyboard.pressed(keys.3) {
                    direction.x += 1.0;
                }
            }
            InputSource::Gamepad(gamepad_entity) => {
                if let Ok(gamepad) = gamepads.get(gamepad_entity) {
                    let left_stick = gamepad.left_stick();

                    // Apply deadzone to prevent drift
                    if left_stick.length() > STICK_DEADZONE {
                        direction.x = left_stick.x;
                        direction.y = left_stick.y;
                    }

                    // Also support D-pad
                    if gamepad.pressed(GamepadButton::DPadUp) {
                        direction.y += 1.0;
                    }
                    if gamepad.pressed(GamepadButton::DPadDown) {
                        direction.y -= 1.0;
                    }
                    if gamepad.pressed(GamepadButton::DPadLeft) {
                        direction.x -= 1.0;
                    }
                    if gamepad.pressed(GamepadButton::DPadRight) {
                        direction.x += 1.0;
                    }
                }
            }
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            transform.translation.x += direction.x * speed * time.delta_secs();
            transform.translation.y += direction.y * speed * time.delta_secs();
        }
    }
}

fn back_to_menu(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
