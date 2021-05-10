fn a_function() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x);  // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward
    println!(x);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn another_function() {
    let s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
// moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned (no ;) and
    // moves out to the calling
    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) { // Its possible to declare functions that return multiple values
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn borrowing_references() {
    let s1 = String::from("hello");

    let len = calculate_length_borrow(&s1); // & is a reference, this way ownership wont change


    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length_borrow(s: &String) -> usize { // s is a reference to a String
    s.len() // No need to return the tuple to give the String back
}


fn imposible_append(s: &String) {
    some_string.push_str(", world"); // Failes at compile time cause its borrowed
}

fn change_a_reference() {
    let mut s = String::from("hello"); // Create s as mutable

    change(&mut s);


    let r1 = &mut s; // You can have only ONE mutable reference
    let r2 = &mut s; // Fails at compile time

    // Unless we put it on another scope:
    {
        let r1 = &mut s; // This works cause its on a different scope
    } // r1 goes out of scope here so we are good to reference

    let r2 = &mut s;


    // Another rule of mutability
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    let mut s = String::from("hello");

    // Changing references scopes
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // r1 and r2 are used here.
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
/* Rules:
    - At any given time, you can have either one mutable reference or
        any number of immutable references.
    - References must always be valid.

 */