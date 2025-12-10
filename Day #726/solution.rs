// Day 726: Singleton holding TWO instances; alternate on even/odd getInstance() calls.
// Approach: Static counter; odd call -> instance #2, even call -> instance #1.
// Time: O(1) per call, Space: O(1).
use std::sync::Mutex;

struct DualSingleton {
    first: i32,
    second: i32,
    count: u64,
}

static SINGLETON: Mutex<DualSingleton> = Mutex::new(DualSingleton { first: 1, second: 2, count: 0 });

fn get_instance() -> i32 {
    let mut s = SINGLETON.lock().unwrap();
    s.count += 1;
    if s.count % 2 == 0 { s.first } else { s.second } // even->first, odd->second
}

fn main() {
    for i in 1..=4 {
        println!("Call {}: instance {}", i, get_instance());
    }
}
