// Job scheduler: run f after n ms using a spawned thread that sleeps then calls f.
// std::thread::spawn + sleep + join. Schedule is O(1); main joins to wait.
use std::thread;
use std::time::Duration;

fn schedule<F>(f: F, n: u64) -> thread::JoinHandle<()>
where
    F: FnOnce() + Send + 'static,
{
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(n));
        f();
    })
}

fn main() {
    let handle = schedule(|| println!("Job executed after 100 ms"), 100);
    handle.join().unwrap(); // wait for the job to run before exiting
    println!("Main: job completed, exiting");
}
