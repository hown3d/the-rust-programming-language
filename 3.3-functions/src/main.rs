fn main() {
    parametered_function(32);
    expression();
    let square = squared(3);
}

fn parametered_function(num: i32) {
    println!("num is {}", num);
}

fn expression() {
    // Calling a function is an expression. Calling a macro is an expression.
    // A new scope block created with curly brackets is an expression.
    let y = {
        let x = 3;
        // the last line of an expression is the return value
        // If you add a semicolon to the end of an expression,
        // you turn it into a statement, and it will then not return a value
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn squared(num: i32) -> i32 {
    // is equal to return num*num;
    num * num
}
