pub fn vectors() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    v.push(3);
    // two ways to access values at an index
    let third = &v[2];
    let third = match v.get(2) {
        // without dereferencing, this would throw an error.
        // This is because of the push() on line 29.
        // why should a reference to the first element care
        // about what changes at the end of the vector?
        // This error is due to the way vectors work:
        /*
        adding a new element onto the end of the vector might require allocating new memory
        and copying the old elements to the new space,
        if there isn’t enough room to put all the elements next to each other
        where the vector currently is.
        In that case, the reference to the first element would be pointing to deallocated memory.
        */
        Some(&third) => third,
        None => panic!("no value"),
    };

    v.push(9);
    println!("The third element is {}", third);

    // Iterating over vectors
    for i in &v {
        println!("{}", i);
    }

    // changing the references in the vector
    for i in &mut v {
        *i += 50;
    }

    // using enums in a vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
