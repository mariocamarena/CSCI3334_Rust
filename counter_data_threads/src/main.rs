use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); 
    let mut threads = vec![]; 

    for i in 0..5 {
        println!("starting thread: {}...", i + 1); 
        let counter_clone = Arc::clone(&counter); 
        threads.push(thread::spawn(move || {
            for i in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        }));
    }

    for t in threads { 
        t.join().unwrap();
    }

    println!("Final value: {}", counter.lock().unwrap()); 
}
