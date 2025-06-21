# Guest Tracker

Simple console app demonstrating Rust ownership, borrowing, and lifetimes.

## Run
```bash
cd guest_tracker && cargo run
```

## Concepts Covered
- **Ownership**: `add_guest()` takes ownership of parameters
- **Borrowing**: `greet_guest()` borrows name without taking ownership  
- **Mutable borrow**: `update_notes()` requires exclusive access
- **Lifetimes**: `get_name()` returns borrowed reference

## Usage
```
1. Add  2. Greet  3. Update  4. List  5. Get name  6. Exit
Choice: 1
Name: Alice
Notes: Coffee lover
```