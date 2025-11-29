// Day 671: Debounce. Each call bumps a generation id and schedules f after N ms;
// f fires only if no newer call arrived. Per-call O(1).
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Debouncer {
    gen: Arc<AtomicU64>,
    n: u64,
    f: Arc<dyn Fn(String) + Send + Sync>,
}

impl Debouncer {
    fn new(n: u64, f: Arc<dyn Fn(String) + Send + Sync>) -> Self {
        Debouncer { gen: Arc::new(AtomicU64::new(0)), n, f }
    }
    fn call(&self, arg: String) {
        let id = self.gen.fetch_add(1, Ordering::SeqCst) + 1;
        let gen = self.gen.clone();
        let f = self.f.clone();
        let n = self.n;
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(n));
            if gen.load(Ordering::SeqCst) == id {
                f(arg);
            }
        });
    }
}

fn main() {
    let d = Debouncer::new(50, Arc::new(|s: String| println!("f called with: {}", s)));
    for s in ["a", "b", "c", "d", "e"] {
        d.call(s.to_string()); // rapid burst
    }
    thread::sleep(Duration::from_millis(200)); // prints once: f called with: e
}
