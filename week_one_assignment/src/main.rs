fn main() {
    println!("Hello, world!");

}

fn calculator(a: i32, b: i32, op : &str ) -> i32 {

    if op == "+" {
        return a + b;
    } else if op == "-" {
        return a - b;
    } else if op == "*" {
        return a * b;
    } else if op == "/" {
        if b != 0 {
            return a / b;
        } else {
            panic!("Division by zero is not allowed.");
        }
    } else if op == "%" {
        if b != 0 {
            return a % b;
        } else {
            panic!("Division by zero is not allowed.");
        }
        } else {
        panic!("Unsupported operation: {}", op);
    }   
}