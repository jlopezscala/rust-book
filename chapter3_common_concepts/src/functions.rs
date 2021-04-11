// Use snake case for declaring functions and variable names

fn some_function() {
    println!("Hello, world!");

    another_function(5, 9); // Order of declaration does NOT matter to Rust compiler
}

fn another_function(x: i32, y: i32) { // Use variables in parenthesis to add arguments/parameters to your functions
    // Type must be declared on parameters

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn statements_and_expressions() {

    // This is a statement
    let y = 6;
    // Statements do not return any value
    // let x = (let y = 6); -> this PANICS, let does not return anything

    let y = {  // This is an expression (the {} block)
        let x = 3;
        x + 1 // -> this line has no ; at the end, meaning it will return the value. Adding
        // the ; will make this a statement
    };
    println!("The value of y is: {}", y);
}


fn functions_with_return_values() {
    let x = five();
    println!("The value of x is: {}", x);
}

fn five() -> i32 { // Adding the -> i32 makes this function return a i32 element
    5 // Functions return the value of the final expression (no ; at the end and last line of the
    // function body.
    // Return statement can be used to make the function return early by specifying the type.
}

fn another_example() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}