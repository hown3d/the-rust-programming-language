use std::time::Duration;
use std::{thread, vec};
pub fn wait_for_thread_to_finish() {
    // Join Handle holds the return value of the thread. If
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        panic!("error")
    });
    // waits for the thread to finish
    let value = handler.join().unwrap_or_else(|err| {
        println!("{:?}", err);
        "ERROR"
    });

    println!("{}", value);
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn moving_data_into_thread_closure() {
    let my_vec = vec![1, 2, 3, 4, 5];
    // move is neccessecary to access myVec inside the closure
    let handler = thread::spawn(move || {
        println!("{:?}", my_vec);
    });
    handler.join().unwrap();
    // access vector after thread
    // code wont compile
    //println!("{:?}", myVec)
}
