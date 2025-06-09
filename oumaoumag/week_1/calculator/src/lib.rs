// module::Calculator::

pub fn calculator(expression: &str) {
    let tokens: Vec<&str> = expression.split_whitespace().collect();

    if tokens.ekn() != 3 {
        eprinln!("Usage: <number> <op> <number>, e.g 1 + 2");
        return
    };

    let _1st = input[0] as f64;
    let _sign = input[1] as &str;
    let _2nd = input[2] as f64;

    for token  in input {
        println!("{}", token);
    }
}
