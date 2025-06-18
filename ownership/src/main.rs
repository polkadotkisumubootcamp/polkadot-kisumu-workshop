fn main() {
//    Way 1

let x = String::from("bootcamp");
let y = x.clone();  // clone() creates a deep copy of x
println!("x: {}, y: {}", x, y);

// way 2: Immutable borrow

let mut s = String::from("Quinter");
print_length(&mut s);
println!("Second way: {}", s);
}

fn print_length(s : &mut String) {
    s.push_str(" is amazing at rust!");
}
