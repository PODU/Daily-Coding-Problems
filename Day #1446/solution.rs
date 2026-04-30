// Day 1446: Minimum set of points hitting every closed interval.
// Greedy: sort by right endpoint; whenever the current interval is unhit, pick
// its right endpoint. Time O(n log n), Space O(1). (Any minimal set is valid.)
fn min_stabbing_set(mut intervals: Vec<(i64, i64)>) -> Vec<i64> {
    intervals.sort_by_key(|iv| iv.1);
    let mut points = Vec::new();
    let mut last = i64::MIN;
    for (l, r) in intervals {
        if l > last {
            points.push(r);
            last = r;
        }
    }
    points
}

fn main() {
    let intervals = vec![(0, 3), (2, 6), (3, 4), (6, 9)];
    let pts = min_stabbing_set(intervals);
    let strs: Vec<String> = pts.iter().map(|p| p.to_string()).collect();
    println!("{{{}}}", strs.join(", ")); // e.g. {3, 9}; {3, 6} also valid
}
