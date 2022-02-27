fn main() {
    // inline_if();
    // breaking_loops();
    // returning_values_from_loops();
    // conditional_loops();
    ranges()
}

fn inline_if() {
    let condition = true;
    // in this case, the value of the whole if expression depends on which block of code executes
    // but: types have to match
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

fn forever_loop() {
    loop {
        println!("hello world")
    }
}

fn breaking_loops() {
    let mut count = 0;
    // 'counting_up is a label, which can be used to break outer loops etc.
    // labels must be prefixed with a '
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn returning_values_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn conditional_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn looping_through_collections() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}

fn ranges() {
    // the rev() function reverses a range
    // the first diget is included inside the range while the later one is excluded
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
