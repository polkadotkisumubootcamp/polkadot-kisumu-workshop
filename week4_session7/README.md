# 🛠️ Developer Dashboard – Contributor Manager

A simple Rust command-line program to manage project contributors. It demonstrates how to:

- Store a list of contributors in a vector
- Add status tags using mutable iteration
- Preview a subset of contributors using slices
- Count contributors
- Simulate adding and removing contributors
- Print lists generically using a generic function

---

## 📦 Features

- ✅ Dynamic `Vec<String>` of contributor names
- ✅ Add "Active" status using `.iter_mut()`
- ✅ Slice the list to preview top contributors
- ✅ Count total contributors using `.len()`
- ✅ Simulate adding with `.push()` and removing with `.pop()`
- ✅ Generic `print_list()` function supports any displayable list

---

## 🧑‍💻 How to Run

### 1. 🧱 Prerequisites

Ensure you have Rust installed.

```bash
cd week4_session7
cargo run
```

