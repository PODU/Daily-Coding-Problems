// Busiest period: sort events by timestamp, sweep occupancy, track max-occupancy interval [t_i, t_{i+1}). O(n log n) time, O(n) space.

struct Event {
    timestamp: i64,
    count: i64,
    typ: &'static str,
}

fn main() {
    let mut events = vec![
        Event { timestamp: 1526579928, count: 3, typ: "enter" },
        Event { timestamp: 1526580000, count: 2, typ: "enter" },
        Event { timestamp: 1526580382, count: 2, typ: "exit" },
        Event { timestamp: 1526580500, count: 1, typ: "enter" },
        Event { timestamp: 1526580700, count: 4, typ: "exit" },
    ];
    events.sort_by_key(|e| e.timestamp);

    let mut current: i64 = 0;
    let mut max_occ: i64 = -1;
    let mut best_start: i64 = 0;
    let mut best_end: i64 = 0;
    for i in 0..events.len() {
        if events[i].typ == "enter" {
            current += events[i].count;
        } else {
            current -= events[i].count;
        }
        let next_ts = if i + 1 < events.len() {
            events[i + 1].timestamp
        } else {
            events[i].timestamp
        };
        if current > max_occ {
            max_occ = current;
            best_start = events[i].timestamp;
            best_end = next_ts;
        }
    }
    println!("({}, {})", best_start, best_end);
}
