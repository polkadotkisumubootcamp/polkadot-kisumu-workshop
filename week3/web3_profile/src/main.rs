
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