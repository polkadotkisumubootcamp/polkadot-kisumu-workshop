
struct Developer{
    name: String,
    favorite_lang:String,
    years: u32,
}
 trait Introduce {
    fn introduce(&self);
 }

// Impl Block for Developer.Implements a constructor method new() 
// to easily create a Developer instance.
// Converts &str to String to match the struct types.

 impl Developer{
    fn new (name:&str, favorite_lang:&str, years:u32) -> Self {
        Developer{
        name:name.to_string(),
        favorite_lang:favorite_lang.to_string(),
        years:years,
    }
    }
 }
//  Trait Implementation for Developer
// Implements the Introduce trait for the Developer struct.
// This method prints a customized intro when called.

impl Introduce for Developer {
    fn introduce(&self){
        println!("Hello, my name is {} and I am a developer who has been coding for {} years. My favorite language is {}", self.name, self.years, self.favorite_lang);
    }
}

 
struct Designer{
    name:String,
    favorite_tool:String,
    projects:u32,
}

impl Designer {
    fn new(name:&str, favorite_tool:&str, projects:u32) ->Self{
        Designer{
            name:name.to_string(),
            favorite_tool:favorite_tool.to_string(),
            projects:projects,
        }
    }
}

impl Introduce for Designer {
    fn introduce(&self) {
        println!("Hello, my name is {} and I am a designer who has been designing for {} projects. My favorite tool is {}", self.name, self.projects, self.favorite_tool);
    }
}

// Lifetimes illustration for the longest between designer and developer

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {

    let dev = Developer::new("Alice", "Rust", 4);
    let designer = Designer::new("Bob", "Figma", 12);

    dev.introduce();
    designer.introduce();

    let intro1 = "Alice loves Rust";
    let intro2 = "Bob is a creative Figma wizard";
    let longer = longest(intro1, intro2);
    println!("📢 Longer intro: {}", longer);
}
