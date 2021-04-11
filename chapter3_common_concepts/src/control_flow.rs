
// Rust Control Flow

// IF expressions
fn if_statements() {
    let number = 3;

    if number < 5 { // Condition must be a bool
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn else_if_statements() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_and_let() {

    let condition = true;
    let number = if condition { 5 } else { 6 }; // Code blocks will return the last expression!

    println!("The value of number is: {}", number);
}

// LOOP expressions

fn loop_example() {
    loop { // loop makes the block execute infinite times or until break expression.
        println!("again!");

    }
}

fn break_and_return_example() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // breaks the loop and returns the value of counter * 2 and store
                                // the result in the result variable
        }
    };
    println!("The result is {}", result);
}

// WHILE expressions

fn while_example() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// FOR expression
fn for_example() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}