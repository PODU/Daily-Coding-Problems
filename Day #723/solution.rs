// Day 723: Smallest set of points stabbing all closed intervals.
// Approach: Sort by right endpoint; greedily pick the end of each uncovered interval.
// Time: O(n log n), Space: O(n). (Any minimum-size set is valid; {3,9} here.)

fn min_stabbing_points(mut intervals: Vec<(i64, i64)>) -> Vec<i64> {
    intervals.sort_by_key(|iv| iv.1);
    let mut points = Vec::new();
    let mut last = i64::MIN;
    for (s, e) in intervals {
        if s > last {
            last = e;
            points.push(e);
        }
    }
    points
}

fn main() {
    let intervals = vec![(0, 3), (2, 6), (3, 4), (6, 9)];
    let pts = min_stabbing_points(intervals);
    let s: Vec<String> = pts.iter().map(|p| p.to_string()).collect();
    println!("{{{}}}", s.join(", "));
}
