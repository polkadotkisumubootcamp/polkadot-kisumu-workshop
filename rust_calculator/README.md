# 🧮 Rust Calculator

A simple command-line calculator written in Rust.

This project was created as part of the Polkadot learning homework to practice core Rust concepts such as:
- User input handling
- Shadowed variables
- Error checking
- Constants
- Colored terminal output

---

## 🚀 Features

- Accepts two numbers from the user  
- Accepts an operation (`+`, `-`, `*`, `/`)  
- Handles division by zero  
- Uses at least one shadowed variable  
- ✅ Bonus features:
  - Accepts **`f32`** (floating-point) numbers  
  - Uses a `const` welcome banner  
  - Displays colored output using ANSI escape codes

---

## 🧰 Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- A terminal or IDE like [Visual Studio Code](https://code.visualstudio.com/)

---

## 📦 How to Run

1. Clone or download the project

2. Open terminal and run:

   ```bash
   cargo run

🖥️ Example Output

🧮 Welcome to RustCalc!
Enter the first number:
> 12.5
Enter the second number:
> 4
Enter operation (+, -, *, /):
> /
Result: 3.125

rust_calculator/
├── src/
│   └── main.rs      # Calculator logic
├── Cargo.toml       # Rust package metadata
└── README.md        # This file

