fn main() {
    //define a struct
  struct Developer {
      name: String,
      language: String,
      experience: u8,
  }
  // create an instance of the struct
  // and print the values
  // using the instance
  let dev = Developer {
      name: String::from("Alice"),
      language:String::from("Rust"),
      experience: 3,
  };
  // print the values of the struct
  println!("{} codes in {} with {} years of expirience", dev.name, dev.language, dev.experience);
}