fn main() {
    // Lifetime
        // let s1: String = String::from("short");
        // let s2: String = String::from("longer");
        let s1: String = String::from("Allan");
        let s2: String = String::from("Kamau");

        // let result: &str = longest(a: &s1, b: &s2);
        let result = longest(&s1, &s2); 
        println!("The longest string is {}", result)
}

fn longest<'a> (a: &'a str, b: &'a str) -> &'a str {
  
    if a.len() > b.len() {
        a
    } else {
        b
    }


}