# Web3 Profile System

This project is a simple Rust program modeling a Web3 professional profile system. It includes:

## Features

- **Structs** for `Developer` and `Designer`
- **Trait** called `Introduce`
- **Trait Implementation** for both structs
- **Method Implementation** via `impl` blocks
- **Lifetime Annotations** for referencing data correctly

## Structs

- **Developer**
  - `name`: &str
  - `favorite_lang`: &str
  - `years_experience`: u8

- **Designer**
  - `name`: &str
  - `design_tool`: &str
  - `years_experience`: u8

## Trait: `Introduce`

Implemented by both `Developer` and `Designer`, allowing them to print a custom introduction.

## Lifetime Example

The `favorite_language()` method in `Developer` uses Rust's lifetime annotations to return a reference.

## Running the Project

1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone this repository:
   ```bash
   git clone https://github.com/polkadotkisumubootcamp/polkadot-kisumu-workshop.git
   
   ```
3. Switch to branch cynthia
    ```bash
    git checkout cynthia
    ```
4. Navigate to the project
     ```bash
    cd week3
    cd web3_profile
    cd src
     ```
5. Run the project
    ```bash
    cargo run
    ```
6. Expected results
    ```bash

    Hi, I'm Cynthia. I love writing Rust and have 3 years of experience in Web3.
    Favorite Language: Rust
    Hi, I'm Brian. I craft Web3 experiences using Figma with 4 years of experience.
    ```