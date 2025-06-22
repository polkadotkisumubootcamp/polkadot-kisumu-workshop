#[derive(Debug)]
struct Developer<'a> {
    name: String,
    favorite_lang: &'a str,
    years_experience: u8,
}

#[derive(Debug)]
struct Designer {
    name: String,
    specializes_in: String,
    tools: Vec<String>,
}

trait Introduce {
    fn introduce(&self);
}

impl<'a> Developer<'a> {
    fn new(name: String, favorite_lang: &'a str, years_experience: u8) -> Self {
        Developer {
            name,
            favorite_lang,
            years_experience,
        }
    }

    fn introduce(&self) {
        println!(
            "I'm {}, a Web3 dev. I love {} and have {} years of exp",
            self.name, self.favorite_lang, self.years_experience
        );
    }
}

impl Designer {
    fn new(name: String, specializes_in: String, tools: Vec<String>) -> Self {
        Designer {
            name,
            specializes_in,
            tools,
        }
    }
}

impl<'a> Introduce for Developer<'a> {
    fn introduce(&self) {
        println!(
            "Meet {} ({} dev, {} yrs exp)",
            self.name, self.favorite_lang, self.years_experience
        );
    }
}

impl Introduce for Designer {
    fn introduce(&self) {
        println!(
            "{} here! I specialize in {} and work with {:?}",
            self.name, self.specializes_in, self.tools
        );
    }
}

fn get_favorite_lang<'a>(dev: &'a Developer) -> &'a str {
    dev.favorite_lang
}

fn main() {
    let lang = "Rust";
    let dev = Developer::new("Kherld".to_string(), lang, 5);

    dev.introduce();

    Introduce::introduce(&dev);

    let designer = Designer::new(
        "Nelly".to_string(),
        "NFT Art".to_string(),
        vec!["Figma".to_string(), "Blender".to_string()],
    );

    designer.introduce();

    let fav_lang = get_favorite_lang(&dev);
    println!("\n Favorite lang (via lifetime function): {}", fav_lang);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_developer_intro() {
        let dev = Developer::new("Nerd".to_string(), "Solidity", 3);
        assert_eq!(dev.name, "Nerd");
        assert_eq!(dev.favorite_lang, "Solidity");
    }

    #[test]
    fn test_lifetime_function() {
        let dev = Developer::new("Kherld".to_string(), "Rust", 2);
        assert_eq!(get_favorite_lang(&dev), "Rust");
    }
}
