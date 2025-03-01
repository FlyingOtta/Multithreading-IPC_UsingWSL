// Phase 2: Resource Protection

// This phase introduces mutexes (`Mutex<T>`) to ensure safe access to shared data.
// Nurses now update a shared patient record, and `Arc<Mutex<T>>` is used to allow multiple threads to access the resource safely.

use std::sync::{Arc, Mutex}; 
use std::thread;
use std::time::Duration;

struct PatientRecord {
    name: String,          
    treatment_count: u32,  
}

fn main() {
    let patient = Arc::new(Mutex::new(PatientRecord {
        name: String::from("Alexander Greatness"), 
        treatment_count: 0,             
    }));

    let mut nurses = vec![];

    for i in 1..=5 {
        let patient_clone = Arc::clone(&patient); 

        let nurse = thread::spawn(move || { 
            for _ in 0..3 { 
                let mut patient_record = patient_clone.lock().unwrap(); 
                patient_record.treatment_count += 1; 
                println!("Nurse {} updated {}'s record. Treatments: {}", i, patient_record.name, patient_record.treatment_count);
                thread::sleep(Duration::from_millis(700)); 
            }
        });

        nurses.push(nurse); 
    }

    for nurse in nurses {
        nurse.join().unwrap();
    }

    let final_record = patient.lock().unwrap(); 
    println!(
        "\nFinal record for {}: {} treatments completed.",
        final_record.name, final_record.treatment_count
    );
}
