# How to Play the Game

- Download the latest binary for your system from Releases section
- Run the binary

On Linux-based systems:
```bash
wget https://github.com/rohith-vp/stone-paper-scissor-rust/releases/download/v0.1/stone_paper_scissor_linux_x86
chmod +x ./stone_paper_scissor_linux_x86
./stone_paper_scissor_linux_x86
```

# Compiling & Running

## System Requirements

- `Rust`
- `Cargo`

## Compile and run the program for debugging

```bash
git clone git@github.com:rohith-vp/stone-paper-scissor-rust.git
cd stone-paper-scissor-rust

cargo build
# On UNIX-based systems
./target/debug/stone_paper_scissor
# On Windows
target\debug\stone_paper_scissor

# OR

cargo run
```

## Compile and run the program for production

```bash
git clone git@github.com:rohith-vp/stone-paper-scissor-rust.git
cd stone-paper-scissor-rust/

cargo build --release
# On UNIX-based systems
./target/release/stone_paper_scissor
# On Windows
target\release\stone_paper_scissor
```
