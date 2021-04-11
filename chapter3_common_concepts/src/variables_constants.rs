// Rust is statically typed. All types must be known at compile time


/* Constants are always immutable, this can't be changed
 Type MUST be specified for declaring constants
 Constants are use for global access too */
const MAX_POINTS: u32 = 100_000;

fn variables() {
    // Variables are immutable by default
    // By adding mut the variable becomes mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // This assignation can be only done if x is mutable
    println!("The value of x is: {}", x);
}

fn shadowing() {
    let x = 5;

    // By saying let again, x is "shadowed".
    let x = x + 1;

    /* By using let, we can perform a few transformations on a value but have the variable
    be immutable after those transformations have been completed.
    we’re effectively creating a new variable when we use the let keyword again,
    we can change the type of the value but reuse the same name. */
    let x = x * 2;

    println!("The value of x is: {}", x);
}
