// fn main() {
//     //define a struct
//   struct Developer {
//       name: String,
//       language: String,
//       experience: u8,
//   }
//   // create an instance of the struct
//   // and print the values
//   // using the instance
//   let dev = Developer {
//       name: String::from("Alice"),
//       language:String::from("Rust"),
//       experience: 3,
//   };
//   // print the values of the struct
//   println!("{} codes in {} with {} years of expirience", dev.name, dev.language, dev.experience);
// }


// //Initialize multiple developers
// fn main() {
    
//   struct Developer {
//       name: String,
//       language: String,
//       experience: u8,
//   }
//   let dev1 = Developer {
//       name: String::from("Bob"),
//       language:String::from("rust"),
//       experience: 23,
//   };
//   let dev2 = Developer {
//       name: String::from("Kamau"),
//       language:String::from("Go"),
//       experience: 20,
//   };
  
  
//   println!("{} and  {} are web3 developers! each having over {} years of experience in {} and  {} languages", dev1.name, dev2.name, dev2.experience, dev1.language, dev2.language);
// }


// fn main(){
//   struct Developer {
//       name: String,  // owned String
//       language: String, // owned String
//       experience: u8,
//   }
//   //a helper function that creates a Developer instance for you.

//   fn create_developer(name: &str, language: &str, experience: u8) -> Developer { //automatically borrowed when passed to this function:
//       Developer {
//           name: name.to_string(),  //// clone the string from &str into owned String
//           language: language.to_string(),  // clone the string from &str into owned String
//           experience,
//       }
//   }
//   let dev = create_developer("Diana", "rust", 4);  // borrowed string values.
//   //a helper function that creates a Developer instance for you.

//   fn create_frontend(name: &str, language: &str, experience: u8) -> Developer { //automatically borrowed when passed to this function:
//       Developer {
//           name: name.to_string(),  //// clone the string from &str into owned String
//           language: language.to_string(),  //// clone the string from &str into owned String
//           experience,
//       }
//   }
//   let dev2 = create_frontend("Allan", "Go", 23); // borrowed string values.
//   println!("{} uses {} and has {} years of experience.", dev.name, dev.language, dev.experience);
//   println!("{} uses {} and has {} years of experience.", dev2.name, dev2.language, dev2.experience);
// }