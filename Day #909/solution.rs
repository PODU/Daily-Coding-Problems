// Minimum points covering all closed intervals: greedy, sort by END ascending; open a new
// group when start>anchor-end, pick each group's MAX start as its point. Time O(n log n), Space O(n).

fn min_points(mut intervals: Vec<(i64, i64)>) -> Vec<i64> {
    intervals.sort_by_key(|iv| iv.1);
    let mut points = Vec::new();
    let mut have = false;
    let (mut anchor_end, mut group_max_start) = (i64::MIN, i64::MIN);
    for (start, end) in intervals {
        if !have || start > anchor_end {
            if have {
                points.push(group_max_start);
            }
            have = true;
            anchor_end = end;
            group_max_start = start;
        } else if start > group_max_start {
            group_max_start = start;
        }
    }
    if have {
        points.push(group_max_start);
    }
    points
}

fn main() {
    let intervals = vec![(0, 3), (2, 6), (3, 4), (6, 9)];
    let pts = min_points(intervals);
    let strs: Vec<String> = pts.iter().map(|p| p.to_string()).collect();
    println!("{{{}}}", strs.join(", "));
}
