fn main() {
    // 🍪 We create a snack using `Some`. This means there's a value in the box!
    let snack = Some("Cookie");

    // 🎁 We're checking what's inside the `snack` using match
    match snack {
        // 🎉 If it's `Some(food)`, we got a treat! Print it
        Some(food) => println!("Yay! I got a {}!", food),

        // 😭 If it's `None`, the snack box was empty
        None => println!("No snack today..."),
    }

    // 📭 This is an explicitly empty snack box. We declare it as `None`
    let empty_box: Option<&str> = None;

    // 🧐 Check the empty box using match again
    match empty_box {
        // 🍔 If there's food, we celebrate (won’t happen in this case)
        Some(food) => println!("Yum! {}", food),

        // 😢 If the box is empty, we print a sad message
        None => println!("Aw man, it's empty!"),
    }
}
