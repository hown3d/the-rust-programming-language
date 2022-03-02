#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
/*
We define a module by starting with the mod keyword
and then specify the name of the module
and place curly brackets around the body of the module.
Inside modules, we can have other modules,
as in this case with the modules hosting and serving.

Modules can also hold definitions for other items,
such as structs, enums, constants, traits,

Modules aren’t useful only for organizing your code.
They also define Rust’s privacy boundary.

Using a semicolon after mod front_of_house
rather than using a block tells Rust to load the contents of the module
from another file with the same name as the module.
*/
mod front_of_house;

pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // with use
    hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        // The fix_incorrect_order function is in the back_of_house module,
        // so we can use super to go to the parent module of back_of_house,
        // which in this case is crate, the root.
        super::serve_order();
    }

    fn cook_order() {}
}
