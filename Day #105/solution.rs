// Day 105: Debounce. Each call bumps a generation counter and schedules f after N
// ms; f runs only if its generation is still the latest. O(1) per call.
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn debounce<F: Fn() + Send + Sync + 'static>(f: F, ms: u64) -> impl Fn() {
    let gen = Arc::new(AtomicU64::new(0));
    let f = Arc::new(f);
    move || {
        let my = gen.fetch_add(1, Ordering::SeqCst) + 1;
        let gen = Arc::clone(&gen);
        let f = Arc::clone(&f);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(ms));
            if gen.load(Ordering::SeqCst) == my {
                f(); // only the latest scheduled call fires
            }
        });
    }
}

fn main() {
    let count = Arc::new(AtomicU64::new(0));
    let c = Arc::clone(&count);
    let d = debounce(move || {
        c.fetch_add(1, Ordering::SeqCst);
        println!("f called");
    }, 100);
    d(); d(); d();                          // 3 rapid calls
    thread::sleep(Duration::from_millis(300));
    println!("Total calls to f: {}", count.load(Ordering::SeqCst)); // 1
}
