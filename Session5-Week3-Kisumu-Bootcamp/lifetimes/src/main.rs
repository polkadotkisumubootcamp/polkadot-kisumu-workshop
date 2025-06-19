/*
 Lifetimes in Rust?

In simple words:

    Lifetimes tell the Rust compiler how long references are valid — so it can protect your program from using data that no longer exists.

 Why Do We Need Lifetimes?

Rust doesn’t allow dangling references (pointers to dead data). So:

    Without lifetimes, Rust can't tell how long each reference is supposed to live.

    Lifetimes help Rust guarantee that your references stay valid.

    They are especially important when:

        A function returns a reference

        You are working with multiple references

 Real-Life Analogy

Imagine you're borrowing two books from a library, and you want to keep the one with more pages.

But:

    One book must be returned tomorrow (short lifetime)

    One can be kept for a month (long lifetime)

Now, if you say:

    “I'll keep the longer book,”

The librarian needs to know:

    “Which book are you keeping? How long can you legally keep it?”

That’s what lifetimes do in Rust:
They tell the compiler how long each borrowed reference is valid, and make sure you're not holding onto invalid data.
*/


//example ;


// Define a function called `longest`
// The `'a` is a lifetime parameter that tells Rust:
// "The returned reference will live as long as the shortest-lived input reference"
// fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//     // Compare lengths and return a reference to the longer string
//     if a.len() > b.len() {
//         a // Return reference to `a`
//     } else {
//         b // Return reference to `b`
//     }
// }

// fn main() {
//     // Create two String values
//     let s1 = String::from("short");
//     let s2 = String::from("longer");

//     // Pass references of both strings into the `longest` function
//     // Rust checks lifetimes here to ensure safety
//     let result = longest(&s1, &s2);

//     // Print the longer string
//     println!("Longest: {}", result); // Output: Longest: longer
// }
