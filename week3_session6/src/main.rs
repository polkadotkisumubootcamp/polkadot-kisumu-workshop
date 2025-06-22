// Web3 Profile System in Rust

// Defining the Introduce trait
trait Introduce {
    fn introduced(&self);
}

// Defining a Developer struct
struct Developer {
    name: String,
    favorite_lang: String,
    years_experience: u8,
}

// Implementing the Introduce trait for Developer
impl Introduce for Developer {
    fn introduced(&self) {
        println!(
            "Hi, I'm {}, a Web3 Developer who loves building on {} with {} years of experience!",
            self.name, self.favorite_lang, self.years_experience
        );
    }
}

// Defining a Designer struct
struct Designer {
    name: String,
    favorite_tool: String,
    focus_area: String,
}

// Implementing the Introduce trait for Designer
impl Introduce for Designer {
    fn introduced(&self) {
        println!(
            "Hello! I'm {}, a Web3 Designer focused on {} using {}.",
            self.name, self.focus_area, self.favorite_tool
        );
    }
}

// Bonus: Function using lifetimes that returns the name that comes first alphabetically
fn first_alphabetically<'a>(name1: &'a str, name2: &'a str) -> &'a str {
    if name1 < name2 {
        name1
    } else {
        name2
    }
}