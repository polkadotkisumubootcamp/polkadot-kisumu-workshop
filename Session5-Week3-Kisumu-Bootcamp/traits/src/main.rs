/*
What is a Trait?

A trait is like a promise or a contract in Rust.

It says:

    “Any type that implements this trait must provide these specific behaviors (methods).”
    
👉 A trait is like a superpower badge.

You know how in cartoons, each character has their own superpower?
Like:

    Superman can fly

    Aquaman can talk to fish

    Flash can run fast

Now imagine you created a badge called “Flyable”.
To earn that badge, you must know how to fly.

That’s what a trait is in Rust.

It's a badge that says:

    “If you want this badge, you MUST show me how you do the thing.”

So:

    The trait says what the ability is (like fly() or greet())

    The character (struct) must show how they do it
💭 Analogy: Trait Club

Let’s say you have a “Say Hello” Club (trait).
To join the club, you must know how to say hello.

Different people say hello in different ways:

    👨‍💻 Developer: “Hi, I’m the Developer!”

    🎨 Designer: “Hello, I design things!”

    🕵️ Detective: “Greetings, I’m on the case.”

Each of them agrees to the club rule and shows how they greet.

That’s how traits work. You say:

    “Anyone who joins must have this ability, but you can do it in your own style.”

🧠 WHY do we use Traits?

Because we want to:

    Group things by what they can do
    → Example: Everyone who can say hello goes in the Hello Club

    Write less code
    → You can make one function that works for anyone in the club

    Keep things organized
    → Everyone shows their abilities in their own place

🕰️ WHEN should I use a trait?

Use a trait when:

    You have many different characters (structs)…

    …and you want them to share the same ability (like greet, print, fly, etc.)

    But each one should do it their own way

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
