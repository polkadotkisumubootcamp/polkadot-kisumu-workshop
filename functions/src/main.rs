fn main() {
   let dub = double(50000.0);
   let product = multiply(4, 8);
    let lucky_number = get_lucky_number(); 
    println!("The double of 50000.0 is: {}", dub);
    println!("The product of 4 and 8 is: {}", product);
    println!("Your lucky number is: {}", lucky_number);
}

fn double(num: f64) -> f64 {
    num * 2.0
}

fn multiply(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn get_lucky_number() -> i32 {
    let lucky = 7;
    lucky
}
