// We define a function that safely divides two numbers
fn safe_divide(dividend: i32, divisor: i32) -> Result<i32, String> {
    // 🧠 First check if the divisor is zero
    if divisor == 0 {
        // ❌ If it's zero, return an Err with a message
        Err("Can't divide by zero!".to_string())
    } else {
        // ✅ If it's safe, return Ok with the result
        Ok(dividend / divisor)
    }
}

fn main() {
    // 🧪 Try dividing 10 by 2 — this should work
    let result1 = safe_divide(10, 2);

    // 🧪 Try dividing 10 by 0 — this should fail
    let result2 = safe_divide(10, 0);

    // 🔍 Let's check result1 using match
    match result1 {
        Ok(value) => println!("✅ Success! Result is: {}", value),
        Err(msg) => println!("❌ Error: {}", msg),
    }

    // 🔍 Check result2 — will enter the Err arm
    match result2 {
        Ok(value) => println!("✅ Success! Result is: {}", value),
        Err(msg) => println!("❌ Error: {}", msg),
    }
}
