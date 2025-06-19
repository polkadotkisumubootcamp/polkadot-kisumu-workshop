/*
What is a Trait?

A trait is like a promise or a contract in Rust.

It says:

    “Any type that implements this trait must provide these specific behaviors (methods).”

You can think of traits like interfaces in other languages (such as Go or Java).
They define what a type can do, but not how — the how is implemented later.

*/
//Example 1
// // Define a trait named Greet
// // This trait says: "Any type that uses me must have a `greet()` method"

// trait Greet {
//     fn greet(&self); // This method must be implemented
// }

// // Define a struct to represent a Developer
// struct Developer {
//     name: String, // The developer's name
// }

// // Implement the Greet trait for the Developer struct
// // This means: "Developer agrees to use the Greet trait and defines how `greet()` works"
// impl Greet for Developer {
//     fn greet(&self) {
//         // This is the custom behavior when a Developer greets
//         println!("Hi, I'm {} the Developer!", self.name);
//     }
// }

// // Main function to run the program
// fn main() {
//     // Create a Developer instance
//     let dev = Developer {
//         name: "Eve".to_string(), // Set the name to "Eve"
//     };

//     // Call the greet method from the Greet trait
//     dev.greet(); // Output: Hi, I'm Eve the Developer!
// }

//Example 2: Another struct using same trait

// Define a trait called Greet
// A trait is like a contract: any type that uses this trait must define a greet method

// trait Greet {
//     fn greet(&self); // Method signature: no body here, only the promise
// }

// // Define a struct called Designer
// // This struct holds a designer's name
// struct Designer {
//     name: String,
// }

// // Implement the Greet trait for Designer
// // This means Designer is now allowed to use the greet method defined by the trait
// impl Greet for Designer {
//     fn greet(&self) {
//         // Define how a Designer should greet
//         println!("Hello! I'm {} the Designer.", self.name);
//     }
// }

// // Main function: entry point of the program
// fn main() {
//     // Create an instance of Designer with name "Liam"
//     let des = Designer {
//         name: "Liam".to_string(), // Convert &str to String
//     };

//     // Call the greet method (coming from the Greet trait)
//     des.greet(); // Output: Hello! I'm Liam the Designer.
// }
