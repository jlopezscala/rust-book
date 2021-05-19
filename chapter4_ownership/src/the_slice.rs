/*

    SLICE:
    - Does not have ownership
    - Let you reference a CONTIGUOUS sequence of element in a collection

    String slice for example would be a reference to part of a String.

    String literals (stored inside the binary) are Slices

        let s = "Hello, world!";

    s is &str, a slice pointing that specific point of the binary
    &str being an immutable reference
 */


// EXAMPLE
/*
    Write a function that takes a string and returns the first word it finds in that string.
    If the function doesn’t find a space in the string, the whole string must be one word,
    so the entire string should be returned.

 */

fn first_world_main() {
    // The bad one
    {
        let mut s = String::from("hello world");

        let word = first_world_example1(&s); // word will get the value 5

        s.clear(); // this empties the String, making it equal to ""
        // word still has the value 5 here, but there's no more string that
        // we could meaningfully use the value 5 with. word is now totally invalid!

        // THIS COMPILES  -.- !! (of course, word is not connected to the state of s at all!)}
    }

    // Using slices help you prevent the above:
    // BOOM, compile error.
    {
        let mut s = String::from("hello world");

        let word = first_word_example2_slices(&s); // immutable borrow here

        s.clear(); // mutable borrow here

        println!("the first word is: {}", word); // immutable borrow used here

        // BOOM, compile error.
    }

    // The actual way of doing it..
    // Now first_word takes a string slice not a reference to a String
    {
        let my_string = String::from("hello world");

        // first_word works on slices of `String`s
        let word = first_word_example3_slices(&my_string[..]);

        let my_string_literal = "hello world";

        // first_word works on slices of string literals
        let word = first_word_example3_slices(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word_example3_slices(my_string_literal);
    }
}

fn first_world_example1(s: &String) -> usize { // Returns the index of the end of the word
    let word_in_bytes = s.as_bytes(); // Converts a string to an array of bytes
    /*
            .iter() creates an iterator that returns each of the elements in a collection
            .enumerate() returns a tuple that look like (index, &element_at_index)

            Tuple can be unpacked like (i, &item) .enumerate returns references of the items
            in the collection we use &
     */
    for (i, &item) in word_in_bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_example2_slices(s: &String) -> &str {
    let bytes_from_s = s.as_bytes();

    for (i, &item) in bytes_from_s.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]

    // Much better!
}


fn first_word_example3_slices(s: &str) -> &str { // Signature change to receive
    /*
        Now if we have a string slice we can pass it directly
        Better to define a function to take a string slice instead of a reference to a string
     */
    let bytes_from_s = s.as_bytes();

    for (i, &item) in bytes_from_s.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]

    // Even better!!
}

fn some_slice_examples() {
    let s = String::from("hello world");
    let hello = &s[0..5]; // Reference to a part of s
    let world = &s[6..11]; // 6 is index to start and 11 to include until 10 (yes, one less)

    // Following two examples are equals
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    // Another example
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..len]; // Or take all of it
    let slice = &s[..]; // with this
}