// Day 1435: Singleton holding two instances; even call -> first, odd call -> second.
// Approach: Two static instances + atomic call counter, return by parity of count.
// Time: O(1) per call, Space: O(1).
use std::sync::atomic::{AtomicUsize, Ordering};

struct DualSingleton {
    id: u32,
}

static FIRST: DualSingleton = DualSingleton { id: 1 };
static SECOND: DualSingleton = DualSingleton { id: 2 };
static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn get_instance() -> &'static DualSingleton {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst); // 0-indexed call number
    if n % 2 == 0 {
        &FIRST
    } else {
        &SECOND
    }
}

fn main() {
    for i in 0..4 {
        let inst = get_instance();
        println!("Call {} -> instance {}", i, inst.id);
    }
    // Even calls (0,2) -> instance 1, odd calls (1,3) -> instance 2
}
