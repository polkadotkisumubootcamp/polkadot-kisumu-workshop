# Web3 Professionals Modeling System

A Rust implementation demonstrating structs, traits, and lifetimes by modeling Web3 roles.

## Features

- **Developer** and **Designer** structs
- `Introduce` trait with role-specific implementations
- Lifetime annotations for reference handling
- Instance methods and constructors
- Unit tests included

## Usage

```rust
let dev = Developer::new("Kherld".to_string(), "Rust", 5);
dev.introduce();

let designer = Designer::new(
    "Nerd".to_string(),
    "NFT Art".to_string(),
    vec!["Figma".to_string()]
);
designer.introduce();
```

## Concepts Demonstrated

- Struct definition
- Trait implementation
- Lifetime annotations
- Method syntax
- Ownership/borrowing

Run with `cargo run`, test with `cargo test`
