// Enum for professional roles - adds type safety
#[derive(Debug, Clone)]
enum Role {
    Developer,
    Designer,
}

// Enhanced trait with an additional useful method
trait Introduce {
    fn introduce(&self);
    fn get_experience(&self) -> u32;
    fn get_role(&self) -> Role;

    // Default implementation for experience level
    fn experience_level(&self) -> &'static str {
        match self.get_experience() {
            0..=2 => "Junior",
            3..=7 => "Mid-level",
            _ => "Senior"
        }
    }
}

// Represents a Web3 developer
#[derive(Debug, Clone)]
struct Developer {
    name: String,
    favorite_lang: String,
    years_experience: u32,
}

impl Developer {
    fn new(name: String, favorite_lang: String, years_experience: u32) -> Self {
        Self { name, favorite_lang, years_experience }
    }
}

// Represents a Web3 designer
#[derive(Debug, Clone)]
struct Designer {
    name: String,
    favorite_tool: String,
    years_experience: u32,
}

impl Designer {
    fn new(name: String, favorite_tool: String, years_experience: u32) -> Self {
        Self { name, favorite_tool, years_experience }
    }
}

// Trait implementations with enhanced introductions
impl Introduce for Developer {
    fn introduce(&self) {
        println!(
            "Hi, I'm {}, a {} developer with {} years of experience. My favorite language is {}.",
            self.name, self.experience_level(), self.years_experience, self.favorite_lang
        );
    }

    fn get_experience(&self) -> u32 {
        self.years_experience
    }

    fn get_role(&self) -> Role {
        Role::Developer
    }
}

impl Introduce for Designer {
    fn introduce(&self) {
        println!(
            "Hello, I'm {}, a {} designer with {} years of experience. My favorite tool is {}.",
            self.name, self.experience_level(), self.years_experience, self.favorite_tool
        );
    }

    fn get_experience(&self) -> u32 {
        self.years_experience
    }

    fn get_role(&self) -> Role {
        Role::Designer
    }
}

// Actually compares experience and returns the most experienced person
fn get_most_experienced<'a>(
    person1: &'a dyn Introduce,
    person2: &'a dyn Introduce,
) -> &'a dyn Introduce {
    if person1.get_experience() >= person2.get_experience() {
        person1
    } else {
        person2
    }
}

// Bonus: Simple team function demonstrating collections
fn introduce_team(team: &[&dyn Introduce]) {
    println!("\n🚀 Team Introduction:");
    for member in team {
        member.introduce();
    }
}

fn main() {
    // Create instances using constructor methods
    let dev = Developer::new(
        "Vincent".to_string(),
        "Rust".to_string(),
        5
    );

    let designer = Designer::new(
        "Cherypic".to_string(),
        "Figma".to_string(),
        3
    );

    let senior_dev = Developer::new(
        "Allan".to_string(),
        "Solidity".to_string(),
        8
    );

    // Individual introductions
    dev.introduce();
    designer.introduce();
    senior_dev.introduce();

    // Demonstrate actual comparison with lifetimes
    println!("\nMost experienced between Allan and Cherypic:");
    let most_experienced = get_most_experienced(&dev, &designer);
    most_experienced.introduce();

    // Team demonstration
    let team = vec![&dev as &dyn Introduce, &designer, &senior_dev];
    introduce_team(&team);
}