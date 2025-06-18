# 🌤️ Weather Suggestion Assistant (Rust)

A beginner-friendly Rust program that helps users decide what to wear and provides weather-based comments using user input, conditionals, and pattern matching.

---

## ✅ Task Overview

This project was built as a **practice task for Polkadot learning** to apply fundamental Rust concepts like:

- Reading user input
- Using `match` for pattern matching
- Applying `if` / `else if` logic
- Writing and calling functions
- Returning tuples
- Handling unknown input
- Using terminal colors with ANSI escape codes
- (Bonus) Looping for multiple suggestions

---

## 🧠 Features

- Asks for **temperature** and **weather condition**
- Gives:
  - 🧥 **Clothing suggestion** based on temperature
  - ☁️ **Weather comment** based on condition
- Supports weather types: `sunny`, `rainy`, `cloudy`, `snowy`
- Gracefully handles **unknown weather** input
- Option to **repeat** for multiple suggestions

---

## 📦 Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- VS Code or any code editor with terminal support

---

## 🚀 Running the App

1. **Clone the repo or create a new project**:

```bash
cargo new weather_suggestion
cd weather_suggestion
cargo run

🌤️ Weather Suggestion Assistant
Enter the temperature (°C):
> 8
Enter the weather condition (e.g., sunny, rainy):
> snowy

Your Suggestions:
🧥 Wear a heavy jacket.
❄️ Snowy! Stay warm and drive safe.

Would you like another suggestion? (yes/no):
> no
Goodbye! Stay safe and dress well!
```
