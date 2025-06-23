
trait Introduce {
    fn introduce(&self);
}

struct Developer<'a> {
    name: &'a str,
    favorite_lang: &'a str,
    years_experience: u8,
}

