✅ Minimal README.md for Developer Dashboard

# 🖥️ Developer Dashboard – Rust Practice

This is a console-based Rust program that simulates a simple developer dashboard to manage project contributors. It was built as part of the **Polkadot Kisumu Rust Bootcamp** Week 4 practice task.

---

## 🔧 What This App Does

- Stores a list of contributor names using `Vec<String>`
- Uses mutable iteration to tag each contributor as ✅ Active
- Displays a preview of the first 3 contributors using a slice
- Shows total number of contributors with `.len()`
- Simulates adding and removing contributors using `.push()` and `.pop()`
- Prints the list before and after changes
- Includes a generic function to print any list (`Vec<String>`, `Vec<i32>`, etc.)

---

## ▶️ How to Run

### 🛠 Requirements

Make sure Rust and Cargo are installed:

```bash
rustc --version
cargo --version

If not installed:

curl https://sh.rustup.rs -sSf | sh

🚀 Run the App

    Navigate to the project folder:

cd developer_dashboard

    Run it with Cargo:

cargo run

You should see terminal output showing contributors being marked as active, list previews, totals, and simulated updates.
📦 Build the App

To build the compiled binary:

cargo build
