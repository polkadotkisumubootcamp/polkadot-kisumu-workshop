/*
What is an impl Block?

impl is short for implementation.

It lets you define methods (functions) for a struct.

It allows you to attach functions (called methods) to a struct, just like how objects in other languages can have methods.

Analogy: Just like a person can introduce themselves, we let our Developer struct introduce itself.

Think of impl as saying: "Hey, I want to teach my Developer struct how to do something."

impl turns your struct from just data into a living object that can take action.
*/

// fn main() {
//     // Define what a Developer is (structure/data)
//     struct Developer {
//         name: String,
//         language: String,
//     }

//     // Teach the Developer how to introduce themselves (behavior/method)
//     impl Developer {
//         fn introduce(&self) {
//             // Use self to access its own name and language
//             println!("Hi, I'm {} and I code in {}.", self.name, self.language);
//         }
//     }

//     // Create a Developer
//     let dev = Developer {
//         name: String::from("Eve"),
//         language: String::from("Rust"),
//     };

//     // Tell the Developer to introduce themselves
//     dev.introduce();
// }
