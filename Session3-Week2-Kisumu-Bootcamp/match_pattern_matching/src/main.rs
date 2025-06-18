fn main() {
    
    // Build a login system
      // The match statement is used to compare the value of username against multiple patterns.
     
    // This is the first way. THe lazy way. Test DRY concept

    let username = "";

    match username {
        "bobo" => println!("Welcome back , {}!", username),
        "alice" => println!("Welcom back, {}!", username),
        "jack" => println!("Welcome back, {}!", username),
        _ => println!("User not found, please try again!"),
    }
    
    // This is the second way

    // let username = "unknown";

    // match username {
    //     "bobo" | "alice" | "jack" => println!("Welcome back {}!", username),
    //     _ => println!("User not found! Please try again later")
    // }
   
    // THe third way . Clean and maintainable

    // let usernames = ["bob", "alice", "jack"];

    // for username in usernames {
    //     match username {
    //         "bobo" | "alice" | "jack" => println!("Welcome back user {}!", username),
    //         _ => println!("User not found,Please try again later!")
    //     }
    // }   
}