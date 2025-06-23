
trait Introduce {
    fn introduce(&self);
}

struct Developer<'a> {
    name: &'a str,
    favorite_lang: &'a str,
    years_experience: u8,
}

// Implement the Introduce trait for Developer
impl<'a> Introduce for Developer<'a> {
    fn introduce(&self) {
        println!(
            "Hi, I'm {}. I love writing {} and have {} years of experience in Web3.",
            self.name, self.favorite_lang, self.years_experience
        );
    }
}
// Impl block for Developer 
impl<'a> Developer<'a> {
    fn new(name: &'a str, favorite_lang: &'a str, years_experience: u8) -> Self {
        Developer {
            name,
            favorite_lang,
            years_experience,
        }
    }

    // Function using lifetime annotation
    fn favorite_language<'b>(&'b self) -> &'b str {
        self.favorite_lang
    }
}

// Designer struct
struct Designer<'a> {
    name: &'a str,
    design_tool: &'a str,
    years_experience: u8,
}

// Implement the Introduce trait for Designer
impl<'a> Introduce for Designer<'a> {
    fn introduce(&self) {
        println!(
            "Hi, I'm {}. I craft Web3 experiences using {} with {} years of experience.",
            self.name, self.design_tool, self.years_experience
        );
    }
}

fn main() {
    let dev = Developer::new("Cynthia", "Rust", 3);
    dev.introduce();
    println!("Favorite Language: {}", dev.favorite_language());

    let designer = Designer {
        name: "Brian",
        design_tool: "Figma",
        years_experience: 4,
    };
    designer.introduce();
}