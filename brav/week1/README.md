# Rust Calculator - Polkadot Workshop Assignment

## Assignment Requirements

This calculator implementation fulfills all the required tasks:

### Core Requirements:
- **Accepts two numbers**: Uses `f32` for floating-point precision
- **Accepts operation from user**: Supports +, -, *, / operations
- **Handles division by zero**: Proper error handling with clear message
- **Uses at least one shadowed variable**: Multiple examples of variable shadowing

### Bonus Features:
- **f32 inputs instead of integers**: All calculations use `f32` type
- **Const for welcome banner**: `WELCOME_MESSAGE` constant
- **Colors using escape codes**: Green color for result lines

## Features

### Variable Shadowing Examples:
1. `first_number` is shadowed to itself (simple shadowing)
2. `first_number` is shadowed again with `.abs()` (absolute value)
3. `input` variable is reused for different inputs

### Error Handling:
- Invalid number input validation
- Division by zero protection
- Invalid operation handling

### User Experience:
- Clean, simple output
- Clear error messages
- Continuous operation loop
- Graceful exit with 'quit' command

## How to Run

```bash
# Navigate to the calculator directory
cd brav/week1

# Run the calculator
cargo run

# Or compile and run
cargo build
./target/debug/calculator
```

## How to Test

```bash
# Run the included tests
cargo test
```

## Example Usage

```
Enter the first number: 10.5
Enter the second number: 2.5
Choose an operation (+, -, *, /): /
10.5 / 2.5 = 4.2
```

## Code Structure

- **Main function**: Handles user interaction loop
- **Input validation**: Robust parsing with error handling
- **Welcome constant**: Simple welcome message
- **Test module**: Basic unit tests for validation
