// A function that doubles a number
fn double(num: i32) -> i32 {
    num * 2
    // return num * 2
}
// A function that multiplies 2 numbers

fn multiply(a: i32, b: i32) -> i32 {
     a * b
}
// A function to convert celsius to Fahrenheit

fn to_fahreneit (celsius: f32) -> f32 {
    (celsius * 1.8) + 32.0
}
// A function with no param and returns a fixed value

fn get_lucky_number() -> i32 {
    4
}
// A function that checks if a number is even

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
fn main() {
       // doubling a number
       let result = double(5);
       println!("The result of our operation is: {}", result);

       // multiplying a number
       let product = multiply(3, 2);
       println!("The multiplied number is: {}", product);

       // convert celsius to fahrenheit
       let ft = to_fahreneit(25.0);
       println!("25 is {}°F", ft);

       // no param and returns a fixed value
       let lucky = get_lucky_number();
       println!("Your lucky number is: {}", lucky);

       // checks if a number is even
       let check_even_number = is_even(12);
       println!("The even number is: {}", check_even_number);

}