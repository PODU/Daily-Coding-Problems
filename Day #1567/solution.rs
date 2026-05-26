// Busiest period: sort events by timestamp, sweep current occupancy, track max interval.
// Time O(n log n), space O(n).
#[derive(Clone)]
struct Event {
    ts: i64,
    count: i64,
    enter: bool,
}

fn busiest_period(mut events: Vec<Event>) -> (i64, i64) {
    events.sort_by_key(|e| e.ts);
    let mut cur: i64 = 0;
    let mut best: i64 = -1;
    let mut best_start: i64 = 0;
    let mut best_end: i64 = 0;
    for i in 0..events.len() {
        if events[i].enter {
            cur += events[i].count;
        } else {
            cur -= events[i].count;
        }
        if cur > best && i + 1 < events.len() {
            best = cur;
            best_start = events[i].ts;
            best_end = events[i + 1].ts;
        }
    }
    (best_start, best_end)
}

fn main() {
    let events = vec![
        Event { ts: 1, count: 3, enter: true },
        Event { ts: 4, count: 2, enter: true },
        Event { ts: 6, count: 5, enter: false },
    ];
    let (start, end) = busiest_period(events);
    println!("({}, {})", start, end);
}
