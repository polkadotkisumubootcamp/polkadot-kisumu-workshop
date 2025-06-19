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


//Initialize multiple developers
fn main() {
    
  struct Developer {
      name: String,
      language: String,
      experience: u8,
  }
  let dev1 = Developer {
      name: String::from("Bob"),
      language:String::from("rust"),
      experience: 23,
  };
  let dev2 = Developer {
      name: String::from("Kamau"),
      language:String::from("Go"),
      experience: 20,
  };
  
  
  println!("{} and  {} are web3 developers! each having over {} years of experience in {} and  {} languages", dev1.name, dev2.name, dev2.experience, dev1.language, dev2.language);
}
