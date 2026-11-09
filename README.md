# How to Play the Game

- Download the latest binary for your system from Releases section
- Run the binary

# Compiling & Running

## System Requirements

- Rust
- Cargo

## Compile and run the program for debugging

```rust
git clone git@github.com:rohith-vp/stone-paper-scissor-rust.git
cd stone-paper-scissor-rust

cargo build
// On UNIX-based systems
./target/debug/stone_paper_scissor
// On Windows
target\debug\stone_paper_scissor

// OR

cargo run
```

## Compile and run the program for production

```rust
git clone git@github.com:rohith-vp/stone-paper-scissor-rust.git
cd stone-paper-scissor-rust/

cargo build --release
// On UNIX-based systems
./target/release/stone_paper_scissor
// On Windows
target\release\stone_paper_scissor
```
