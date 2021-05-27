fn main2() {
    let width1 = 30;
    let height1 = 5;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}


fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Refactoring above with Tuples
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_tuple(rect1)
    );
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// We can do better with a Struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn main3() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_struct(&rect1)
    );

    // println!("rect1 is {}", rect1); this is a compile error
    // rect1 does not implement Display, (like str or repr in python?)
    // Primitive types already implement their own Display

}

fn area_with_struct(rectangle: &Rectangle) -> u32 { // rectangle is an immutable borrow of the instance
    rectangle.width * rectangle.height
}

#[derive(Debug)] // Adding this annotation to use :? or Debug trait. More in chapter 10
struct DebuggableRectangle {
    width: u32,
    height: u32,
}

fn debugging() {
    let rect1 = DebuggableRectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    // to show something like rect1 is Rectangle { width: 30, height: 50 }
    // or
    println!("rect1 is {:#?}", rect1);
    /*
        :?# to something like this
        rect1 is Rectangle {
            width: 30,
            height: 50,
        }
     */

}



// We can do even better if we add the area function as part of the rectangle Struct

