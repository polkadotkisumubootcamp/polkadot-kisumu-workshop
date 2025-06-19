# 📝 Guest Tracker - Rust Console App

This is a **tiny Rust-based console application** for tracking guest names while learning key Rust concepts like ownership, borrowing (both mutable and immutable), scopes, and lifetimes.

## 🎯 Objectives

- ✅ Ask users to input guest names and store them (practice **ownership**).
- ✅ Borrow guest names to greet them without taking ownership (practice **immutable borrowing**).
- ✅ Safely allow one part of the code to edit guest details at a time (practice **mutable borrowing**).
- ✅ Add code comments to show where **scope ends** and **memory is freed**.
- ✅ Include a function that returns a **borrowed reference** to a guest name (practice **lifetimes**).

---

## 🛠 Features

- Add multiple guest names interactively via terminal input.
- Greet each guest using borrowed data.
- Allow safe editing of guest names using mutable references.
- Print farewell messages when guest data goes out of scope.
- Return the longest guest name using a lifetime-annotated function.

---

## 🧠 Rust Concepts Practiced

| Concept       | Purpose                                                                 |
|---------------|-------------------------------------------------------------------------|
| Ownership     | Store each guest name with proper value ownership.                      |
| Borrowing     | Read guest data safely without taking ownership.                        |
| Mutable Borrowing | Allow only one part of the code to change a guest name at a time.   |
| Scopes        | Highlight where memory is automatically freed when variables go out of scope. |
| Lifetimes     | Ensure references returned from functions are valid and safe to use.    |

---

## ▶️ Running the App

Make sure you have Rust installed. Then:

```bash
cargo run
```
## 📅 Submission Deadline:
Tuesday, 20th June 2025