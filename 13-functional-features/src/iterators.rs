use std::{iter::Sum, vec};

pub fn run() {
    let mut v1 = vec![1, 2, 3];

    //let mut v1_mut_iter = v1.iter_mut();
    // for val in v1_iter {
    //     println!("{}", val)
    // }
    // for val in v1_mut_iter {
    //     *val = *val + 1;
    // }

    // for val in v1 {
    //     println!("{}", val)
    // }

    let sum = sum(&v1);
    println!("{}", sum);
}

fn sum(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}
