// Rust has scalar and compound types

// SCALAR Types
/*
    - Integer
        Length	Signed	Unsigned
        8-bit	i8	u8
        16-bit	i16	u16
        32-bit	i32	u32
        64-bit	i64	u64
        128-bit	i128	u128
        arch	isize	usize

        Signed and unsigned refer if its possible to be negative.
        Signed integers allow negative signs. (+ or -)
        Unsigned are always positive, do not include the sign

        Compiling on debug mode, Rust checks for integer overflow (i.e u8 > 255)
        Compiling on release mode does not check, but wraps the values. (i.e u8 > 255 becomes
        like 256 becomes 0, 257 becomes 1)
        Relying on integer overflow’s wrapping behavior is considered an error.

    - Floating-Point
        Length	Declaration
        32-bit	f32
        64-bit	f64 -> More precision

    Example:
 */


fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

fn numeric_operation() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder or MOD
    let remainder = 43 % 5;
}

//  -Boolean

fn booleans() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

/*
    - Character
        Specified with single quotes '' different than Strings that are double quotes ""
 */

fn character() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

// COMPOUND Types

/*
    - Tuple

        Can group together values of different type
        Have fixed length! Once declared they cannot grow or shrink
        Example:
 */
fn tuple_definition() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup2 = (500, 6.4, 1);

    // Pattern matching can be used to unwrap the tuple.
    let (x, y, z) = tup2;

    println!("The value of y is: {}", y);

    // Tuple values can be accessed by index
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

/*
    - Array

        Holds elements of the same type.
        FIXED LENGTH LIKE TUPLES

        Example:
 */

fn array_definition() {
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // Specifying type and size in declaration

    let a = [3; 5]; // Creates an array of 5 elements -> let a = [3, 3, 3, 3, 3];

    let first = a[0]; // Accessing first element of the array
    let second = a[1]; // Accessing second element of the array

    let five = a[10]; // This will PANIC on runtime as the index provided is out of bounds
}


