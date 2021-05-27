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
    // Usually declare all methods in one impl block to not make a mess..

    fn can_hold(&self, other: &Rectangle) -> bool {
        // Takes an immutable borrow of a Rectangle as a parameter
        self.width > other.width && self.height > other.height
    }
    // Associated functions are like class methods.
    fn square(size: u32) -> Rectangle {
        /*
            Dont take self as parameter
            It can be called like Rectangle::square(5);
            Already used it with String::from("hola");
         */
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // calling a method
    );
}

fn main2_can_hold() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

