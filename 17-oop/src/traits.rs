// traits can be used to define common behavior.
// We can use trait objects in place of a generic or concrete type.
// Wherever we use a trait object, Rust’s type system will ensure at compile time
// that any value used in that context will implement the trait object’s trait.
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw> is a stand-in for any type inside a Box that implements the Draw trait.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// this implementation would restrict components to be all of the same type T.
// components can't hold mixed implementations of Draw trait.
//
// If you’ll only ever have homogeneous collections, using generics and trait bounds is preferable
// because the definitions will be monomorphized at compile time to use the concrete types.
pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen2<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}