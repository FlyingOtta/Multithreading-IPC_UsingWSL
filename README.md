# Multithreaded Programming and Inter-Process Communication (IPC)

## Overview
This project demonstrates fundamental operating system concepts, including **multithreading** and **inter-process communication (IPC)**, using **Rust** within an **Ubuntu WSL** environment inside **VS Code**. The project consists of two major parts:

1. **Project A: Multithreading**
   - Implements concurrent execution using multiple threads.
   - Demonstrates synchronization mechanisms (e.g., mutexes) for shared resource protection.
   - Simulates deadlock conditions and implements deadlock resolution strategies.

2. **Project B: Inter-Process Communication (IPC)**
   - Uses **pipes** to establish communication between processes.
   - Implements a **producer-consumer** model where a doctor (child process) generates reports, and a nurse (parent process) reads them.
   - Ensures error handling for broken pipes and resource management.

---

## System Requirements
- **Operating System**: Ubuntu (via Windows Subsystem for Linux - WSL)
- **Development Environment**: Visual Studio Code
- **Programming Language**: Rust
- **Dependencies**: `nix` crate (for IPC and system calls)

---

## Installation and Setup

### 1. Install WSL and Ubuntu
Ensure that Windows Subsystem for Linux (WSL) is installed and set up with Ubuntu:

```sh
wsl --install
```

### 2. Install Rust
Run the following command in Ubuntu (WSL) to install Rust:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3. Install Nix (Rust library for system calls & IPC)
Run the following command in Ubuntu (WSL) to install Nix:

```sh
cargo install nix
```

### 4. Clone this repository within your environment
Run the following command in Ubuntu (WSL) to clone this repository:

```sh
git clone https://github.com/FlyingOtta/Multithreading-IPC_UsingWSL.git
cd multithreading-ipc
```

## Installation and Setup

### Each phase of the project is implemented in its own Rust file
Project files can be run using a command such as: 

```sh
cargo run --bin phase1
cargo run --bin phase2
```

Etc.

## Testing Guidelines

### Multithreading Tests
- **Concurrency Test:** Ensure multiple threads execute independently.
- **Synchronization Test:** Verify that mutexes prevent race conditions.
- **Deadlock Test:** Observe blocked threads indefinitely in the deadlock scenario.
- **Deadlock Resolution Test:** Confirm that using `try_lock()` prevents indefinite blocking.

### IPC Tests
- **Data Integrity Test:** Check if data passed through pipes is intact.
- **Error Handling Test:** Simulate a broken pipe and confirm graceful handling.
- **Performance Benchmarking:** Measure IPC latency for large messages.

---

## Author
**Gavin Doby**  
CS 3502 - Operating Systems  
Kennesaw State University  

For questions or issues, please submit a GitHub issue.

---

## References
- [Rust Documentation](https://doc.rust-lang.org/)
- [Mutex in Rust](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [Nix Crate (System Calls)](https://docs.rs/nix)
- **[CS 3502 Project 1 Document](CS_3502_Project_1_Multi_Threaded_Programming.pdf)** (Provided by Instructor)


### Project Files:
- [APhase1BasicThreadOps.rs](APhase1BasicThreadOps.rs) - Basic Threading
- [APhase2ResourceProtection.rs](APhase2ResourceProtection.rs) - Mutex Synchronization
- [APhase3DeadlockCreation.rs](APhase3DeadlockCreation.rs) - Deadlock Scenario
- [APhase4DeadlockResolution.rs](APhase4DeadlockResolution.rs) - Deadlock Resolution
- [BPhase1IPC.rs](BPhase1IPC.rs) - Inter-Process Communication
