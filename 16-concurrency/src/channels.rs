use std::sync::mpsc;
use std::time::Duration;
use std::{thread, vec};
pub fn channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        thread::sleep(Duration::new(5, 0));
        tx.send(val).unwrap()
        // this wont compile, because val is moved out of this scope
        // println!("val is {}", val);
    });

    // rx.recv is blocking
    if let Ok(recieved) = rx.recv() {
        println!("Got: {}", recieved);
    };
}

pub fn channel_as_iterator() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        println!("Got: {}", recieved)
    }
}

pub fn channel_multiple_senders() {
    let (tx, rx) = mpsc::channel();

    // cloning the sender will allow us to pass the new cloned sender into another thread
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
