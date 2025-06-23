// 1) Define a Trait `Introduce` with a single method `introduce`
trait Introduce {
    fn introduce(&self);
}

// 2) Create two structs: `Developer` and `Designer`
struct Developer {
    name: String,
    favorite_lang: String,
    years_experience: u32,
}

struct Designer {
    name: String,
    specialty: String,
    years_experience: u32,
}

// 3) Implement the `Introduce` trait for `Developer`
impl Introduce for Developer {
    fn introduce(&self) {
        println!(
            "Hi, I’m {}! I’ve been coding in {} for {} years.",
            self.name, self.favorite_lang, self.years_experience
        );
    }
}

// 4) Implement the `Introduce` trait for `Designer`
impl Introduce for Designer {
    fn introduce(&self) {
        println!(
            "Hello, I’m {}! I design {} interfaces and have {} years of experience.",
            self.name, self.specialty, self.years_experience
        );
    }
}

// 5) (Bonus) A function that returns a reference into one of our structs
//    We need a lifetime annotation `'a` to prove the returned &str
//    won’t outlive the data it borrows from.
fn get_name_ref<'a, T>(person: &'a T) -> &'a str
where
    T: IntroduceName, // see below
{
    person.name()
}

// To make the bonus function generic, we extract a tiny helper trait:
trait IntroduceName {
    fn name(&self) -> &str;
}

impl IntroduceName for Developer {
    fn name(&self) -> &str {
        &self.name
    }
}

impl IntroduceName for Designer {
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    // Instantiate a Developer
    let dev = Developer {
        name: "Clinton".into(),
        favorite_lang: "Rust".into(),
        years_experience: 2,
    };

    // Instantiate a Designer
    let designer = Designer {
        name: "Bob".into(),
        specialty: "UX/UI".into(),
        years_experience: 5,
    };

    // Call their `introduce` methods
    dev.introduce();
    designer.introduce();

    // Bonus: get a &str reference to their name
    let dev_name_ref = get_name_ref(&dev);
    let designer_name_ref = get_name_ref(&designer);
    println!("Developer’s name (via ref): {}", dev_name_ref);
    println!("Designer’s name (via ref): {}", designer_name_ref);
}
