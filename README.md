# Tetris (Rust)

A simple Tetris clone written in Rust using [raylib-rs](https://github.com/raylib-rs/raylib-rs).

## Controls

| Key   | Action                  |
|-------|-------------------------|
| ← / → | Move                    |
| Space | Rotate                  |
| ↓     | Soft drop               |
| Enter | Restart after game over |

## Build & Run

Requirements:
- Rust (stable)
- CMake and LLVM/Clang (needed to build raylib)

```
cargo run --release
```

On Windows, if the build can't find CMake or libclang, set the `CMAKE` and `LIBCLANG_PATH`
environment variables (see `.cargo/config.toml` for an example).
