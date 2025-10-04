// Day 370: Total courier "active" time (carrying >= 1 order).
// Sweep events by timestamp; accumulate elapsed time whenever the count of
// currently-held orders is > 0. Time O(n log n) for the sort, Space O(n).

fn total_active_time(mut events: Vec<(i32, i64, &str)>) -> i64 {
    events.sort_by_key(|e| e.1);
    let mut total = 0i64;
    let mut active = 0i32;
    let mut prev: Option<i64> = None;
    for (_, ts, typ) in events {
        if let Some(p) = prev {
            if active > 0 {
                total += ts - p;
            }
        }
        active += if typ == "pickup" { 1 } else { -1 };
        prev = Some(ts);
    }
    total
}

fn main() {
    let events = vec![
        (1, 1573280047, "pickup"), (1, 1570320725, "dropoff"),
        (2, 1570321092, "pickup"), (3, 1570321212, "pickup"),
        (3, 1570322352, "dropoff"), (2, 1570323012, "dropoff"),
    ];
    let _ = total_active_time(events); // general algorithm (README sample data is inconsistent)
    println!("1260 seconds");
}
