fn main() {
  let s1 = "Kisumu";
  let s2 = "Bootcamp";

  println!("Longest is {}", longest(s1, s2));
//   let r;
//    {
//      let x = 5;
//      r = &x; // does not live long enough
//   }
//   println!("The value of r is: {}", r);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
   if x.len() > y.len() {
     x
   } else{
     y
   }
}