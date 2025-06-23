 # Web3 Professionals System

A Rust implementation modeling Web3 professionals (developers and designers) with trait-based introductions.

## Features
- `Developer` struct with name, language, and experience
- `Designer` struct with name, tool, and experience
- `Introduce` trait with default implementation
- Lifetime-annotated factory function
- Enum-based type handling

## Usage
```rust
let dev = get_web3_professional("developer", "Alice", "Rust", 3);
dev.introduce();

Requirements

    Rust 1.60+

    Cargo

## Instalaation

git clone [your-repo-url]
cd polkadot-kisumu-workshop/odinga-val/week3/session6
cargo run

## Project Structure

src/
  main.rs     # Main implementation
Cargo.toml    # Dependency management
README.md     # This file

## License