fn main() {
    // Explicit way

    let mut num: (i32, f64, i32) = (10, 20.5, 30);

    // access individual elements
    println!("The third element: {}", num.2);
    num.2 = 45;
    println!("Third element after modification: {}", num.2);
    println!("The whole tupple is: {:?}", num);

    // implicit way
    // let data = ("polkadot", true);


    // Destructuring a tupple
    let (mut a, b, c) = num;
    a = 200; // modifying the value of b
    println!("The value of a is: {}, The value of b is: {}, The value of c is: {}", a,b,c)


}
