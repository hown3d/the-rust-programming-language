enum List {
    Cons(i32, Box<List>),
    Nil,
}
use self::List::{Cons, Nil};

fn main() {
    let list = 1;
}
