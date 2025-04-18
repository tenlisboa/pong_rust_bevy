# Project 1 – Pong on Steroids (Rust + Bevy)

> First project of the study plan focused on learning the fundamentals of Rust and Bevy with a simple 2D game, using ECS, user input, movement, and game logic.

## Menu Screen

![Picture of the menu screen](docs/menu.png)

## On Game Screen

![Picture of the game running](docs/on_game.png)

---

## Objective

- Learn Rust in practice, understanding ownership, borrow checker, modules, and enums.
- Master the basics of the Bevy Engine and its ECS architecture.
- Create a functional and modular game with a clean and extensible structure.

---

## Key Concepts

- Bevy App lifecycle
- ECS (Entities, Components, Systems)
- Transform and Sprite
- Keyboard input
- Simple collisions
- Scoring system
- HUD with text

---

## Suggested Project Structure

```txt
pong_rust_bevy/
├── Cargo.toml
├── src/
│ ├── main.rs
│ ├── game.rs
│ ├── components.rs
│ ├── systems/
│ │ ├── input.rs
│ │ ├── movement.rs
│ │ ├── collision.rs
│ │ └── scoring.rs
│ └── config.rs
```

---

## Basic Features

| Feature                | Status |
| ---------------------- | ------ |
| Window with 2D camera  | [x]    |
| Player paddle          | [x]    |
| Ball with movement     | [x]    |
| Movement input (J/K)   | [x]    |
| Ball/paddle collision  | [x]    |
| Wall collision         | [x]    |
| Scoring system         | [ ]    |
| UI with scoreboard     | [ ]    |
| Match restart          | [ ]    |
| Sounds (bounce, point) | [ ]    |

---

## Milestones

### Milestone 1 – Initial Setup

- [x] Create project with `cargo new`
- [x] Add Bevy to `Cargo.toml`
- [x] Window + 2D camera working
- [x] Paddle sprite visible on screen

### Milestone 2 – Movement and Collision

- [x] Input system working (J/K)
- [x] Paddle movement
- [x] Ball moving automatically
- [x] Collision with paddle and edges

### Milestone 3 – Scoring and HUD

- [x] Track points (when ball passes paddle)
- [x] Display score on screen (HUD with text)

### Milestone 4 – Polish (optional)

- [x] Basic sounds using
- [x] Splash screen / simple start menu

---

## Useful Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/)
- [Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)

---

## Tips

- Start simple: one paddle, one ball.
- Test each system separately (input, physics, collision).
- Use `println!()` for debugging early on.
- Modularize systems early — it helps scale the project later.

---

## License

MIT
