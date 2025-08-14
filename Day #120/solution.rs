// Day 120: Dual singleton. Two fixed instances; odd call -> 2nd, even call -> 1st.
// getInstance() is O(1); instances are process-global statics.
use std::sync::atomic::{AtomicU64, Ordering};

struct Instance {
    name: &'static str,
}

static FIRST: Instance = Instance { name: "first" };
static SECOND: Instance = Instance { name: "second" };
static COUNT: AtomicU64 = AtomicU64::new(0);

fn get_instance() -> &'static Instance {
    let c = COUNT.fetch_add(1, Ordering::SeqCst) + 1; // 1-based call number
    if c % 2 == 0 {
        &FIRST // even -> first
    } else {
        &SECOND // odd -> second
    }
}

fn main() {
    for _ in 0..4 {
        println!("{}", get_instance().name); // second, first, second, first
    }
}
