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

    let input_number: i32 = number.trim().parse::<i32>().unwrap();

    fn fah_to_cel(fah: i32) -> i32 {
        let cel = (fah - 32) * 5 / 9;
        cel
    }
    fn cel_to_fah(cel: i32) -> i32 {
        let fah: i32 = (cel * 9 / 5) + 32; 
        fah
    }

    match input {
        "F" => {
            let result = fah_to_cel(input_number);
            println!("{}°F converted to {}°C", input_number, result)
        }
        "C" => {
            let result = cel_to_fah(input_number);
            println!("{}°C converted to {}°F", input_number, result)
        }
        _ => println!("Something went wrong")
    }

    

}
