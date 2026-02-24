// Day 1124 - Minimum points to stab all intervals
// Greedy: sort by right endpoint, place a point at the end of each not-yet-
// stabbed interval. Time: O(n log n), Space: O(n).

fn stab(intervals: &[(i32, i32)]) -> Vec<i32> {
    let mut iv = intervals.to_vec();
    iv.sort_by_key(|x| x.1);
    let mut points = Vec::new();
    let mut last = i64::MIN;
    for (s, e) in iv {
        if (s as i64) > last {
            last = e as i64;
            points.push(e);
        }
    }
    points
}

fn main() {
    let intervals = [(1, 4), (4, 5), (7, 9), (9, 12)];
    println!("{:?}", stab(&intervals)); // [4, 9]
}
