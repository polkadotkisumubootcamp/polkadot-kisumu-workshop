// Generic function to print any list
fn print_list<T: std::fmt::Display>(list: &[T]) {
    for (i, item) in list.iter().enumerate() {
        println!("{}: {}", i + 1, item);
    }
}
