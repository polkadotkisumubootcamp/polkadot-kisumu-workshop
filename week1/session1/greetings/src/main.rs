// fn main() {
//     let message = "Hello Polkadot Kisumu!"; 

//     println!("{}", message);
// }

//using constants
// const GREETING: &str = "Hello, world!";
// const NUMBER: i32 = 42;

// fn main() {
//     println!("{}", GREETING);
//     println!("The number is: {}", NUMBER);
// }


//using static
// static LANGUAGE: &str = "Rust";

// fn main() {
//     println!("We are coding in: {}", LANGUAGE);
// }


//primitive data types
// fn show_primitives() {

//     let my_float: f64 = 3.14;
//     let my_int: i32 = 42;
//     let my_str: &str = "Rust";
//     let my_char: char = 'R';

//     println!("float: {}, int: {}, str: {}, char: {}", my_float, my_int, my_str, my_char);
// }

// fn main() {
//     show_primitives();
// }

//shadowing: concept of creating new variable under same name

// fn main() {
//     let  x: i32 = 5;
//     let  x: i32 = x+5;
//     let x:i32 = x*5;

//     println!("The value of x is: {}", x)
// }


//shadowing using strings
fn main() {
    let _spaces = "   ";      
    let spaces = "helloo";   
    let spaces = spaces.len(); 

    println!("spaces: {}", spaces); 
}

