use std::io;
use rand::Rng; // Random lib

use std::cmp::Ordering; // An Enum like Result (line 31) its variants are Less, Greater, and Equal

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");


    let secret_number = rand::thread_rng().gen_range(1, 101); // gen_range is inclusive in lower exclusive on upper
    // println!("The secret number is: {}", secret_number);

    loop { // creates infinite loop

        // let creates a variable
        // mut stands for mutable
        // String::new() new is a static method (associated function) of the type String
        // Associated functions are implemented on types
        // To summarize, the let mut guess = String::new(); line
        // has created a mutable variable that is currently bound to a new, empty instance of a String.
        let mut guess = String::new();

        io::stdin()
            // read_line gets a mutable string where to store the user input in this case by keyboard
            // the string received MUST be mutable
            // & symbol indicates the argument is a reference
            // mut is also needed, by default, references are inmutable
            .read_line(&mut guess)// Result is what read_line returns from IO package
            // Result holds either OK or Err.
            // OK result contains the value in the result
            // Err will contain an error message
            // Expect is used to unpack the result. If its an Err type, it will make the program crash
            // if its an OK type, it will unpack the value and return it

            // FROM RUST DOCS -> ERROR HANDLING
            // Because this function may panic, its use is generally discouraged. Instead,
            // prefer to use pattern matching and handle the [Err] case explicitly,
            // or call unwrap_or, unwrap_or_else, or unwrap_or_default.

            .expect("Failed to read line");

        // We are not creating a new variable, guess is going to change to the new value (commonly used
        // to change variable types and not having to declare a new variable
        let guess: u32 = match guess.trim().parse() {
            // In this case, instead of using except method, we use match to
            Ok(num) => num,
            Err(_) => {
                println!("Please insert a number!");
                continue; // continues the loop
            },
        };
        match guess.cmp(&secret_number) { // similar to switch case?
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; //cuts the loop
            }
        }
    }
}