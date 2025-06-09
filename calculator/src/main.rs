// 1/50 Simple CLI Calculator Project
// Basic calculator performing basic arithmetic calculation
// Project the takes two numbers from user and operator the calculate it and display result 

// Techical Detail
// I used std:io module to read user input and write 
// the main function(entry to the projects)
// then we have main loo to make users to perform multple calculations
// the get_num function to get/read f64(to handle floating numbers)too or we can use i32 but it handles only integers
// get_op function to get/red oprator as a character because i we you know +,-,* and / are special charaters, 
// calculate function to perform arthimetic ops using imputs from user which is num1,num2 and opp
// finally error handling refer other material for that
// thanks





use std::io;

fn main() {
    println!("1/50 Simple CLI Calculator Project");
    println!("Enter two numbers and an operation (e.g., +, -, *, /)");

    loop {
        println!("Enter the first number:");
        let num1 = match get_num() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please try again.");
                continue;
            }
        };

        println!("Enter the operation (+, -, *, /):");
        let op = match get_operator() {
            Ok(op) => op,
            Err(_) => {
                println!("Invalid operator. Please use +, -, *, or /.");
                continue;
            }
        };

        println!("Enter the second number:");
        let num2 = match get_num() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please try again.");
                continue;
            }
        };

        let result = match calculate(num1, num2, op) {
            Ok(res) => res,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        println!("Result: {} {} {} = {}", num1, op, num2, result);

        println!("Do you want to perform another calculation? (y/n)");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        if choice.trim().to_lowercase() != "y" {
            break;
        }
    }
    // println!("Thank you for using the calculator!");
}

fn get_num() -> Result<f64, std::io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse().expect("Not a valid number"))
}

fn get_operator() -> Result<char, std::io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let op = input.trim();
    if op.len() == 1 && "+-*/".contains(op) {
        Ok(op.chars().next().unwrap())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid operator"))
    }
}

fn calculate(num1: f64, num2: f64, op: char) -> Result<f64, &'static str> {
    match op {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => {
            if num2 == 0.0 {
                Err("Division by zero is not allowed")
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Invalid operator"),
    }
}
