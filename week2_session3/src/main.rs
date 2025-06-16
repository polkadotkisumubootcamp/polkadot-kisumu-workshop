use std::io::{self, Write};

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    // flush to ensure print! macro displays before input
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}

// Function to match weather and provide comment
fn get_weather_comment(weather: &str) -> String {
    match weather {
        "sunny" => "It's a beautiful day with clear skies!".to_string(),
        "rainy" => "Don't forget your umbrella!".to_string(),
        "cloudy" => "A bit gloomy, but no rain yet.".to_string(),
        "snowy" => "It's snowy — stay warm and drive safe.".to_string(),
        _ => format!("Hmm, I don't recognize '{}', but stay safe out there!", weather),
    }
}
