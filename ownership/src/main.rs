fn main() {
//    Way 1

let x = String::from("bootcamp");
let y = x.clone();  // clone() creates a deep copy of x
println!("x: {}, y: {}", x, y);

// way 2: Mutable borrow

let mut s = String::from("Quinter");
print_length(&mut s);
println!("Second way: {}", s);

// way 3: Immutable borrow

let t = String::from("Wahala");

change(& t);

// Example 2: Immutable borrows: 

let b = String::from("Kusama");
let r1 = &b; // Immutable borrow
let r2 = &b; // Another immutable borrow
println!("r1: {}, r2: {}", r1, r2);

// let r3 = &mut b; // Mutable borrow
// println!("r3: {}", r3); // This line would cause a compile-time error

// Example 3: Dangling references
let dangling = dangle();
println!("Dangling reference: {}", dangling);

}

fn print_length(s : &mut String) {
    s.push_str(" is amazing at rust!");
}


fn change(v: &String) {
    println!("The length is: {}", v.len());
}

fn dangle() -> String {
    let s = String::from("buju");
    s
}