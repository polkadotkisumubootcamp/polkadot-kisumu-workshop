/// Adds ANSI color formatting to text
pub fn colorize(text: &str, color_code: u8) -> String {
    format!("\x1b[38;5;{}m{}\x1b[0m", color_code, text)
}

/// Processes a mathematical expression in the format "number operator number"
/// Supports operations: +, -, *, x, X, /
pub fn calculator(expression: &str) {
    // Split input into tokens (number, operator, number)
    let tokens: Vec<&str> = expression.split_whitespace().collect();

    // Validate input format
    if tokens.len() != 3 {
        eprintln!( "{}", 
        colorize("Usage: <number> <op> <number>, e.g 1 + 2", 196)
    );
        return;
    }

    let first = tokens[0];
    let first = match first.parse::<f32>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("{}",
            colorize(&format!("Invalid number: {}", tokens[0]), 196)
        );
            return;
        }
    };

    let second = tokens[2];
    let second = match second.parse::<f32>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("{}",
            colorize(&format!("Invalid number: {}", tokens[2]), 196) 
        );
            return;
        }
    };

    // Process the operation based on the operator
    let result = match tokens[1] {
        "+" => first + second,
        "-" => first - second,
        "*" | "x" | "X" => first * second,
        "/" => {
            if second == 0.0 {
                eprintln!("{}", colorize("Error:division by zero", 196));
                return;
            }
            first / second
        }
        op => {
            eprintln!(
                "{}",
                colorize(&format!("Unsupported operator: {}", op), 196)
            );
            return;
        }
    };

    // Display the result in green color
    println!(
        "{}",
        colorize(&format!("The Solution is : {}", result), 46)
    );
}
