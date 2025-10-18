// Day 456: Busiest period in a building from enter/exit events.
// Sort by timestamp, sweep occupancy, track interval of max. Time O(n log n).
struct Event {
    ts: i64,
    count: i64,
    typ: &'static str,
}

fn busiest(mut ev: Vec<Event>) -> (i64, i64) {
    ev.sort_by_key(|e| e.ts);
    let (mut cur, mut best) = (0i64, -1i64);
    let (mut best_start, mut best_end) = (0i64, 0i64);
    let n = ev.len();
    for i in 0..n {
        cur += if ev[i].typ == "enter" { ev[i].count } else { -ev[i].count };
        let end = if i + 1 < n { ev[i + 1].ts } else { ev[i].ts };
        if cur > best {
            best = cur;
            best_start = ev[i].ts;
            best_end = end;
        }
    }
    (best_start, best_end)
}

fn main() {
    let ev = vec![
        Event { ts: 1526579928, count: 3, typ: "enter" },
        Event { ts: 1526579940, count: 2, typ: "enter" },
        Event { ts: 1526580000, count: 1, typ: "exit" },
        Event { ts: 1526580382, count: 4, typ: "exit" },
    ];
    let (s, e) = busiest(ev);
    println!("({}, {})", s, e); // (1526579940, 1526580000)
}
