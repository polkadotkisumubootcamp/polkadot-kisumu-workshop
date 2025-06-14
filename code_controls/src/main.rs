fn main() {

    // Example of a simple if-else statement
    let age = 80;
    if age < 18 {
        println!("You are a minor.");
    } else if age >= 18 && age < 65 {
        println!("You are an adult.");
    } else {
        println!("You are a senior citizen.");
    }

    // Example of a match statement
    // way1

    let username = "bob";

    match username {
        "bob" => println!("Welcome back, {}", username),
        "alice" => println!("Welcome back, {}", username),
        "piper" => println!("Welcome back, {}", username),
        _ => println!("Welcome, new user!")
    }

    // way2
    let username = "bob";
    match username{
        "bob" | "alice" |"piper" => println!("Welcome back, {}", username),
        _ => println!("Welcome, new user!")
    }

    // way3

    let username = ["bob", "alice", "piper"];
    for user in username{
        match user {
            "bob" | "alice" | "piper" => println!("Glad you are back, {:?}", user),
            _ => println!("Welcome, new user!")
        }
    }
}
