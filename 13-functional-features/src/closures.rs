use std::{collections::HashMap, hash::Hash, thread, time::Duration};
pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensitiy: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensitiy < 25 {
        println!("Today, do {} pushups", expensive_result.value(intensitiy));
        println!("Next, do {} situps", expensive_result.value(intensitiy));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensitiy)
            );
        }
    }
}

struct Cacher<T, K, V> {
    calculation: T,
    cache: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Hash + Eq + Clone,
    V: Clone
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }
    fn value(&mut self, arg: K) -> V {
        if self.cache.contains_key(&arg) {
            return self.cache.get(&arg).unwrap().clone();
        }
        let v = (self.calculation)(arg.clone());
        self.cache.insert(arg.clone(), v);
        self.cache.get(&arg).unwrap().clone()
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a: i32| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}
