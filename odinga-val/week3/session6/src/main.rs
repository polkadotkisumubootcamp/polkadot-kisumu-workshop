// Define a trait for introduction functionality
trait Introduce {
    fn introduce(&self);
}

// Developer struct
struct Developer<'a> {
    name: &'a str,
    favorite_language: &'a str,
    years_experience: u8,
}

impl<'a> Introduce for Developer<'a> {
    fn introduce(&self) {
        println!(
            "Gm!Gm! my name is {} I am a {} dev and I have {} years of Web3 experience.",
            self.name, self.favorite_language, self.years_experience
        );
    }
}

// Designer struct
struct Designer<'a> {
    name: &'a str,
    favorite_tool: &'a str,
    years_experience: u8,
}

impl<'a> Introduce for Designer<'a> {
    fn introduce(&self) {
        println!(
            "Hey! I'm {} and I specialize in {}. I've been designing for Web3 for {} years.",
            self.name, self.favorite_tool, self.years_experience
        );
    }
}

// Enum to represent either professional
enum Web3Professional<'a> {
    Developer(Developer<'a>),
    Designer(Designer<'a>),
}

// Implement Introduce for the enum
impl<'a> Introduce for Web3Professional<'a> {
    fn introduce(&self) {
        match self {
            Web3Professional::Developer(dev) => dev.introduce(),
            Web3Professional::Designer(designer) => designer.introduce(),
        }
    }
}

// Function that returns the enum (no lifetime annotation needed in return type)
fn get_web3_professional<'a>(
    choice: &str,
    name: &'a str,
    tool: &'a str,
    years: u8,
) -> Web3Professional<'a> {
    match choice {
        "developer" => Web3Professional::Developer(Developer {
            name,
            favorite_language: tool,
            years_experience: years,
        }),
        "designer" => Web3Professional::Designer(Designer {
            name,
            favorite_tool: tool,
            years_experience: years,
        }),
        _ => panic!("Invalid profession"),
    }
}

fn main() {
    // Create instances through the function
    let dev = get_web3_professional("developer", "James", "Rust", 3);
    let designer = get_web3_professional("designer", "Bronia", "Figma", 5);
    
    // Use the introduce method
    dev.introduce();
    designer.introduce();
    
    // Create directly
    let carol = Web3Professional::Developer(Developer {
        name: "Carol",
        favorite_language: "Solidity",
        years_experience: 2,
    });
    carol.introduce();
}