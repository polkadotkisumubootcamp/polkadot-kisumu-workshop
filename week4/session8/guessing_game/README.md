# Rust Guessing Game

A simple terminal-based number guessing game built with Rust, demonstrating safe error handling, crate usage, and terminal UI enhancements.

## Features

- Random number generation (`rand`)
- User input with `Result` and `Option` handling
- Styled terminal messages using `colored`
- Current time display with `chrono`
- Graceful handling of invalid or out-of-range input

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
    cd week4
    cd session8
    cd guessing_game
    cd src
     ```
5. Run the project
    ```bash
    cargo run
    ```
6. Expected results
    ```bash
    Please input your guess:
    45
    You guessed: 45
    Too small!

    Please input your guess:
    226
    Please enter a number between 1 and 100.

    Please input your guess:
    96
    You guessed: 96
    Too big!

    Please input your guess:
    50
    You guessed: 50
    Too small!

    Please input your guess:
    68
    You guessed: 68
    Too big!

    Please input your guess:
    60
    You guessed: 60
    Too small!

    Please input your guess:
    65
    You guessed: 65
    Too small!

    Please input your guess:
    67
    You guessed: 67
    You win!
    ```