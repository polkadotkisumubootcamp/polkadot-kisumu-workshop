/*
What is #[derive(Debug)]?

It’s a special instruction that tells Rust:

    “Please automatically give this struct the ability to be printed in a debug-friendly way.”

You place it above your struct like this:
#[derive(Debug)]
struct Developer { ... }

This allows you to print the whole struct using the {:?} format in println!.

Why do we need it?

By default, Rust does not allow printing custom structs directly with println!("{:?}", my_struct) because it doesn't know how to display them.

So without #[derive(Debug)], this line would cause an error:

println!("{:?}", dev); //  ERROR: Developer doesn't implement `Debug`
// But with #[derive(Debug)], it works perfectly:

By adding #[derive(Debug)], you say:

    “Go ahead Rust, automatically generate the code to let this struct be printed in debug format.”

This is super useful for:

    Debugging your values

    Inspecting struct data while testing

    Understanding what's inside your struct during development
*/

//example 1

// This line tells Rust to automatically create functionality
// so that this struct can be printed using {:?} format (debug output)

// #[derive(Debug)]
// struct Developer {
//     name: String,        // The developer's name as a String
//     language: String,    // The programming language they use
//     experience: u8,      // Their years of experience as a number (u8 means 0–255)
// }

// fn main() {
//     // Create an instance of the Developer struct
//     let dev = Developer {
//         name: "Alice".to_string(),       // Convert &str to String for the name
//         language: "Rust".to_string(),    // Convert &str to String for the language
//         experience: 3,                   // Set experience to 3 years
//     };

//     // Print the entire struct using debug format {:?}
//     // This only works because we derived Debug above the struct
//     println!("{:?}", dev);

//     // Optional improvement: You could also use {:#?} for a pretty (multiline) view like this:
//     // println!("{:#?}", dev);
// }


//example 2

// Derive the Debug trait so the Developer struct can be printed using {:?} or {:#?}

// #[derive(Debug)]
// struct Developer {
//     name: String,      // Developer's name as a String
//     language: String,  // Programming language the developer uses
// }

// fn main() {
//     // Create an instance of the Developer struct with name and language
//     let dev = Developer {
//         name: "Bob".to_string(),         // Convert &str into a String
//         language: "Solidity".to_string(),// Convert &str into a String
//     };

//     // Print the struct in a nicely formatted, line-by-line debug style
//     // {:#?} is used for pretty-printing with indentation and line breaks
//     println!("{:#?}", dev);

//     // Output will look like:
//     // Developer {
//     //     name: "Bob",
//     //     language: "Solidity",
//     // }
// }
