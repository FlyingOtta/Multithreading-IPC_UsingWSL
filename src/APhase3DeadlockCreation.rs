// Phase 3: Deadlock Creation

// This phase intentionally creates a deadlock scenario to demonstrate multi-threading pitfalls.
// Nurses try to lock resources in different orders, leading to a circular wait condition where threads become stuck indefinitely.

use std::sync::{Arc, Mutex}; 
use std::thread;
use std::time::Duration;

struct PatientRecord {
    name: String,          
    treatment_count: u32,  
}

struct Surgeries {          
    surgery_count: u32,
}

fn main() {
    let patient = Arc::new(Mutex::new(PatientRecord {
        name: String::from("Alexander Greatness"), 
        treatment_count: 0,             
    }));

    let surgeries = Arc::new(Mutex::new(Surgeries {
        surgery_count: 0,
    }));

    let mut nurses = vec![];

    for i in 1..=2 {
        let patient_clone = Arc::clone(&patient);
        let surgeries_clone = Arc::clone(&surgeries);

        let nurse = thread::spawn(move || {
            if i % 2 == 0 {
                let mut patient_lock = patient_clone.lock().unwrap();
                thread::sleep(Duration::from_millis(100));
                let mut surgery_lock = surgeries_clone.lock().unwrap();
                patient_lock.treatment_count += 1;
                surgery_lock.surgery_count += 1;
                println!("Nurse {} updated {}'s record. Treatments: {}, Surgeries: {}", 
                    i, patient_lock.name, patient_lock.treatment_count, surgery_lock.surgery_count);
            } else {
                let mut surgery_lock = surgeries_clone.lock().unwrap();
                thread::sleep(Duration::from_millis(100));
                let mut patient_lock = patient_clone.lock().unwrap();
                patient_lock.treatment_count += 1;
                surgery_lock.surgery_count += 1;
                println!("Nurse {} updated {}'s record. Treatments: {}, Surgeries: {}", 
                    i, patient_lock.name, patient_lock.treatment_count, surgery_lock.surgery_count);
            }
        });

        nurses.push(nurse);
    }

    for nurse in nurses {
        nurse.join().unwrap();
    }
}
