// Day 1627: Debounce f by N ms. Each call resets an N-ms timer; f fires
// only after a quiet gap of N ms. Time O(1) per call.
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn debounce<F>(f: F, n: Duration) -> impl Fn()
where
    F: Fn() + Send + Sync + 'static,
{
    let f = Arc::new(f);
    let generation = Arc::new(AtomicU64::new(0));
    move || {
        let my = generation.fetch_add(1, Ordering::SeqCst) + 1;
        let gen2 = Arc::clone(&generation);
        let f2 = Arc::clone(&f);
        thread::spawn(move || {
            thread::sleep(n);
            if gen2.load(Ordering::SeqCst) == my {
                f2();
            }
        });
    }
}

fn main() {
    let calls = Arc::new(AtomicU64::new(0));
    let c2 = Arc::clone(&calls);
    let g = debounce(move || { c2.fetch_add(1, Ordering::SeqCst); }, Duration::from_millis(100));
    g(); g(); g();
    thread::sleep(Duration::from_millis(200));
    println!("f was called {} time(s)", calls.load(Ordering::SeqCst));
}
