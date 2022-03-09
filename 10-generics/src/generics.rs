fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_with_traits<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

struct GenericStruct<T> {
    x: T,
    y: T,
}

// implement on all types of GenericStruct
impl<T> GenericStruct<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// implement only on f64 of GenericStruct
impl GenericStruct<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// to parse different generic types, we must specify 2 different placeholders
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T, U2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main2() {
    let integer = GenericStruct { x: 10, y: 20 };

    println!("integer.x = {}", integer.x());

    let float = GenericStruct { x: 10.0, y: 20.0 };
    println!("distance_from_origin = {}", float.distance_from_origin());
    let integer_and_float = Point { x: 10, y: 20.0 };

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
