
struct developer{
    name: String,
    favorite_lang:String,
    years: u32,
}
 trait introduce {
    fn introduce(&self);
 }

// Impl Block for Developer.Implements a constructor method new() 
// to easily create a Developer instance.
// Converts &str to String to match the struct types.

 impl developer{
    fn new (name:&str, favorite_lang:&str, years:u32) -> Self {
        developer{
        name:name.to_string(),
        favorite_lang:favorite_lang.to_string(),
        years:years,
    }
    }
 }
//  Trait Implementation for Developer
// Implements the Introduce trait for the Developer struct.
// This method prints a customized intro when called.

impl introduce for developer {
    fn Introduce(&self){
        println!("Hello, my name is {} and I am a developer who has been coding for {} years. My favorite language is {}", self.name, self.years, self.favorite_lang);
    }
}

 
struct designer{
    name:String,
    favorite_tool:String,
    projects:u32,
}

impl designer {
    fn new(name:&str, favorite_tool:&str, projects:u32) ->Self{
        designer{
            name:name.to_string(),
            favorite_tool:favorite_tool.to_string(),
            projects:projects,
        }
    }
}
 impl introduce for designer {
    fn introduce (&self){
        println!("Hello, my name is {} and I am a designer who has been designing for {} years. My favorite tool is {}", self.name, self.projects, self.favorite_tool);
    }
 }
  // Lifetimes illustration for the longest between designer and developer

  fn longest <'a>(x:'a &str, y:'a &str) -> & 'a str {
    if x.len() > y.len() {
        x
    }else{
        y
    }
  }