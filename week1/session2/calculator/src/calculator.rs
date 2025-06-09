use std::io;

// Color constants struct for better organization
pub struct Colors {
    pub red: &'static str,
    pub green: &'static str,
    pub yellow: &'static str,
    pub blue: &'static str,
    pub magenta: &'static str,
    pub cyan: &'static str,
    pub reset: &'static str,
}

pub const COLORS: Colors = Colors {
    red: "\x1b[31m",
    green: "\x1b[32m",
    yellow: "\x1b[33m",
    blue: "\x1b[34m",
    magenta: "\x1b[35m",
    cyan: "\x1b[36m",
    reset: "\x1b[0m",
};

// Welcome banner
pub const WELCOME_BANNER: &str = "
╔══════════════════════════════════════╗
║          🧮 RUST CALCULATOR 🧮        ║
║   Enter expressions like: 5 + 3      ║
║     Supports: +, -, *, /, ^, %       ║
╚══════════════════════════════════════╝
";

pub fn get_expression() -> String {
    loop {
        print!("{}Enter expression (e.g., 5 + 3, 10.5 * 2): {}", COLORS.blue, COLORS.reset);
        io::Write::flush(&mut io::stdout()).unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        // Shadow the input variable to trim it
        let input = input.trim().to_string();
        
        if !input.is_empty() {
            return input;
        } else {
            println!("{}Please enter a valid expression!{}", COLORS.red, COLORS.reset);
        }
    }
}

pub fn parse_and_calculate(expression: &str) -> Result<f32, String> {
    // Remove all spaces for easier parsing
    let expression = expression.replace(" ", "");
    
    // Find the operator and its position
    let mut operator_pos = None;
    let mut operator = ' ';
    
    // Look for operators (skip first character to allow negative numbers)
    for (i, ch) in expression.chars().enumerate().skip(1) {
        if "+-*/^%".contains(ch) {
            operator_pos = Some(i);
            operator = ch;
            break;
        }
    }
    
    let operator_pos = match operator_pos {
        Some(pos) => pos,
        None => return Err("No valid operator found! Use +, -, *, /, ^, or %".to_string()),
    };
    
    // Spliting the expression into parts
    let num1_str = &expression[..operator_pos];
    let num2_str = &expression[operator_pos + 1..];
    
    // Parsing the numbers
    let num1 = match num1_str.parse::<f32>() {
        Ok(n) => n,
        Err(_) => return Err(format!("Invalid first number: '{}'", num1_str)),
    };
    
    let num2 = match num2_str.parse::<f32>() {
        Ok(n) => n,
        Err(_) => return Err(format!("Invalid second number: '{}'", num2_str)),
    };
    
    // Calculate using the existing function
    calculate(num1, operator, num2)
}

pub fn calculate(num1: f32, operation: char, num2: f32) -> Result<f32, String> {
    match operation {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => {
            if num2 == 0.0 {
                Err("No division by zero!".to_string())
            } else {
                Ok(num1 / num2)
            }
        },
        '^' => Ok(num1.powf(num2)),
        '%' => {
            if num2 == 0.0 {
                Err("No modulo by zero!".to_string())
            } else {
                Ok(num1 % num2)
            }
        },
        _ => Err("Unknown operation".to_string()),
    }
}