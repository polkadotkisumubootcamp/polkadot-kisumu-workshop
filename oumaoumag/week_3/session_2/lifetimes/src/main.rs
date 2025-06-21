// // Takes two string and returns the longer
// fn longest<'a>(b: &'a str, c: &'a str) -> &'a str {
//     if b.len() > c.len() {
//         b
//     } else {
//         c
//     }
// }

// fn main() {
//     let string1 = String::from("Godwin");
//     let string2 = "ouma-1";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);

//     let string3 = String::from("long string is long");
//     let result2;
//     {
//         // let string4 = String::from("ouma-2-ouma-am strill godwin ouma and very long -> ouma");
//         let string4 = String::from("ouma-ouma-hfhlfhagllaflahlflghlahghlaghhlghhg");
//          result2 = longest(string3.as_str(), string4.as_str());
//         println!("The longest string is {}", result2);
//     }
//     println!("The longest string is {}", result2);

// }

// // Takes two string and returns the longer
// fn longest(b: &str, c: &str) -> &str {
//     if b.len() > c.len() {
//         b
//     } else {
//         c
//     }
// }

// fn main() {
//     let string1 = String::from("Godwin");
//     let string2 = "ouma-1";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);

//     let string3 = String::from("long string is long");
//     {
//         let string4 = String::from("ouma-2-ouma-am strill godwin ouma and very long -> ouma");
//         let result = longest(string3.as_str(), string4.as_str());
//         println!("The longest string is {}", result);
//     }
//     println!("The longest string is {}", result);

// }

fn longest<'a>(b: &'a str, c: &'a str) -> &'a str {
    if b.len() > c.len() {
        b
    } else {
        c
    }
}

fn main() {
    let string1 = String::from("Godwin");
    let string2 = "ouma-1";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string3 = String::from("long string is long");
    {
        let string4 = String::from("ouma-2-ouma-am strill godwin ouma and very long -> ouma");
        let result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    }
    println!("The longest string is {}", result);

}
