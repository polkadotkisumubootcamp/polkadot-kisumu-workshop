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


//example 2 

// fn main() {
//     // Define a struct named Developer with name and experience fields
//     struct Developer {
//         name: String,       // name is a String (owned text)
//         experience: u8,     // experience is an unsigned 8-bit number
//     }

//     // Implement methods (functions) for the Developer struct
//     impl Developer {
//         // Define a method named `summary` that returns a String
//         fn summary(&self) -> String {
//             // Use the format! macro to create and return a String
//             // {} are placeholders for self.name and self.experience
//             format!("{} has {} years of experience.", self.name, self.experience)
//         }
//     }

//     // Create a Developer instance named `dev`
//     let dev = Developer {
//         name: String::from("frank"),  // set name to "frank"
//         experience: 6,                // set experience to 6
//     };

//     // Call the summary() method on `dev` and print the returned string
//     println!("{}", dev.summary()); // Output: frank has 6 years of experience.
// }

/*
✅ format!() is like println!() but instead of printing, it builds a string and returns it.

📦 You can then use the string anywhere — store it, return it, or print it later.

&self lets the method access the values inside the specific Developer (like dev).

This is a clean way to give your struct a "behavior" — just like a real object.
*/

