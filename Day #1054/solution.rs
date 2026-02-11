// Day 1054: Dual-instance singleton. Holds two lazily-created instances and an
// alternating counter; get_instance() returns instance[count % 2], then bumps the
// counter. Time O(1) per call, Space O(1).

use std::sync::Mutex;

#[derive(Clone, Copy)]
struct DualSingleton {
    id: u32,
}

static INSTANCES: Mutex<Option<([DualSingleton; 2], usize)>> = Mutex::new(None);

fn get_instance() -> DualSingleton {
    let mut guard = INSTANCES.lock().unwrap();
    if guard.is_none() {
        *guard = Some(([DualSingleton { id: 1 }, DualSingleton { id: 2 }], 0));
    }
    let state = guard.as_mut().unwrap();
    let inst = state.0[state.1 % 2];
    state.1 += 1;
    inst
}

fn main() {
    for call in 0..6 {
        let (kind, parity) = if call % 2 == 0 {
            ("first", "even")
        } else {
            ("second", "odd")
        };
        println!(
            "call {} ({}) -> {} instance (id={})",
            call, parity, kind, get_instance().id
        );
    }
}
