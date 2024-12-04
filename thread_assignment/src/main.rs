use std::thread;

fn main() {
    let mut handles = Vec::new();
    for i in 1..=3 {
        handles.push(thread::spawn(move || {
            println!("Thread {}", i);
        }));
    }

    for i in handles {
        i.join().unwrap();
    }
    
    println!("All threads completed.");
}
