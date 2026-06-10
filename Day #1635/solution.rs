// Minimum stabbing points: greedy sort intervals by right endpoint; add right
// endpoint when current interval is unstabbed. Time O(n log n), Space O(n).

fn min_stabbing_points(intervals: &[(i64, i64)]) -> Vec<i64> {
    let mut sorted = intervals.to_vec();
    sorted.sort_by_key(|iv| iv.1);
    let mut points = Vec::new();
    let mut last = i64::MIN;
    for (a, b) in sorted {
        if a > last {
            points.push(b);
            last = b;
        }
    }
    points
}

fn main() {
    let intervals = [(1, 4), (4, 5), (7, 9), (9, 12)];
    let points = min_stabbing_points(&intervals);
    let parts: Vec<String> = points.iter().map(|p| p.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
