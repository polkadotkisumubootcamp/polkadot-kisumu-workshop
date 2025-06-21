// enum Direction {
//     North,
//     South,
//     East,
//     West,
// }

// fn main() {
//     let dir = Direction::East;

//     match dir {
//         Direction::North => println!("Go North"),
//         Direction::South => println!("Go South"),
//         Direction::East => println!("Go East"),
//         Direction::West => println!("Go West"),
//     }
// }

// //1. Define the enum:
// enum Mood {
//     Happy,
//     Angry,
//     Sleepy,
// }


// fn main() {
// //2. Use the enum:
//     let my_mood = Mood::Sleepy; // This is how you create an instance of the Mood enum
//    // 3. Check what it is (match it):

//     match my_mood {
//         Mood::Happy => println!("Yay! Let's play!"), // if it's happy, print this
//         Mood::Angry => println!("Grrr... leave me alone."),  // if it's angry, print this
//         Mood::Sleepy => println!("Zzz... need nap."), // if it's sleepy, print this
//     } 
    
// }


// // The `derive` attribute automatically creates the implementation
// // required to make this `enum` printable with `fmt::Debug`.
// #[derive(Debug)]

// enum Month_name {
// January,
// February,
// March,
// April,
// May,
// June,
// July,
// August,
// September,
// October,
// November,
// December
// }
// fn main() {
//     let jan = Month_name :: January;
//     let feb = Month_name :: February;
//     let mar = Month_name :: March;
//     let apr = Month_name :: April;
//     let may = Month_name :: May;
//     let jun = Month_name :: June;
//     let jul = Month_name :: July;
//     let aug = Month_name :: August;
//     let sep = Month_name :: September;
//     let oct = Month_name :: October;
//     let nov = Month_name :: November;
//     let dec = Month_name :: December;

//    println!("{:?}",jan);
//    println!("{:?}",feb);
//    println!("{:?}",dec);
// }