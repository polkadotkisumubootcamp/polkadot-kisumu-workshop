# Rust Calculator

A simple command-line calculator written in Rust.

## Features

- Accepts **two `f32` numbers** from the user.
- Supports basic operations: `+`, `-`, `*`, `/`.
- **Handles division by zero** gracefully.
- Uses **variable shadowing** to reuse input variables.
- Displays a **constant welcome banner**.
- Adds color to the CLI using **ANSI escape codes**.

## Requirements
- Rust - Install via [rustup](https://rustup.rs/)

## How to Run

1. **Clone or create the project**:
   ```bash
   cargo new calculator
   cd calculator
   ```

2. **Replace src/main.rs with the main.rs code.**
3. **Run the calculator:**
    ```bash
    cargo run
    ```
4. **Expected Output**
    ```bash
    --- Welcome to the Rust Calculator ---
    Enter the first number:
    > 12.5
    Enter the second number:
    > 4.2
    Enter operation (+, -, *, /):
    > /
    Result: 2.9761903
    ```

