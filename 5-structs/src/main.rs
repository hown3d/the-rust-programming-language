fn main() {
    let user1 = User {
        active: true,
        username: String::from("testuser"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        active: true,
        username: String::from("testuser2"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("some2@example.com");
    println!("{:?},{:?}", user1, user2);
    let user3 = build_user(String::from("some2@example.com"), String::from("test"));

    // struct update syntax
    // after updating the data, the old variable becomes inaccesible

    let user3 = User {
        email: String::from("another@example.com"),
        ..user3
    };

    println!("{:?}", user3);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    // field init shorthand because the email and username parameters have the same name as struct fields
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Tuple structs are useful when you want to give the whole tuple a name
// and make the tuple a different type from other tuples,
// and when naming each field as in a regular struct would be verbose or redundant
struct Point(i32, i32, i32);
