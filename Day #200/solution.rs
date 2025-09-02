// Day 200: Minimum points stabbing all intervals.
// Greedy: sort by right endpoint; pick the right end whenever current interval is unstabbed.
// Time: O(n log n), Space: O(1).
fn stab(intervals: &[(i64, i64)]) -> Vec<i64> {
    let mut iv = intervals.to_vec();
    iv.sort_by_key(|p| p.1);
    let mut pts = Vec::new();
    let mut last = i64::MIN;
    for (lo, hi) in iv {
        if lo > last {
            last = hi;
            pts.push(last);
        }
    }
    pts
}

fn main() {
    println!("{:?}", stab(&[(1, 4), (4, 5), (7, 9), (9, 12)])); // [4, 9]
}
