# 🌤️ Weather Advisor - Rust Console App

This is a **Rust-based console application** designed to give weather-based clothing suggestions. It's a hands-on beginner project that demonstrates `match`, `if/else`, tuple handling, function usage, input parsing, and lifetime-safe logic in Rust.

---

## 🎯 Objectives

* ✅ Ask users for **temperature** and **weather condition**.
* ✅ Use `match` to display a custom **weather comment**.
* ✅ Use `if`, `else if` to suggest **what to wear** based on temperature.
* ✅ Create a **function** that takes temperature and weather as input, and returns a **tuple** of clothing suggestion and comment.
* ✅ Display the final recommendation using the returned tuple.
* ✨ **Bonus:** Let users request **another suggestion** in a loop.
* ✅ Gracefully handle unknown weather conditions with `match _`.

---

## 🛠️ Features

* Accepts real-time input from the user via the terminal.
* Uses `match` to interpret weather types and give friendly messages.
* Evaluates temperature ranges to recommend suitable clothing.
* Wraps logic into a clean function returning a result as a **tuple**.
* Allows repeat use by asking the user if they want to try again (bonus).
* Avoids crashes by gracefully handling unknown input.

---

## 🧠 Rust Concepts Practiced

| Concept        | Purpose                                                                |
| -------------- | ---------------------------------------------------------------------- |
| `match`        | Handle multiple weather conditions with expressive, readable branches. |
| `if/else if`   | Make decisions based on numeric temperature values.                    |
| Tuples         | Return multiple values (clothing + comment) from a function.           |
| Functions      | Encapsulate logic to reuse and separate concerns.                      |
| Loops          | Ask the user if they want to run the program again.                    |
| Error Handling | Use `_` in `match` to manage unknown/invalid input.                    |
| User Input     | Practice getting and parsing input using `std::io`.                    |

---

## ▶️ Running the App

Ensure you have Rust installed. Then:

```bash
cargo run
```

You'll be prompted to enter:

* A **temperature** (e.g. `25`)
* A **weather condition** (e.g. `sunny`, `rainy`, `cloudy`, `snowy`, etc.)

Then you'll get a clothing suggestion and a weather comment based on your input.

---

## 📅 Submission Deadline:

Friday, 16th June 2025

---

Happy coding and stay warm or cool accordingly! 🌡️👕
