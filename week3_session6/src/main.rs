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
