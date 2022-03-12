// fn main2() {
//     {
//         let r;

//         {
//             let x = 5;
//             // compiler complains, because x would be dropped
//             // when exiting the scope
//             r = &x;
//         }

//         println!("r: {}", r);
//     }
// }

fn main() {
    let string1 = String::from("abcd");
    // {
    //     let string2 = String::from("xyz");

    //     // this code doesn't compile either, because the lifetime of the parameters needs
    //     // to be the same. as string2 has the smaller lifetime, it will be used as 'a inside the function.
    //     // String2 will be dropped, when exiting this scope, so the lifetime of result is not valid outside
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    create_important_excerpt();
}

// this function doesn't compile, because on compile time it's not sure
// wether x or y will be returned and need to be keep alive

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// adding a lifetime annotation to the parameters in the function solves the problem
// syntax for lifetime annotation:
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
//
// like generics, lifetimes are annotated before the parameters inside <> bracets
// doing so, will tell rust, that the lifetime of x, y and the returned value will all
// be of the same lifetime = 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// structs can hold references as well!
// doing so, we need to specify the lifetime of a reference inside the struct.
// every instance of ImportExcerpt can't outlive it's reference it holds in "part" field
struct ImportantExcerpt<'a> {
    part: &'a str,
}

use std::fmt::Display;

// creating methods for structs with a lifetime look like the following
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> Display for ImportantExcerpt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "part = {}", self.part)
    }
}

fn create_important_excerpt() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let level = i.level();
    println!("{}, level = {}", i, level);

    //static lifetimes mean, that the reference can live the entire duration of the program
    let s: &'static str = "I have a static lifetime";
}
