# Web3 Profile System in Rust

This project implements a simple Web3 profile system in Rust, demonstrating structs, traits, and lifetimes.

## Features

- **Developer Struct**: Represents a Web3 developer with `name`, `favorite_lang`, and `years_experience`.
- **Designer Struct**: Represents a Web3 designer with `name`, `favorite_tool`, and `years_experience`.
- **Introduce Trait**: Defines an `introduce` method for both `Developer` and `Designer` to print a custom introduction.
- **Lifetime Demonstration**: Includes a function `get_most_experienced` that returns a reference with a lifetime annotation.

## How to Run

1. Navigate to the `web3_profile` directory:

   ```bash
   cd week4/web3_profile
   ```

2. Run the application using Cargo:

   ```bash
   cargo run
   ```

This will compile and execute the program, displaying the introductions for a sample developer and designer, and demonstrating the lifetime function.