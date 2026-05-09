// Day 1499: Job scheduler that calls f after n ms via a thread that sleeps.
// Join the thread so main waits for the job. Time O(1), Space O(1).
use std::thread;
use std::time::Duration;

fn schedule<F: FnOnce() + Send + 'static>(f: F, n: u64) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(n));
        f();
    })
}

fn main() {
    let handle = schedule(|| println!("Job executed after delay"), 100);
    handle.join().unwrap(); // wait for the scheduled job before exiting
}
