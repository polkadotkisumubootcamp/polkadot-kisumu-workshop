use std::io::{self, Write};

fn main() {
    println!(
        "\nWelcome to weather suggestion assistant. Please enter a temprature value to continue:"
    );

    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let temperature = convert_string_to_float(&input.trim());
    println!(
        "Temperature '{}' recorded.\n\nProceed by entering a weather condition:",
        temperature
    );

    input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let weather_condition = input.trim();
    println!("Weather condition '{}' recorded.", weather_condition);

    let temperature_comment = get_temperature_comment(temperature);
    println!(
        "\nTemperature comment for '{} degrees' is '{}'.",
        temperature, temperature_comment
    );

    let clothing_suggestion = suggest_clothing(&temperature_comment);
    println!("\nClothing suggestion: {}", clothing_suggestion);
}

// Returns 0.0 as the default value for invalid input
fn convert_string_to_float(input: &str) -> f32 {
    match input.trim().parse::<f32>() {
        Ok(temp) => temp,
        Err(_) => 0.0,
    }
}

fn get_temperature_comment(temperature: f32) -> String {
    match temperature {
        t if t >= 35.0 => "Extremely hot",
        t if t >= 30.0 => "Very hot",
        t if t >= 25.0 => "Hot",
        t if t >= 20.0 => "Warm",
        t if t >= 15.0 => "Mild",
        t if t >= 10.0 => "Cool",
        t if t >= 5.0 => "Cold",
        t if t >= 0.0 => "Very cold",
        _ => "Freezing cold",
    }
    .to_string()
}

fn suggest_clothing(temp_comment: &str) -> String {
    if temp_comment == "Extremely hot" {
        "Light clothing, shorts, t-shirt, and a sun hat"
    } else if temp_comment == "Very hot" {
        "Light breathable clothing, sunglasses, and a cap"
    } else if temp_comment == "Hot" {
        "Light clothing and comfortable shoes"
    } else if temp_comment == "Warm" {
        "Light long sleeves or t-shirt"
    } else if temp_comment == "Mild" {
        "Light jacket or sweater"
    } else if temp_comment == "Cool" {
        "Jacket and long pants"
    } else if temp_comment == "Cold" {
        "Heavy jacket, scarf, and warm shoes"
    } else if temp_comment == "Very cold" {
        "Winter coat, gloves, scarf, and warm boots"
    } else {
        "Heavy winter clothing, thermal wear, and insulated boots"
    }
    .to_string()
}
