use crate::parsers::{parse_integer_input};


pub(crate) fn fibonacci() {
    println!("Insert a number");
    let input: u32 = parse_integer_input();

    let mut result: u32 = 0;

    if input == 0 {
        println!("Result is 0");
    } else if input == 1 {
        println!("Result is 1");
    } else {
        result = (input - 1) + (input - 2);
        print!("Result is {}. ", result);
    }
}