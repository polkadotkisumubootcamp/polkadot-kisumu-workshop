fn main() {
  // Immutable borrow
//    let s = String::from("Kusama");
//    print_length(&s); // Borrowing Immutably
//    println!("s is still valid: {}", s);

  // Mutable borrow
//    let mut s = String::from("blockhead");
//    change(& mut s);
//    println!("{}", s);

  // Mixing the two
  let s = String::from("Wahala!");

  let r1 = &s;  // immutable borrow
  let r2 = &s;  // immutable borrow
//   let r3 = &mut s; // mutable borrow

  println!("{} and {}", r1, r2);
}

// fn change(s: &mut String) {
//     s.push_str(", world");
// }

// fn print_length(s: &String) {
//   println!("The length is: {}", s.len());
// }