# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust implementation of the board game Splendor. The project uses Rust edition 2025 and is in early development stages.

## Common Commands

### Build and Run
```bash
cargo build          # Build the project
cargo run           # Run the main binary
cargo build --release  # Build optimized release binary
```

### Testing
```bash
cargo test          # Run all tests
cargo test <test_name>  # Run a specific test
cargo test -- --nocapture  # Run tests with stdout visible
```

### Code Quality
```bash
cargo check         # Fast check without producing binary
cargo clippy        # Run linter
cargo fmt           # Format code
cargo fmt -- --check  # Check formatting without changing files
```

## Code Architecture

### Module Structure

The codebase follows a domain-driven structure organized around game components:

- **src/models/**: Contains all game domain models
  - `gem.rs`: Defines the `Gem` enum representing the six gem types (Onyx, Sapphire, Emerald, Ruby, Diamond, Gold)
  - `card.rs`: Defines the `Card<Gem>` struct with generic typing for game cards, including id, level (1-3), points, costs (HashMap), and bonus gem
  - `game_state.rs`: Contains `GameState` struct managing the complete game state including board cards, nobles, available tokens, player's hand, and player's resources
  - `models.rs`: Module aggregator for all model submodules

### Key Design Patterns

- **Generic Types**: The `Card` struct uses generic typing (`Card<Gem>`) to maintain type flexibility
- **HashMap-based Resources**: Both card costs and player resources use `HashMap<Token, u8>` or `HashMap<Gem, u8>` for flexible resource tracking
- **Game State Centralization**: All game state is managed in a single `GameState` struct, including both shared state (board, nobles, tokens) and player-specific state (hand, resources)

### Important Notes

- The `GameState` struct currently appears to be single-player focused (uses `my_hand` and `my_resources`)
- There's a type inconsistency: `GameState` uses `Token` type while `Card` uses `Gem` generic - these may need alignment
- The `Noble` type is referenced in `GameState` but not yet defined in the codebase
