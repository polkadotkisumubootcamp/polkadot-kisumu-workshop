
//Option

// fn main() {
//     let snack = Some("cookie");

//     match snack {
//         Some(food) => println!("You'll get some Cookies"),
//         None => println!("No snack today"),

//     }

//     let empty_box : Option<&str> = None;
//    match empty_box {
//        Some(food) => println!("Yummy! {}", food),
//         None => println!("sorry, it's empty"),
//     }
// }


// Options example 2

fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Can't divide by zero".to_string())
    } else{
        Ok(a/b)
    }
}

fn main() {
    let result1 = safe_divide(10,2);
    let result2 = safe_divide(10,0);

    match result2 {
        Ok(value) => println!("Success, result is {}", value),
        Err(msg) => println!("Error: {}", msg),
    }
}
