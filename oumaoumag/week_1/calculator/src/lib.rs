pub fn calculator(expression: &str) {
    let tokens: Vec<&str> = expression.split_whitespace().collect();

    if tokens.len() != 3 {
        eprintln!("Usage: <number> <op> <number>, e.g 1 + 2");
        return
    };


    let first = tokens[0]
    let first = match first.parse::<f32>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid number: {}", tokens[0]);
            return;
        }
    };

    let second = tokens[2];
    let second = match second.parse::<f32>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid number: {}", tokens[2]);
            return;
        }
    };

    let result = match tokens[1] {
        "+" => first + second,
        "-" => first - second,
        "*" | "x" | "X" => first * second,
        "/" => {
            if second == 0.0 {
                eprintln!("Error:division by zero");
                return;
            }
            first / second
        }
        op => {
            eprintln!("Unsupported operator: {}", op);
            return
        }
    };

        println!("The Solution is : {}", result);
}
