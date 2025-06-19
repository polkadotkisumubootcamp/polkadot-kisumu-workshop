// fn main() {
//    //define the mutable variable 
//    let mut kisumu = "Hello Kisumu!";

//    println!("Innitial string :{}", kisumu);
// //change the variable to another state
//    kisumu = "hello Kisii";

//    println!("Updated string: {}", kisumu);

   
// }

//define constant variable by the key word const
// it can be accessed globally / outside the functions
// const MAX_SPEED : u32 = 100;
// const APP_VERSION : &str = "version 2.0";

// fn main() {
// println!("The maximum speed allowed is : {} Km/hr", MAX_SPEED);
// println!("The new released application is : {}", APP_VERSION);
// }


// //primitive Data types

// fn main() {

//    let age : i32 = 50 ;  //integer
//    let pi : f64 = 3.14;  //float
//    let is_inclass : bool = true ; //boolean
//    let grade : char = '🥳' ;  // character

// //growable string -- can be changed later
//    let name = String::from("John Doe");

// let name : &str = "John Doe"; 



// println!("Age: {}, Pi: {}, In-Class: {}, Grade:{}, Name:{}", age, pi, is_inclass, grade, name);

// }


//shadowing - creating a new variable under same

fn main() {

   let _spaces = 2;
   let _spaces = "hello";
   let spaces = 5; //shadow it as an integer

   println!("number of spaces is :  {}", spaces);
}

//Basic operations

// fn main() {

//    let sum = 5+3 ; //addition
//    let diff = 5 -3 ; //subtraction
//    let mult = 5*3; //multiply
//    let quotient = 5/3; //division
//    let remainder = 5%3; //modulus

//    println!("sum:{}, diff:{}, mult:{}, quotient:{}, remainder:{}." , sum, diff, mult, quotient, remainder);
// }

