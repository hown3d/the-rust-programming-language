pub fn strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3: String = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);

    println!("Hello world in Chars:");
    // accessing the characters of String can be done like this
    for c in s3.chars() {
        println!("{}", c)
    }

    // the underlying byte values can be accessed with the bytes method

    println!("Hello world in Bytes:");
    for b in s3.bytes() {
        println!("{}", b)
    }
}
