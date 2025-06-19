// fn main() {
// //     struct Developer {
// //         name: String,
// //         language: String,
// //         experience: u8,
// //     }
   
// // let dev: Developer = Developer{
// //    name: String::from("Cheryl"),
// //    language: String::from("Rust"),
// //    experience: 5,

// // };
// // println!("{} code in {}  with {} years of experience", dev.name, dev.language, dev.experience)

// // struct Developer2 {
// //     name: String,
// //     language: String,
// //     experience: u8,
// // }

// // //  Helper function that creates instance of Developer
// // fn create_developer(name: &str, language: &str, experience:)

// struct Designer{
//     name:String,
//     tool: String,
// }

// struct Developer{
//     name: String,
//     language: String,
// }

// let dev: Developer = Developer{
//     name: String::from("Hassan"),
//     language: String::from("Rust"),
// };

// let designer: Designer = Designer {
//     name: String::from("Bob"),
//     tool: String::from("Figma"),
// };

// println!("{} codes in {} language", dev.name, dev.language);
// println!("{} designs using {} tool", designer.name, designer.tool);


// }

// Implementation:

fn main() {
    struct Developer{
        name: String,
        experience: u8,
    }

    impl Developer {
        fn summary(&self) -> String {
            format!("{} has {} years of experience", self.name, self.experience)

        }
    }
    let dev: Developer = Developer {
        name:String::from("Bob"),
        experience: 5,
    };
    println!("{}", dev.summary())
}

