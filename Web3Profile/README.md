# Web3 Profile System in Rust

This project is a Rust implementation of a simple Web3 profile system that models professionals like Developers and Designers.

---

## ✅ Features

- Structs for `Developer` and `Designer`
- Trait implementation for custom introductions
- `impl` blocks with custom methods
- Bonus: Lifetime demo using annotated functions

---

## 🔧 Structs

```rust
struct Developer<'a> {
    name: &'a str,
    favorite_lang: &'a str,
    years_experience: u32,
}
