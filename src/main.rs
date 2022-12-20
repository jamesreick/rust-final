/// A simple calculator program.
use std::io;

fn main() {
    println!("Welcome to the calculator!");

    println!("Enter the first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: i32 = num1.trim().parse().expect("Please type a number!");

    println!("Enter the second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: i32 = num2.trim().parse().expect("Please type a number!");

    println!("Enter the operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation: char = operation.trim().parse().expect("Please type a valid operation (+, -, *, /)");

    let result = match operation {
        '+' => add(num1, num2),
        '-' => subtract(num1, num2),
        '*' => multiply(num1, num2),
        '/' => divide(num1, num2),
        _ => panic!("Invalid operation"),
    };

    println!("Result: {}", result);
}

/// Adds two numbers together and returns the result.
fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Subtracts two numbers and returns the result.
fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

/// Multiplies two numbers together and returns the result.
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

/// Divides two numbers and returns the result as a floating-point number.
fn divide(x: i32, y: i32) -> i32 {
    x / y
}


