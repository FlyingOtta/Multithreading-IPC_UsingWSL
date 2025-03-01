// Project A Phase 1: Basic Thread Operations

// This phase demonstrates the fundamentals of multi-threading in Rust by spawning multiple threads.
// Each nurse runs independently, checking patient vitals without shared data or synchronization mechanisms.

use std::thread;
use std::time::{Duration, Instant};

fn main() 
{
    let start_time = Instant::now();
    let mut nurses = vec![];

    for i in 1..=3 {
        let nurse = thread::spawn(move || 
            {
                println!("Nurse {}: Checking patient’s vital signs...", i);
                thread::sleep(Duration::from_secs(2));
                println!("Nurse {}: Finished recording vitals.", i);
            });

        nurses.push(nurse);
    }

    for nurse in nurses {
        nurse.join().unwrap();
    }

    let elapsed = start_time.elapsed().as_secs();
    println!("[{}s] All nurses have completed their tasks.", elapsed);
}
