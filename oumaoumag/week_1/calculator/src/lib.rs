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
            return
        }
    },
    let sign = tokens[1].parse::< &str;

    for token  in input {
        println!("{}", token);
    }
}
