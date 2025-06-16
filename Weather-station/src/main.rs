use std::io;

fn main() {
    println!("Welcome to the Weather Suggestion Assistant!");

    loop {
        // Program will go here
        println!("Would you like a weather suggestion? (yes/no):");
let mut choice = String::new();
io::stdin().read_line(&mut choice).expect("Failed to read line");

match choice.trim().to_lowercase().as_str() {
    "yes" | "y" => {
        // Get inputs and show suggestions
    },
    "no" | "n" => {
        println!("Goodbye!");
        break; // Exit the loop
    },
    _ => {
        println!("Please enter 'yes' or 'no'");
        continue; // Skip to next iteration
    }
}
    }
}
fn get_temperature_input() -> i32 {
    loop {
        println!("Please enter the current temperature (in degrees):");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line");

        match temp.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number:"),
        }
    }
}
fn get_weather_condition_input() -> String {
    loop {
        println!("Please enter the weather condition (sunny, rainy, cloudy, snowy):");
        let mut condition = String::new();
        io::stdin().read_line(&mut condition).expect("Failed to read line");

        let condition = condition.trim().to_lowercase();
        if !condition.is_empty() {
            return condition;
        }
        println!("Please enter a valid condition");
    }
}
fn get_suggestions(temp: i32, condition: &str) -> (String, String) {
    // Weather comment
    let comment = match condition {
        "sunny" => "It's sunny! Don't forget sunscreen.",
        "rainy" => "It's raining - grab an umbrella!",
        "cloudy" => "Cloudy skies today.",
        "snowy" => "Snow! Watch for icy surfaces.",
        _ => "Unusual weather today!",
    };

    // Clothing suggestion
    let clothing = if temp <= 32 {
        "Heavy winter coat"
    } else if temp <= 50 {
        "Jacket"
    } else if temp <= 70 {
        "Light jacket"
    } else {
        "T-shirt"
    };

    (clothing.to_string(), comment.to_string())
}
let (clothing, comment) = get_suggestions(temp, &condition);
println!("\nSuggestions:");
println!("- Weather: {}", comment);
println!("- Clothing: {}", clothing);