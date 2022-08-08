use std::{iter::Sum, ops::Add, vec};

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

pub fn sum(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

pub fn sum_generic<T: Sum + Copy>(v: &Vec<T>) -> T {
    v.iter().copied().sum()
}

pub fn create_vec(n: i32) -> Vec<i32> {
    let mut vec = vec![];
    for index in 0..n {
        vec.push(index);
    }
    vec
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;

//     #[bench]
//     fn bench_sum(b: &mut Bencher) {
//         let vec = create_vec(1000000);
//         b.iter(|| sum(&vec));
//     }

//     #[bench]
//     fn bench_sum_generic(b: &mut Bencher) {
//         let vec = create_vec(1000000);
//         b.iter(|| sum_generic(&vec));
//     }
// }
