fn main() {
     // We create a vector of numbers using the 'vec!' macro.
   let numbers = vec![10, 20, 30, 40, 50];
   
   // We create a slice from the vector, starting at index 1 up to but not including index 4.
    // This slice will contain [20, 30, 40]

   let slice = &numbers[1..4];

   println!("Slice of numbers: {:?}", slice);

    // We create a String type. String in Rust is growable and mutable.
   let sentence = String::from("Rust is awesome!");
    
    // We take a slice of the string from index 0 to index 4 (not including 4).
    // This extracts the word "Rust"
   let word = &sentence[0..4];

    // Print the sliced part of the string.
   println!("The first word is: {}", word);
}