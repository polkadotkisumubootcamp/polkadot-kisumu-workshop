fn main(){
    let age = 28 ;
    
    // Check if the user is underage
    if age < 18 {
        println!("You're a minor and can't be allowed to the party");
    } 
    // Check if the user is exactly 18
    else if age == 18 {
        println!("Just turned 18? You're right on time for the party!");
    }
    // Everyone else is older and welcome
    else {
        println!("You're more than welcome, come on in!");
    }
} 