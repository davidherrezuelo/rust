use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create an Arc and Mutex wrapping an integer
    let counter = Arc::new(Mutex::new(0));

    // Vector to hold the handles of the spawned threads
    let mut handles = vec![];

    for _ in 0..4000 {
        // Clone the Arc to share ownership with the new thread
        let counter = Arc::clone(&counter);

        // Spawn a new thread
        let handle = thread::spawn(move || {
            loop {
                // Lock the Mutex to get access to the inner value
                let mut num = counter.lock().unwrap();
                // Increment the value
                *num += 1;
                // Print the incremented value
                println!("Thread incremented value to: {}", *num);
                // Sleep for a short duration to prevent excessive CPU usage
            }
        });

        // Push the handle to the vector to keep it from being dropped
        handles.push(handle);
    }

    // Join all the threads to ensure they continue running indefinitely
    for handle in handles {
        handle.join().unwrap();
    }
}