fn main() {
     // What is a tuple?
    // A tuple is a collection of values of different types grouped together.

    // Characteristics:
    // - Fixed size
    // - Can contain multiple types
    // - Index-based access using dot notation
    // - Can be mutable as a whole, but not partially

    // Accessing individual elements in a tuple using the dot notation.
    // Printing the whole tuple out with elements.
    // Mutating a tuple (updating an individual item in a tuple).
    // Destructuring a tuple.
    
    // Explicit typing
    let num : (i32, f64, i32) = (1, 20.45, 3);

    // Accessing individual elements
    // println!("The first number is : {}", num.0);
    // println!("The first number is : {}", num.1);
    // println!("The first number is : {}", num.2);

   // Updating the value at index 0
    // num.0 = 50;

    // println!("The new updated number is {}", num.0);
    // println!("The first number is : {}", num.1);
    // println!("The first number is : {}", num.2);

    //Printing the whole tuple
    println!("The whole tuple is: {:?} ", num); 

    // Destructuring the tuple
    let ( a, mut b, c) = num;
    
    println!("The value of a is: {}, The value of b: {}, The value of c: {}", a, b, c);

    b = 300.0;

    println!("The value of a is: {}, The value of b: {}, The value of c: {}", a, b, c);
    
    // Implicit typing
    let data = ("Polkadot", true);
    let swapped_data = swap(data.0, data.1);
    
    println!("THe orginal data is: {:?}", data);
    println!("The newly swapped data is: {:?}", swapped_data)
     
}

fn swap(a: &str, b:bool)-> (bool, &str) {
    // return(b, a);
    (b, a)
}