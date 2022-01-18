use std::io;

fn main() {
    println!("Welcome to this wonderful temperature convertion app!");
    println!("Do you want to convert Celsius or Fahrenheit? You are in the right place!");
    println!("Input C to convert Celsius or input F to convert Fahrenheit");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: &str = input.trim();

    match input {
        "F" => println!("Input the number of Fahrenheit you want to convert"),
        "C" => println!("Input the number of Celsius you want to convert"),
        _ => println!("Wrong input! Try again!")
    }

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read number");

    let number: u32 = number.trim().parse();

    println!(number);

    

}
