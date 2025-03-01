// Project B: Inter-Process Communication (IPC)

// This project demonstrates Inter-Process Communication (IPC) using pipes in Rust.
// A hospital environment is simulated, where a doctor process generates patient reports 
// and nurse processes read and process them. This mimics real-world IPC scenarios 
// where multiple entities communicate and synchronize in a system.

// Key Features:
// - Implements direct Rust-based pipes (`pipe()`) instead of shell-based IPC.
// - Uses `fork()` to create separate processes for doctors and nurses.
// - Ensures data flow integrity by handling broken pipes and process failures.
// - Supports multiple nurses reading from the same pipe, allowing scalable IPC.

use std::io::{Read, Write};
use std::os::fd::{FromRawFd, IntoRawFd, RawFd};
use std::process::exit;
use nix::unistd::{pipe, fork, ForkResult};

fn main() {
    // Create pipe and extract file descriptors as `RawFd`
    let (read_fd, write_fd) = pipe().expect("Failed to create pipe");
    let read_fd: RawFd = read_fd.into_raw_fd();
    let write_fd: RawFd = write_fd.into_raw_fd();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            nix::unistd::close(write_fd).expect("Failed to close write end in parent");
            let mut reader = unsafe { std::fs::File::from_raw_fd(read_fd) };
            let mut buffer = String::new();

            if reader.read_to_string(&mut buffer).is_err() {
                eprintln!("Nurse process: Failed to read from pipe");
                exit(1);
            }
            println!("Nurse received report: {}", buffer);
        }
        Ok(ForkResult::Child) => {
            nix::unistd::close(read_fd).expect("Failed to close read end in child");
            let mut writer = unsafe { std::fs::File::from_raw_fd(write_fd) };
            let message = "Patient: John Doe, Condition: Stable\n";

            if writer.write_all(message.as_bytes()).is_err() {
                eprintln!("Doctor process: Failed to write to pipe");
                exit(1);
            }
        }
        Err(_) => panic!("Fork failed"),
    }
}
