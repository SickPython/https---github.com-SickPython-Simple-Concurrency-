use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let mut handles = vec![];

    // Spawn 3 threads
    for i in 1..=3 {
        let handle = thread::spawn(move || {
            println!("Thread {} is running", i);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread failed to join");
    }

    println!("All threads completed.");

//Assignment 2

    // Shared counter wrapped in Arc and Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread failed to join");
    }

    // Print final counter value
    println!("Final counter value: {}", *counter.lock().unwrap());
}