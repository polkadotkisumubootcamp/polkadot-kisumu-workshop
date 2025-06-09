// module::Calculator::

pub fn calculator(expression: &str) {
    let tokens: Vec<&str> = expression.split_whitespace().collect();

    if tokens.len() != 3 {
        eprinln!("Usage: <number> <op> <number>, e.g 1 + 2");
        return
    };


    let 1st = match tokens[0].parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            eprinln!("Invalid number: {}", tokens[0]);
            return;
        }
    };

    let 2nd = tokens[2].parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            eprinln!("Invalid number: {}", tokens[2]);
            return;
        }
    };

    let result = match tokens[1] {
        "+" => 1st + 2nd,
        "-" => 1st - 2nd,
        "*" | "x" | "X" => 1st * 2nd,
        "/" => {
            if 2nd == 0.0 {
                eprinln!("Error:division by zero");
                return;
            }
            1st / 2nd
        }
        op => {
            eprinln!("Unsupported operator: {}", op);
            return
        }
    };

        println!("{}", Result);
}
