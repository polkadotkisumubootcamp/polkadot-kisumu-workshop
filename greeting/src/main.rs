// greetings fn
fn greet(team: &str) -> String {
    format!("Hello, Team {}!", team)
}

// main fn
fn main() {
    let message = greet("Kisumu Builders");
    println!("{}", message);
}
