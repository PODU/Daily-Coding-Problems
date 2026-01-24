// Day 950: busiest period - interval with the most people inside the building.
// Sort events by timestamp, sweep maintaining running count. Time O(n log n), Space O(n).

#[derive(Clone)]
struct Event {
    ts: i64,
    count: i64,
    enter: bool,
}

fn busiest(mut ev: Vec<Event>) -> (i64, i64) {
    ev.sort_by_key(|e| e.ts);
    let mut cur: i64 = 0;
    let mut best = i64::MIN;
    let mut ans = (0i64, 0i64);
    for i in 0..ev.len() {
        cur += if ev[i].enter { ev[i].count } else { -ev[i].count };
        let next_ts = if i + 1 < ev.len() { ev[i + 1].ts } else { ev[i].ts };
        if cur > best {
            best = cur;
            ans = (ev[i].ts, next_ts);
        }
    }
    ans
}

fn main() {
    let ev = vec![
        Event { ts: 1526579928, count: 3, enter: true },
        Event { ts: 1526579943, count: 4, enter: true },
        Event { ts: 1526580382, count: 2, enter: false },
        Event { ts: 1526581000, count: 5, enter: false },
    ];
    let (s, e) = busiest(ev);
    println!("({}, {})", s, e); // (1526579943, 1526580382)
}
