# Cave Pupper

Local multiplayer brawl platformer. Players share one keyboard, 2 keys each. Characters can have different abilities per key.

## Quick Start
```bash
cargo run              # Run the game
cargo run --release    # Run optimized
cargo check            # Fast compile check
```

## How to Play
1. Press SPACE to start from the menu
2. Each player presses their keys to join (first press claims slot, second spawns)
3. Move left/right with your two keys
4. Fall off the bottom = death, press key to respawn

## Controls (Default)
- P1: Q/W
- P2: E/R
- P3: T/Y
- P4: U/I
- P5: O/P
- P6: A/S
- P7: D/F
- P8: G/H

ESC returns to menu.

## Architecture

### Core Concepts
- **PlayerSlot** (0-7): Input slot that persists across deaths
- **Character**: Defines what actions each key triggers
- **Action**: A single behavior (MoveLeft, MoveRight, Jump, etc.)

### Module Layout
```
src/
  input/       - 2-key input system, bindings per slot
  characters/  - Character definitions and action system
  arena/       - Platforms, death zones, spawn points
  combat/      - Health (not fully used yet)
  player/      - Player lifecycle (join/die/respawn)
  game/        - Game state systems
  menu/        - Menu UI
```

### Key Types
- `Player` - Component with slot ID
- `Character` - Component with action_a, action_b, stats
- `Action` - Enum (MoveLeft, MoveRight, Jump, ...)
- `PlayerSlots` - Resource tracking slot states
- `SlotState` - Empty, WaitingToSpawn, Alive(Entity), Dead

## Adding New Characters

Edit `src/characters/actions.rs`:

1. Add new actions to the `Action` enum if needed:
```rust
pub enum Action {
    MoveLeft,
    MoveRight,
    Jump,
    Dash,     // new
    Shoot,    // new
}
```

2. Implement the action in `execute_action()`:
```rust
Action::Dash => {
    if just_pressed {
        velocity.x = character.move_speed * 3.0 * facing_dir;
    }
}
```

3. Create a new character constructor:
```rust
pub fn dasher() -> Self {
    Self {
        name: "Dasher".to_string(),
        action_a: Action::Dash,
        action_b: Action::MoveRight,
        move_speed: 300.0,
        jump_force: 400.0,
    }
}
```

4. Use it in `player/systems.rs` `spawn_player()`:
```rust
Character::dasher(),
```

## Physics
Using avian2d for 2D physics:
- `RigidBody::Dynamic` - players
- `RigidBody::Static` - platforms
- `Collider` - hitboxes
- `LinearVelocity` - movement
- `Collisions` - query collisions

## Files

| File | Purpose |
|------|---------|
| `main.rs` | App setup, plugins |
| `state.rs` | GameState enum |
| `input/bindings.rs` | Key bindings, PlayerInputs |
| `characters/actions.rs` | Action enum, Character struct |
| `arena/mod.rs` | Platform/death zone spawning |
| `player/systems.rs` | Join/respawn/death logic |

## Dependencies
- `bevy` 0.18 - Game engine
- `avian2d` 0.6 - 2D physics
