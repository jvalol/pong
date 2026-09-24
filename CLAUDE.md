# pong

Two paddles, a ball, five points to win. The first game built on `blitzkit`,
which owns the window, rendering, input, and sound. The dependency is the
published crate, overridden by the engine checkout at `../blitzkit` when built
inside this project folder.

## Build and test

Requires Rust 1.87 or newer, the engine's MSRV.

```
cargo build
cargo test
cargo run
cargo clippy
cargo fmt
```

## How work happens here

Behavior changes are spec driven:

1. **Write the spec first.** Copy `specs/TEMPLATE.md` to `specs/NNNN-short-name.md`
   and fill it in. Say what the game does, not how the code does it.
2. **Make the acceptance criteria testable.** Each one names the test that proves
   it, or goes under "Verified by hand" when it needs a window or a speaker.
3. **Write the tests, then the code.** `cargo test` passes before a commit.
4. **Update the spec when behavior changes.** A spec that disagrees with the game
   is a bug in the spec.

## Layout

- `src/main.rs` — hands a `PongGame` to `blitzkit::start`.
- `src/pong_game.rs` — the `Game` impl, the event list, sound.
- `src/state.rs` — everything the game knows, and `layout()` which sizes it all.
- `src/system.rs` — one system per game state, each stepping the state.
- `src/input.rs` — engine key events to held flags.
- `src/player.rs`, `src/ball.rs` — the moving pieces.
- `src/util.rs` — speeds and the bounce angle.

## Conventions

- **Everything is in pixels** with the origin at the top-left and y down, the space
  the engine draws in. `State::field` is the window size, and everything is placed
  relative to it in `State::layout`.
- **Speeds are per second**, as fractions of the window, multiplied by
  `State::delta_time`.
- **Systems are pure game logic.** They take input, state, and events, and touch no
  GPU, window, or audio. That is what makes them testable, so keep it that way.
- Tests live next to the code in `#[cfg(test)] mod tests`.
