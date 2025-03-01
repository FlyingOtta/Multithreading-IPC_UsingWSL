// Phase 4: Deadlock Resolution

// This phase resolves the deadlock from Phase 3 by implementing Lock Ordering and Try-Lock (`try_lock()`).
// Lock Ordering ensures that all nurses always lock `PatientRecord` first before `MedicineLog`, preventing circular waits.
// `try_lock()` prevents a thread from blocking indefinitely by forcing it to release its first lock and retry if it fails to acquire the second.
// This guarantees forward progress, ensuring that all nurses eventually complete their tasks without deadlock.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct PatientRecord {
    name: String,
    treatment_count: u32,
}

struct MedicineLog {
    medicine_given: u32,
}

fn main() {
    let patient = Arc::new(Mutex::new(PatientRecord {
        name: String::from("Alexander Greatness"),
        treatment_count: 0,
    }));

    let medicine_log = Arc::new(Mutex::new(MedicineLog {
        medicine_given: 0,
    }));

    let mut nurses = vec![];

    for i in 1..=2 {
        let patient_clone = Arc::clone(&patient);
        let medicine_clone = Arc::clone(&medicine_log);

        let nurse = thread::spawn(move || {
            loop {

                let record_lock = patient_clone.lock().unwrap();
                thread::sleep(Duration::from_millis(100));
                

                if let Ok(mut medicine_lock) = medicine_clone.try_lock() {

                    println!("Nurse {} updating records. (Lock Successful)", i);
                    let mut record = record_lock; 
                    record.treatment_count += 1;
                    medicine_lock.medicine_given += 1;
                    break;
                } else {
                  
                    println!("Nurse {} could not acquire MedicineLog, retrying...", i);
                    thread::sleep(Duration::from_millis(50)); 
                }
            }
        });

        nurses.push(nurse);
    }

    for nurse in nurses {
        nurse.join().unwrap();
    }

    let final_record = patient.lock().unwrap();
    let final_medicine = medicine_log.lock().unwrap();
    println!(
        "\nFinal record for {}: {} treatments completed, {} medicines given.",
        final_record.name, final_record.treatment_count, final_medicine.medicine_given
    );
}
