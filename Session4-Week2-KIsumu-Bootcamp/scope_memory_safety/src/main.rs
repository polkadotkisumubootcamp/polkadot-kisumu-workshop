fn main(){
  // Scope ends
//   let s = String:: from("admin");
//   println!("{}", s);
  let r = dangle();
  println!("The dangle reference is: {}", r); // Unsafe and Rust will prevent it at compile time.Will catch it using a borrower checker.
}
// println!("{}", s); // s is out of scope

// dangling reference

fn dangle() -> &String {
    let s = String::from("buju");
    &s
}