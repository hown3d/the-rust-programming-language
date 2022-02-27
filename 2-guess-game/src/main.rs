// use is the import statement in rust
// rust automaticly imports a few resources into every program.
// It's called the prelude and more information can be found here:
// https://doc.rust-lang.org/std/prelude/index.html
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // :: is used when a function is associated with the type.
        let mut guess: String = String::new();

        io::stdin()
            // The & indicates that this argument is a reference,
            // which gives you a way to let multiple parts of your code access one piece of data
            // without needing to copy that data into memory multiple times.
            // references are immutable by default.
            // To enable mutation, you need to specify &mut to make them mutable
            //
            // the return value of the read_line fn is a Result.
            // The Result types are enumerations, often referred to as enums,
            // which can have a fixed set of possibilities known as variants.
            //
            // Result’s variants are Ok or Err. The Ok variant indicates the operation was successful,
            // and inside Ok is the successfully generated value.
            // The Err variant means the operation failed,
            // and Err contains information about how or why the operation failed.
            .read_line(&mut guess)
            // if Result is of type Error, expect will catch it and panic
            .expect("Failed to read line");

        //if guess.cmp("quit") {}

        // shadow guess var by overriding it as i32 type
        //
        // The parse method on strings parses a string into some kind of number.
        // Because this method can parse a variety of number types,
        // we need to tell Rust the exact number type we want by using let guess: u32
        let guess: u32 = match guess.trim().parse() {
            // if Result matches ok, return the number
            Ok(num) => num,
            // The underscore, _, is a catchall value;
            // in this example, we’re saying we want to match all Err values,
            // no matter what information they have inside them.
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Equal => {
                println!("You won!");
                break;
            }
            std::cmp::Ordering::Greater => {
                println!("Too big!")
            }
            std::cmp::Ordering::Less => {
                println!("Too small!")
            }
        }
    }
}
