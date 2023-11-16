use std::io;

// Define the Operation enum
#[derive(Debug)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Write the calculate function signature
fn calculate(operation: &Operation) -> f64 {
    // Implement the calculate function using pattern matching
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if *b != 0.0 {
                a / b
            } else {
                panic!("Division by zero is not allowed.")
            }
        }
    }
}

fn main() {
    // Prompt the user to input the first number
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let first_number: f64 = input.trim().parse().expect("Invalid input");

    // Prompt the user to input the operation
    println!("Enter the operation (+, -, *, /):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation_str = input.trim();

    // Prompt the user to input the second number
    println!("Enter the second number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let second_number: f64 = input.trim().parse().expect("Invalid input");

    // Parse the user input into appropriate variables
    let operation = match operation_str {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            panic!("Invalid operation. Supported operations are +, -, *, /.");
        }
    };

    // Call the calculate function with the created Operation enum instance
    let result = calculate(&operation);

    // Print the result to the console
    println!("Result: {}", result);
}
