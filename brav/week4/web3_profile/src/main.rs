// Define a trait for types that can introduce themselves.
trait Introduce {
    fn introduce(&self);
}

// Represents a Web3 developer 
struct Developer {
    name: String,
    favorite_lang: String,
    years_experience: u32,
}

// Implements the `Introduce` trait for the `Developer` struct
impl Introduce for Developer {
    fn introduce(&self) {
        println!(
            "Hi, I'm {}, a developer with {} years of experience. My favorite language is {}.",
            self.name, self.years_experience, self.favorite_lang
        );
    }
}

// Represents a Web3 designer
struct Designer {
    name: String,
    favorite_tool: String,
    years_experience: u32,
}

// Implements the `Introduce` trait for the `Designer` struct, providing a custom introduction.
impl Introduce for Designer {
    fn introduce(&self) {
        println!(
            "Hello, I'm {}. I've been a designer for {} years and my favorite tool is {}.",
            self.name, self.years_experience, self.favorite_tool
        );
    }
}

// lifetime annotations.
fn get_most_experienced<'a>(
    person1: &'a dyn Introduce,
    person2: &'a dyn Introduce,
) -> &'a dyn Introduce {
   //this is just an example
    person1
}

fn main() {
    // Create an instance of a Developer.
    let dev = Developer {
        name: String::from("Test User"),
        favorite_lang: String::from("Rust"),
        years_experience: 5,
    };

    // Create an instance of a Designer.
    let designer = Designer {
        name: String::from("Jane Doe"),
        favorite_tool: String::from("Figma"),
        years_experience: 3,
    };

    // Call the `introduce` method on both the developer and designer instances.
    dev.introduce();
    designer.introduce();

    // Demonstrate the use of the `get_most_experienced` function with lifetime annotations.
    let most_experienced = get_most_experienced(&dev, &designer);
    println!("lifetime example");
    most_experienced.introduce();
}