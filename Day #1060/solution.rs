// Debounce: wrap f so it runs only after N ms of silence; each call resets the timer.
// A worker thread sleeps N ms then fires only if no newer call bumped a generation
// counter. Time: O(1) per call, Space: O(1).
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Debouncer {
    gen: Arc<AtomicU64>,
    delay: Duration,
}

impl Debouncer {
    fn new(delay_ms: u64) -> Self {
        Debouncer {
            gen: Arc::new(AtomicU64::new(0)),
            delay: Duration::from_millis(delay_ms),
        }
    }

    fn call<F>(&self, f: F, arg: i32)
    where
        F: Fn(i32) + Send + 'static,
    {
        let my_gen = self.gen.fetch_add(1, Ordering::SeqCst) + 1;
        let gen = Arc::clone(&self.gen);
        let delay = self.delay;
        thread::spawn(move || {
            thread::sleep(delay);
            // Fire only if no newer call happened.
            if gen.load(Ordering::SeqCst) == my_gen {
                f(arg);
            }
        });
    }
}

fn main() {
    use std::sync::Mutex;
    let calls = Arc::new(Mutex::new(0));
    let d = Debouncer::new(100);

    for i in 1..=5 {
        // burst of 5 calls
        let calls = Arc::clone(&calls);
        d.call(
            move |x| {
                let mut c = calls.lock().unwrap();
                *c += 1;
                println!("f called with {}; total calls = {}", x, *c);
            },
            i,
        );
    }

    thread::sleep(Duration::from_millis(300)); // wait > N ms
    println!("done; f ran {} time(s)", *calls.lock().unwrap());
}
