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
