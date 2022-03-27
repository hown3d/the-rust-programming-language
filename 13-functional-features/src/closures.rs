use std::{any::type_name, collections::HashMap, hash::Hash, thread, time::Duration};
pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensitiy: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        &num
    });
    if intensitiy < 25 {
        println!("Today, do {} pushups", expensive_result.value(&intensitiy));
        println!("Next, do {} situps", expensive_result.value(&intensitiy));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(&intensitiy)
            );
        }
    }
}

struct Cacher<'a, T, K: 'a>
where
    T: Fn(&'a K) -> &'a K,
{
    calculation: T,
    cache: HashMap<&'a K, &'a K>,
}

impl<'a, T, K> Cacher<'a, T, K>
where
    T: Fn(&'a K) -> &'a K,
    K: Hash + Eq,
{
    fn new(calculation: T) -> Cacher<'a, T, K> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }
    fn value(&mut self, arg: &'a K) -> &'a K {
        match self.cache.get(arg) {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.cache.insert(arg, v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(&1);
    let v2 = c.value(&2);

    assert_eq!(*v1, 1);
    assert_eq!(*v2, 2);
}
