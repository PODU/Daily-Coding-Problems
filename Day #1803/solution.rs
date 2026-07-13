// Day 1803: Singleton holding two instances; getInstance alternates first (even call) / second (odd call).
// Lazy init guarded by a Mutex; O(1) per call, O(1) space.
use std::sync::{Mutex, OnceLock};

struct Dual {
    id: i32,
}

struct State {
    first: Dual,
    second: Dual,
    counter: u64,
}

fn state() -> &'static Mutex<State> {
    static S: OnceLock<Mutex<State>> = OnceLock::new();
    S.get_or_init(|| {
        Mutex::new(State {
            first: Dual { id: 1 },
            second: Dual { id: 2 },
            counter: 0,
        })
    })
}

// Returns (id, pointer-as-usize for identity check).
fn get_instance() -> (i32, usize) {
    let mut s = state().lock().unwrap();
    let c = s.counter; // call count starts at 0
    s.counter += 1;
    if c % 2 == 0 {
        (s.first.id, (&s.first as *const Dual) as usize)
    } else {
        (s.second.id, (&s.second as *const Dual) as usize)
    }
}

fn main() {
    let mut prev_even: Option<usize> = None;
    for i in 0..4 {
        let (id, ptr) = get_instance();
        println!("call{}->{}", i, id);
        if i % 2 == 0 {
            if let Some(p) = prev_even {
                println!("  even-call identity same: {}", p == ptr);
            }
            prev_even = Some(ptr);
        }
    }
}
