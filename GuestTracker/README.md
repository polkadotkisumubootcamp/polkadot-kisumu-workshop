# 🎉 Guest Tracker App (Rust Practice)

This is a simple **console-based Rust app** to practice key Rust concepts:
- Ownership
- Borrowing (immutable and mutable)
- Lifetimes
- Memory safety

---

## 🧩 Features

- Collect guest names from user input.
- Greet each guest by borrowing their name (immutable borrow).
- Modify a guest's name safely (mutable borrow).
- Demonstrate memory management and lifetimes.
- Commented clearly to explain scope and ownership.

---

## 🔧 How It Works

### 🏷 Ownership
Guest names are stored in a `Vec<String>`. When pushed, ownership of the name is moved into the vector.

### 📚 Borrowing
- Greet functions use **immutable references** (`&String`).
- Edit function uses a **mutable reference** (`&mut String`).

### 🧬 Lifetimes
A function returns a **borrowed reference** to the first guest, using explicit lifetime annotations.

---

## ▶️ How to Run

1. Ensure [Rust is installed](https://www.rust-lang.org/tools/install).
2. Clone this repo or create a `main.rs` file with the code.
3. Run the app:

```bash
cargo run
