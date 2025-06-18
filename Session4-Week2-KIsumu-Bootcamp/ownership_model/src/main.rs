fn main() {
    // Way 1 
    // let x = String::from("polkadot");
    // let y = x ; //ownership moves to y

    // // println!("{}", x) ; // this will fail since ownership has been moved

    // println!("{}", y) ; // this works
    
    // Way 2
    // let s1 = String::from("kisumu");
    // let s2 = s1.clone();

    // println!("s1: {}, s2: {}", s1, s2) //both work because of clone

    // Way 3 
   let s = String::from("bootcamp");
   take_ownership(s);
//    println!("{}", s);
}

fn take_ownership(s: String) {
    println!("Got S: {}", s);
}