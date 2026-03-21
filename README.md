# emo-nemo

Bevy template for simple couch games. Fork this project to quickly start building local multiplayer games.

## Features

- Main menu with control instructions for all players
- Support for 1-4 players
- Keyboard controls for 4 players simultaneously
- Gamepad/controller support (PS5, Xbox, etc.)
- Hot-join: players can join mid-game by pressing their action button
- Clean state management (Menu / Playing)

## Controls

| Player | Move | Action |
|--------|------|--------|
| Player 1 | WASD | Space |
| Player 2 | Arrow Keys | Enter |
| Player 3 | IJKL | U |
| Player 4 | Numpad 8456 | Numpad 0 |
| Gamepad | Left Stick / D-Pad | Any face button |

Press ESC to return to menu during gameplay.

## Running

```bash
cargo run
```

## Building for Release

```bash
cargo build --release
```

## Structure

The template is intentionally minimal to make it easy to extend:

- `src/main.rs` - All game logic in one file
  - `GameState` enum for state management
  - `Player` component with input source tracking
  - `PlayerRegistry` resource to track active players
  - Keyboard and gamepad input handling
  - Basic circle rendering for players

## Extending

To build your game on this template:

1. Fork this repository
2. Add your game logic in the `Playing` state systems
3. Add new components for game objects
4. Modify player appearance (replace circles with sprites)
5. Add game mechanics, scoring, etc.

## Requirements

- Rust 2024 edition
- Bevy 0.18.1
