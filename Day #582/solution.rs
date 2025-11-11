// Greedy interval stabbing: sort by right endpoint, pick right end when uncovered. Time O(n log n).
fn stab(mut intervals: Vec<(i32, i32)>) -> Vec<i32> {
    intervals.sort_by_key(|iv| iv.1);
    let mut points = Vec::new();
    let mut last = i32::MIN;
    for (start, end) in intervals {
        if start > last {
            last = end;
            points.push(end);
        }
    }
    points
}

fn main() {
    let intervals = vec![(1, 4), (4, 5), (7, 9), (9, 12)];
    let pts = stab(intervals);
    let parts: Vec<String> = pts.iter().map(|x| x.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
