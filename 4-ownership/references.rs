fn main() {
    let s1 = String::from("hello");

    // ownership of the variable wont be passed into the function
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// Instead, we can provide a reference to the String value.
// A reference is like a pointer in that it’s an address we can follow to access
// data stored at that address that is owned by some other variable.
// Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type.
//
// references are immutable by default!
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// to declare references to be mutable, we need to explicitly say that by prefixing "&mut".
// Mutable references have one big restriction:
// you can have only one mutable reference to a particular piece of data at a time.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
