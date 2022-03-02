fn main() {
    let v6 = IpAddrKind::V6;
    let v4 = IpAddrKind::V4;
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let home = IpAddrKindV2::V4(127, 0, 0, 1);

    let m = Message::Write(String::from("hello"));
    m.call();

    // using the option enum (like optional in Java)
    let optional_number = Some(123);

    let number = match optional_number {
        Some(n) => n,
        None => panic!("number is empty"),
    };

    if let Some(number) = optional_number {
        println!("optional number has indeed a number: {}", number)
    }

    // catch all patterns
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // this is a variable and can be of any name
        // like default in go switch case
        other => move_player(other),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // throw away all other values
        _ => reroll(),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
fn move_player(num_spaces: u8) {}

enum IpAddrKind {
    // Data inside enum
    // this can be anything (struct, array, common data type etc.)
    V4(String),
    V6(String),
}

enum IpAddrKindV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enums can hold methods like structs
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
