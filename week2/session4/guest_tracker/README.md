# Guest Tracker (Console-Based) - Rust

A tiny Rust console app that demonstrates key Rust concepts like **ownership**, **borrowing**, **mutable references**, **lifetimes**, and **memory management** through a simple guest tracking program.

---

## Features

- Prompt users to enter guest names
- Store names in a list (ownership)
- Greet each guest using **immutable borrowing**
- Allow safe editing of guest names via **mutable references**
- Use a function with **lifetimes** to return a borrowed guest name
- Commented code showing where **scope ends and memory is freed**

---

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (v1.60+ recommended)

---

## How to Run

1. **copy the code into a file:**

2. **Save the code as main.rs inside a src/ directory (Rust project format):**
    ```bash
    cargo new guest-tracker
    cd guest-tracker
    Replace src/main.rs with the app code
    ```

3. **Build and run:**
    ```bash
    cargo run
    ```

