const MAX_SPEED : u32 = 100;
const APP_VERSION : &str = "version 2.0";

fn main() {

   let mut city = "Hello Kisumu!";
   city = "Hello Kisii";
   let age : i32 = 50;
   let pi : f64 = 3.14;
   let is_in_class : bool = true;
   let grade : char = 'A';
   let name : &str = "John Doe";

   let sum = 5 + 3;
   let diff = 5 -3;
   let mult = 5 *3;
   let div = 5 / 3;
   let modl = 5 % 3;


//    Shadowing - creating a new variable under same name

let x = 5;
let x = x + 2;

let spaces = "         ";
let spaces = spaces.len(); // shadow it as integer
  
  
   println!("{city} This is a polkadot bootcamp");
   println!("The maximun speed allowed is : {} Km/hr", MAX_SPEED);
   println!("The new realeased application is : {},", APP_VERSION);
   println!(" Name: {}, Age: {}, Pi: {}, In class: {}, Grade: {}",name, age,pi,is_in_class,grade);
   println!("Shadow value of x is: {}", x);
   println!("Number of spaces is : {}", spaces);
   println!("{},{},{},{},{}", sum, diff, mult, div, modl);

}
