
fn main() {
    arrays()
}

fn arrays() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // array with type annotation and size
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // 5 3's inside the array
    let all_equal_items = [3; 5];
    println!("{:?}", all_equal_items)
}

fn tuples() {
    let tup = (500, 6.4, 1);

    // destructuring
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // accessing by index
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // empty tuples are called unit types and their single value "()" is called unit value
    // used when expressions return nothing
    let unit = ();
}
