#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Everything within this impl block will be associated with the Rectangle type
impl Rectangle {
    // The &self is actually short for self: &Self.
    // Within an impl block, the type Self is an alias for the type that the impl block is for.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // We can define associated functions that don’t have self as their first parameter
    // (and thus are not methods) because they don’t need an instance of the type to work with
    fn square(size: u32) -> Rectangle {
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
        rect1.area()
    );

    let squared = Rectangle::square(10);
    println!("The squared rectangle is {:?}", squared);
}
