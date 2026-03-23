// Interval point stabbing: greedy, sort by right endpoint, pick endpoint when uncovered.
// Time O(n log n), Space O(n).
fn stab(intervals: &[(i64, i64)]) -> Vec<i64> {
    let mut iv = intervals.to_vec();
    iv.sort_by_key(|p| p.1);
    let mut pts = Vec::new();
    let mut last = i64::MIN;
    for (s, e) in iv {
        if s > last {
            last = e;
            pts.push(e);
        }
    }
    pts
}

fn main() {
    let pts = stab(&[(1, 4), (4, 5), (7, 9), (9, 12)]);
    let s: Vec<String> = pts.iter().map(|v| v.to_string()).collect();
    println!("[{}]", s.join(", "));
}
