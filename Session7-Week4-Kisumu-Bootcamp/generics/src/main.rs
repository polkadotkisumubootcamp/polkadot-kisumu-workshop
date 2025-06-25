// A generic function that accepts any type T,
// but T must implement the Debug trait so we can print it using {:?}
fn show_item<T: std::fmt::Debug>(item: T) {
    // Printing the item using the Debug formatter {:?}
    println!("Item: {:?}", item);
}

fn main() {
    // Calling show_item with an integer
    show_item(10);

    // Calling show_item with a string slice
    show_item("Hello");

    // Calling show_item with a floating-point number
    show_item(3.14);
}

// Example 2 using Structs

// Generic Struct
struct Container<T> {
    value: T,
}

fn main() {
    let int_container = Container { value: 100 };
    let str_container = Container { value: "Rust" };

    println!("Integer: {}", int_container.value);
    println!("String: {}", str_container.value);
}

// Example 3 Generics with Traits

// Trait that requires a describe method
trait Describe {
    fn describe(&self);
}

// Book struct implements Describe
struct Book {
    title: String,
}

impl Describe for Book {
    fn describe(&self) {
        println!("This is a book titled '{}'", self.title);
    }
}

// Car struct implements Describe
struct Car {
    model: String,
}

impl Describe for Car {
    fn describe(&self) {
        println!("This is a car model '{}'", self.model);
    }
}

// Generic function that accepts anything that implements Describe
fn print_description<T: Describe>(item: T) {
    item.describe();
}

fn main() {
    let my_book = Book { title: "Rust for Beginners".to_string() };
    let my_car = Car { model: "Tesla Model 3".to_string() };

    print_description(my_book);
    print_description(my_car);
}


