// Rust 101: Basic Program
fn main() {
    // 1. Variables and Mutability
    // Immutable variable by default
    let x = 5;
    println!("x is: {}", x);

    // Mutable variable (use `mut`)
    let mut y = 10;
    y = y + 2;
    println!("y is now: {}", y);

    // Constants (must be annotated with type)
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);

    // 2. Data Types
    let is_active: bool = true;
    let score: i32 = -42;
    let name: &str = "Rustacean";
    println!("Status: {}, Score: {}, Name: {}", is_active, score, name);

    // 3. Functions
    let sum = add_numbers(3, 7);
    println!("Sum from function: {}", sum);

    // 4. Control Flow
    if score < 0 {
        println!("Negative score!");
    } else {
        println!("Positive or zero score!");
    }

    // Loop with break and return value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2;
        }
    };
    println!("Loop result: {}", result);

    // 5. Ownership and Borrowing
    let s1 = String::from("Hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    println!("s2: {}", s2);

    // Borrowing with reference
    let s3 = String::from("World");
    let len = calculate_length(&s3); // Pass reference, no ownership transfer
    println!("{} has length: {}", s3, len);
}

// Function definition
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b // No semicolon = return value
}

// Function using borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
}
