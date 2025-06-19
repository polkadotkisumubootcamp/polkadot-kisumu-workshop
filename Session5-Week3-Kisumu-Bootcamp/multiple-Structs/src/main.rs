// This program demonstrates the use of multiple structs in Rust.
// It defines two structs: Developer and Designer, each with their own fields.
// fn main() {
    
//     struct Developer {
//         name:String,
//         language: String,
//     }
//     struct Designer {
//         name: String,
//         tool: String,
//     }
//     let dev = Developer {
//         name:String::from("Hassan"),
//         language:String::from("Rust"),
//     };
//     let designer = Designer {
//         name:String::from("Ivy"),
//         tool:String::from("Figma"),
//     };
//     println!("{} codes in {} language",dev.name, dev.language);
//     println!("{} designs  using {} tool", designer.name, designer.tool);
// }


//example 2

// This program demonstrates how to use multiple structs in Rust to represent different types of developers and manage their interactions.

// fn main() {
//     // Define a struct for a Backend Developer with a name field
//     struct BackendDev {
//         name: String, // This field stores the developer's name as a String
//     }

//     // Define a separate struct for a Frontend Developer, also with a name field
//     struct FrontendDev {
//         name: String, // This also stores the developer's name
//     }

//     // Define a function that takes two developers (one backend, one frontend)
//     // This function borrows both structs using references (&)
//     fn team_meeting(backend: &BackendDev, frontend: &FrontendDev) {
//         // Print a message showing the names of the two developers in the meeting
//         println!(
//             "team meeting with {} (Backend) and {} (Frontend)",
//             backend.name, frontend.name
//         );
//     }

//     // Create a BackendDev instance and set the name to "john"
//     let backend = BackendDev {
//         name: String::from("john"),
//     };

//     // Create a FrontendDev instance and set the name to "jane"
//     let frontend = FrontendDev {
//         name: String::from("jane"),
//     };

//     // Call the team_meeting function and pass references to both developers
//     // We use & to borrow the data without moving ownership
//     team_meeting(&backend, &frontend);

//     // 🖨️ Output will be:
//     // team meeting with john (Backend) and jane (Frontend)
// }
