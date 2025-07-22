// Job scheduler: spawn a thread that sleeps n ms then invokes f.
// schedule: O(1); each job runs on its own timer thread.
use std::thread;
use std::time::Duration;

fn schedule<F: FnOnce() + Send + 'static>(f: F, n: u64) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(n));
        f();
    })
}

fn main() {
    let handle = schedule(|| println!("Executed after delay!"), 100);
    handle.join().unwrap(); // let the job fire before exit
}
