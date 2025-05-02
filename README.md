# Guessing Game

This is a simple number guessing game written in Rust. It’s a beginner-friendly project designed to demonstrate basic concepts like user input, control flow, random number generation, and pattern matching in Rust.

## How It Works

- The computer randomly selects a number between 1 and 100.
- The player is prompted to guess the number.
- After each guess, the program will tell the player whether the guess was too low, too high, or correct.
- The game continues until the correct number is guessed.

## Getting Started

### Prerequisites

- Rust (Install via [rustup.rs](https://rustup.rs))

### Build & Run

```bash
git clone https://github.com/your-username/guessing_game.git
cd guessing_game
cargo run
```

### Dependencies
rand - For random number generation

- Add this to your Cargo.toml:
