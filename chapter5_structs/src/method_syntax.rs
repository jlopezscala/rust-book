/*
       Methods are like functions, declared with fn and name,
       can have parameters and return a value.

       The first parameter is self, being the instance (like Python)
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /*
        self behaves like any other parameter
        &self is used here since we don't want to take ownership
        using self for taking ownership is rare
        can use &mut self if we need to modify the instance

     */
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}