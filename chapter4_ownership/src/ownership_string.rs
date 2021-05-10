/*
    Scalar and compound types are always stored on the STACK.
    The Stack is FIFO,
 */

fn scalar_variables() {
    let random_variable: u32 = 12; // random_variable value will be stored in the Stack
}

fn create_string() {
    let mut s = String::from("hello"); // Creates a String object from a literal, in this case "hello"
    /*
        In this case S is mutable, this means the memory needed is calculated in run-time, against
        that literals that the text gets added to the compiled binary

        String is stored in the HEAP
     */

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}

fn scope_example() {
    /* ::from("hello") on runtime, requests the memory it needs
    In order to clean the memory when the variable is not used anymore, Rust returns the memory
    once the variable that owns it goes out of scope */
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // When curly bracket is closed, Rust calls `drop` function defined for Strings which
}

fn stack_vs_heap() {
    let x = 5; // 5 is added to the stack bonded to x
    let y = x; // make a copy of the value of x and bind it to y

    // Above we have two variables with the same value (5) in the stack

    let s1 = String::from("hello");
    /*
        s1 is a String, Strings are made up of three parts stored in the Stack
            - ptr: Pointer
            - len: memory in bytes the String is currently using
            - capacity: memory in bytes the String received from the allocator (in this case the ::from())
     */
    {
        let s2 = s1;
        /*
            When assign s1 to s2, the String data is copied (pointer, length and capacity) in the STACK
            The data on the heap that the pointer refers to is not copied.
        */
    }// When s2 goes out of scope Rust will call drop function on s2.
}
/*
    When s1 goes out of scope Rust drop is not called. No double free error.
    let s2 = s1; will make s1 invalid, Rust will not try to free memory when it goes out of scope
 */

fn ownership_error_example() {
    let s1 = String::from("hello");
    let s2 = s1; // Shallow copy, but s1 gets invalidated so its not copy is more like MOVE
    // here the value of s1 was moved to s2

    println!("{}, world!", s1); // This will result on an error at compile time
}

fn deep_copy_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // This will duplicate the data in the HEAP, is costly

    println!("s1 = {}, s2 = {}", s1, s2);
}

/*
    Some types implement the Copy trait, no need to use clone() method.

    Types that implement Copy:
    - All the integer types, such as u32.
    - The Boolean type, bool, with values true and false.
    - All the floating point types, such as f64.
    - The character type, char.
    - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.


 */