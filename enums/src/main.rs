enum Mood {
    Happy,
    Angry,
    Sleepy,
}

fn main() {
    let my_mood: Mood = Mood::Sleepy;

    match my_mood {
        Mood::Happy => println!("I am happy"),
        Mood::Angry => println!("I am angry"),
        Mood::Sleepy => println!("I am heading to bed"),
    }
}

