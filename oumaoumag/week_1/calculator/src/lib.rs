
pub fn colorize(text: &str, color_code: u8) -> String {
    format!("\x1b[38;5;{}m{}\x1b[0m", color_code, text)
}

pub fn calculator(expression: &str) {
    let tokens: Vec<&str> = expression.split_whitespace().collect();

    if tokens.len() != 3 {
        eprintln!( "{}", 
        colorize("Usage: <number> <op> <number>, e.g 1 + 2", 196)
    );
        return;
    }

    let first = match tokens[0].parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("{}",
            colorize(&format!("Invalid number: {}", tokens[0]), 196)
        );
            return;
        }
    };

    let second = match tokens[2].parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("{}",
            colorize(&format!("Invalid number: {}", tokens[2]), 196) 
        );
            return;
        }
    };

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

        println!(
            "{}",
            colorize(&format!("The Solution is : {}", result), 46)
        );
}
