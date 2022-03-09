use std::fmt::{Debug, Display};

// A trait tells the Rust compiler about functionality a particular type has
// and can share with other types.
// We can use trait bounds to specify that a generic type can be any type
// that has certain behavior.
pub trait Summary {
    // methods declared on a trait can have a default behavior
    // by implementing the method inside the trait
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// to implement methods for a trait, put the name of the trait after impl
// and the name of your Struct after for
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// item is of some type that implements the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// rewriting the notify function with generics looks like this:
// "impl Trait" though is used more commonly since it's nicer to read
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

// to specify multiple Traits in an function parameter,
// we can combine Traits with a +
pub fn notify3(item: &(impl Summary + Display)) {}

// using a where clause can make function headers easier to read.
// take for example the following function header:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    10
}

// with a where clause this function header looks like this:
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    10
}

// returning implemented traits is also possible,
// but only if the function always returns the SAME type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

// Pair implements for all types the new method
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// but only for types that implement the Display and PartialOrd Trait the cmp_display method
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// implementing another trait based on a trait:
impl<T: Display> ToString for Pair<T> {
    fn to_string(&self) -> String {
        todo!()
    }
}
