use collection::new_averaged_collection;

mod collection;
mod traits;

fn main() {
    collection();
    traits_main();
}

fn collection() {
    let mut average_collection = new_averaged_collection();
    average_collection.add(3);
    match average_collection.remove() {
        Some(val) => println!("latest: {}", val),
        _ => {}
    }
    println!("average: {}", average_collection.average())
}

use traits::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
fn traits_main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
