use std::process::exit;
use std::io;

pub(crate) fn parse_float_input() -> f64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input).expect("Failed to read line");
    let input: f64 = match input.trim().parse() {
        Ok(input) => input,
        Err(_) => {
            println!("Please insert a number!");
            exit(1)
        }
    };
    input
}

pub(crate) fn parse_integer_input() -> u32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input).expect("Failed to read line");
    let input: u32 = match input.trim().parse() {
        Ok(input) => input,
        Err(_) => {
            println!("Please insert a number!");
            exit(1)
        }
    };
    input
}