fn main() {
    let s = String::from("hello world");

    // We create slices using a range within brackets by specifying [starting_index..ending_index]
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}", first_word(&s));
    other_slices()
}

// returns a string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3])
}
