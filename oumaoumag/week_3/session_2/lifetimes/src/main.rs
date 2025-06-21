// Takes two string and returns the longer
fn longest<'a>(b: &'a str, c: &'a str) -> &'a str {
    if b.len() > c.len() {
        b
    } else {
        c
    }
}
