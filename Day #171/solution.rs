// Busiest period: sort events by timestamp, sweep occupancy, track max-occupancy interval. O(n log n) time, O(n) space.

#[derive(Clone, Copy, PartialEq)]
enum Type { Enter, Exit }

#[derive(Clone, Copy)]
struct Event { ts: i64, count: i64, typ: Type }

fn busiest_period(mut events: Vec<Event>) -> (i64, i64) {
    events.sort_by_key(|e| e.ts);
    let mut occ: i64 = 0;
    let mut max_occ: i64 = -1;
    let mut best = (0i64, 0i64);
    for i in 0..events.len() {
        let e = events[i];
        occ += if e.typ == Type::Enter { e.count } else { -e.count };
        if occ > max_occ && i + 1 < events.len() {
            max_occ = occ;
            best = (e.ts, events[i + 1].ts);
        }
    }
    best
}

fn main() {
    let events = vec![
        Event { ts: 1526579928, count: 3, typ: Type::Enter },
        Event { ts: 1526580382, count: 2, typ: Type::Exit },
        Event { ts: 1526579999, count: 1, typ: Type::Enter },
        Event { ts: 1526580001, count: 5, typ: Type::Enter },
    ];
    let (start, end) = busiest_period(events);
    println!("({}, {})", start, end);
}
