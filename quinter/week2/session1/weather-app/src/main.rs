use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use colored::*;

fn main() {
    // Print welcome message
    println!("{}", "🌻 Welcome to the Artisan Weather Atelier! 🌻".bright_yellow().bold());
    println!("{}", "Where fashion meets meteorology in perfect harmony...\n".italic());
    
    // Get current hour for time-based greeting
    let hour = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() / 3600 % 24;
    
    // Set greeting based on time of day
    let greeting = match hour {
        5..=11 => "Good morning",
        12..=16 => "Good afternoon",
        17..=21 => "Good evening",
        _ => "Hello night owl",
    };
    
    // Main program loop
    loop {
        println!("\n{} {}\n", greeting.green(), "weather enthusiast! What's the current climate?".italic());
        
        // Get temperature input
        println!("{}", "☛ Please enter the temperature (°C):".bright_cyan());
        let temperature = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            
            match input.trim().parse::<i32>() {
                Ok(num) => break num,
                Err(_) => println!("{}", "✗ Please enter a valid number".red()),
            };
        };
        
        // Get weather condition input
        println!("\n{}", "☛ How would you describe the weather today?".bright_cyan());
        println!("{}", "(sunny, rainy, cloudy, snowy, or other)".purple().italic());
        
        let mut weather = String::new();
        io::stdin().read_line(&mut weather).expect("Failed to read input");
        let weather = weather.trim().to_lowercase();
        
        // Get clothing and weather suggestions
        let (clothing, comment) = craft_suggestions(temperature, &weather);
        
        // Display recommendations
        println!("\n{}", "✨ Weather Recommendations ✨".bold().bright_magenta());
        println!("{} {}", "Temperature:".bold(), format!("{}°C", temperature).bright_blue());
        println!("{} {}", "Weather:".bold(), capitalize_first_letter(&weather).bright_blue());
        println!("\n{} {}", "🧥 Clothing Suggestion:".bold().bright_yellow(), clothing.italic());
        println!("{} {}", "💭 Weather Comment:".bold().bright_yellow(), comment.italic());
        
        // Ask to continue
        println!("\n{}", "Would you like another suggestion? (yes/no)".bright_green());
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        
        if choice.trim().to_lowercase() != "yes" {
            println!("\n{}", "🕊️ Goodbye! Have a wonderful day! 🕊️".bright_cyan());
            println!("{}", "                 ~ The Artisan Weather Atelier ~".dimmed());
            break;
        }
    }
}

// Generate clothing and weather suggestions
fn craft_suggestions(temperature: i32, weather: &str) -> (String, String) {
    // Weather description
    let weather_comment = match weather {
        "sunny" => "The sun is shining brightly today".to_string(),
        "rainy" => "Don't forget your umbrella".to_string(),
        "cloudy" => "The sky is overcast".to_string(),
        "snowy" => "Watch out for icy patches".to_string(),
        _ => format!("Interesting weather today: {}", capitalize_first_letter(weather)),
    };
    
    // Clothing recommendation based on temperature
    let clothing_suggestion = if temperature < -10 {
        "Heavy winter coat with warm accessories".to_string()
    } else if temperature < 0 {
        "Winter jacket with layers".to_string()
    } else if temperature < 10 {
        "Light jacket or sweater".to_string()
    } else if temperature < 20 {
        "Light jacket or long sleeves".to_string()
    } else {
        "T-shirt and shorts".to_string()
    };
    
    (clothing_suggestion, weather_comment)
}

// Capitalize first letter of string
fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}