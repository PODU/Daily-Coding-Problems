// Job scheduler: call f after n ms using a worker thread that sleeps, joined before exit.
// Time: O(1) to schedule; Space: O(1).
use std::thread;
use std::time::Duration;

fn schedule<F: FnOnce() + Send + 'static>(f: F, n: u64) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(n));
        f();
    })
}

fn main() {
    println!("Scheduling job...");
    let handle = schedule(|| println!("Job executed after 100 ms"), 100);
    // Join the worker so the scheduled job runs before the program exits.
    handle.join().unwrap();
}
