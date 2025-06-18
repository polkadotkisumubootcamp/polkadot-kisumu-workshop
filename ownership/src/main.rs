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
}

fn print_length(s : &mut String) {
    s.push_str(" is amazing at rust!");
}


fn change(v: &String) {
    println!("The length is: {}", v.len());
}
