# Cave Pupper - Development Plan

## Vision
Local multiplayer brawl platformer. Share one keyboard, 2 keys per player. Characters have wildly different abilities. Hot-join, respawn on death.

---

## Current State (Milestone 1 - Partial)

### Done
- [x] 2-key input system (8 slots)
- [x] Physics with avian2d
- [x] Arena with platforms and death zone
- [x] Player join/respawn lifecycle
- [x] Basic movement character (left/right)
- [x] Modular action system

### Remaining for Milestone 1
- [ ] Add Jump action (works, just not default)
- [ ] Test with 2+ players
- [ ] Add more spawn points if needed

---

## Architecture

### Action System (Current)
```rust
pub enum Action {
    MoveLeft,
    MoveRight,
    Jump,
}

pub struct Character {
    pub action_a: Action,
    pub action_b: Action,
    pub move_speed: f32,
    pub jump_force: f32,
}
```

Each character binds two actions to the two keys. The `execute_actions` system runs every frame and applies the appropriate behavior.

### Adding New Actions
1. Add variant to `Action` enum
2. Implement in `execute_action()` match
3. Create character using new action

---

## Future Milestones

### Milestone 2: Combat
- [ ] Projectiles (simple bullet)
- [ ] Damage/Health display
- [ ] Death from damage
- [ ] Knockback

### Milestone 3: Character Variety
- [ ] Multiple character types
- [ ] Character selection (random or menu)
- [ ] Unique abilities (dash, block, teleport)

### Milestone 4: Polish
- [ ] Sprite graphics
- [ ] Sound effects
- [ ] Options menu (key rebinding)
- [ ] Multiple arenas

---

## Character Ideas

| Name | Key A | Key B |
|------|-------|-------|
| Mover | Move Left | Move Right |
| Jumper | Jump | Move Right |
| Dasher | Dash | Turn |
| Gunner | Jump | Shoot |
| Bomber | Move Left | Drop Bomb |
| Tank | Block | Charge |

---

## Technical Notes

### Bevy 0.18
- Using avian2d 0.6 for physics
- `Collisions` resource for collision queries
- `CollidingEntities` component for per-entity collisions

### Physics Setup
- Gravity: 980 units/sec^2 down
- Length unit: 100 (so 1 meter = 100 pixels)
- Players: Dynamic rigid bodies, locked rotation

### Player Lifecycle
```
Empty → (key press) → WaitingToSpawn → (key press) → Alive
                                              ↓ (death)
                                             Dead → (key press) → Alive
```
