// def get_weather():
//     return 30, "Sunny"

// def check_weather():
//     temperature, condition = get_weather()

//     if temperature > 28:
//         print("It's a hot day!")
//     else:
//         print("The weather is moderate.")

//     if condition == "Rainy":
//         print("Don't forget your umbrella!")
//     else:
//         print("Enjoy your day!")

// check_weather()

fn main() {
    let mut num: (i32, f32, i32) = (1, 20.45, 3); 
   println!("Tuple fisrt values is: {}", num.0);
    // println!("Sum of all parts: {:.2}", sum);

    num.2 = 45;
    // println!("Tuple third value is: {}", num.2);

    // println!("The whole tuple is: {:?}", num);

    // destructure
    let (a, b, c) = num;
    println!("Destructured values: a = {}, b = {:.2}, c = {}", a, b, c);
}