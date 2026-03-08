// Day 1179: Debounce - call f only after N ms elapse with no further invocations.
// Each call bumps a generation counter; a spawned worker fires only if still latest.
// Time O(1) per call.
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Debouncer {
    gen: Arc<AtomicU64>,
    delay: Duration,
    f: Arc<dyn Fn() + Send + Sync>,
}

impl Debouncer {
    fn new(delay: Duration, f: Arc<dyn Fn() + Send + Sync>) -> Self {
        Debouncer { gen: Arc::new(AtomicU64::new(0)), delay, f }
    }
    fn call(&self) {
        let g = self.gen.fetch_add(1, Ordering::SeqCst) + 1;
        let gen = self.gen.clone();
        let delay = self.delay;
        let f = self.f.clone();
        thread::spawn(move || {
            thread::sleep(delay);
            if gen.load(Ordering::SeqCst) == g {
                f();
            }
        });
    }
}

fn main() {
    let count = Arc::new(AtomicU64::new(0));
    let c = count.clone();
    let d = Debouncer::new(Duration::from_millis(100), Arc::new(move || {
        c.fetch_add(1, Ordering::SeqCst);
    }));
    for _ in 0..5 {                 // rapid burst, every 20ms
        d.call();
        thread::sleep(Duration::from_millis(20));
    }
    thread::sleep(Duration::from_millis(300));
    println!("f executed {} time(s)", count.load(Ordering::SeqCst)); // 1
}
