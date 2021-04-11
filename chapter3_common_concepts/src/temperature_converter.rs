use std::io;
use std::process::exit;
use crate::parsers::{parse_integer_input, parse_float_input};


pub(crate) fn convert() {
    println!("Please select an option: \n
    1- Celsius to Fahrenheit\n
    2- Fahrenheit to Celsius\n"
    );

    let option = parse_integer_input();
    if option == 1 {
        println!("Insert Celsius temperature");
        let input = parse_float_input();
        let result = (input * 9.0 / 5.0) + 32.0;
        println!("{} Celsius in Farenheit is {}", input, result)
    } else if option == 2 {
        println!("Insert Fahrenheit temperature");
        let input = parse_float_input();
        let result = (input - 32.0) * 5.0 / 9.0;
        println!("{} Fahrenheit in Celsius is {}", input, result)
    } else {
        println!("Invalid option");
        exit(1)
    }
}
