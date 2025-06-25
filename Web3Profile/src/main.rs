// ===============================
// Web3 Profile System in Rust
// ===============================

trait Introduce {
    fn introduce(&self);
}

// ========== Structs ==========
struct Developer<'a> {
    name: &'a str,
    favorite_lang: &'a str,
    years_experience: u32,
}

struct Designer<'a> {
    name: &'a str,
    tool: &'a str,
    years_experience: u32,
}

// ========== Impl Blocks ==========
impl<'a> Developer<'a> {
    fn introduce(&self) {
        println!(
            "👨‍💻 Hi, I'm {}, a developer with {} years of experience using {}!",
            self.name, self.years_experience, self.favorite_lang
        );
    }
}

impl<'a> Designer<'a> {
    fn introduce(&self) {
        println!(
            "🎨 Hey, I'm {}, a designer with {} years of experience using {}!",
            self.name, self.years_experience, self.tool
        );
    }
}

// ========== Trait Implementations ==========
impl<'a> Introduce for Developer<'a> {
    fn introduce(&self) {
        self.introduce();
    }
}

impl<'a> Introduce for Designer<'a> {
    fn introduce(&self) {
        self.introduce();
    }
}

// ========== Lifetime Bonus Function ==========
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ========== Main ==========
fn main() {
    let dev = Developer {
        name: "Alice",
        favorite_lang: "Rust",
        years_experience: 5,
    };

    let designer = Designer {
        name: "Bob",
        tool: "Figma",
        years_experience: 3,
    };

    dev.introduce();
    designer.introduce();

    // Trait usage
    let people: Vec<&dyn Introduce> = vec![&dev, &designer];
    for person in people {
        person.introduce();
    }

    // Lifetime demo
    let a = String::from("blockchain");
    let b = String::from("web3");
    let result = longest(&a, &b);
    println!("📏 The longest string is: {}", result);
}
