fn greet(team: &str) -> String {
    format!("Hello, Team {}!", team)
}

fn main() {
    let message = greet("Kisumu Builders");
    println!("{}", message);
}
