// Job scheduler: schedule f after n ms using thread::spawn + sleep. O(1) schedule, join handle waits for completion.
use std::thread;
use std::time::Duration;

fn schedule<F>(f: F, delay_ms: u64) -> thread::JoinHandle<()>
where
    F: FnOnce() + Send + 'static,
{
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(delay_ms));
        f();
    })
}

fn main() {
    println!("Scheduling job...");
    let handle = schedule(|| {
        println!("Job executed!");
    }, 100);
    handle.join().unwrap();
}
