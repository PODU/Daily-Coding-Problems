// Merge overlapping intervals: sort by start, merge when next.start <= current.end.
// Time: O(n log n), Space: O(n).

fn merge(mut intervals: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    intervals.sort();
    let mut res: Vec<(i64, i64)> = Vec::new();
    for (s, e) in intervals {
        if let Some(last) = res.last_mut() {
            if s <= last.1 {
                last.1 = last.1.max(e);
                continue;
            }
        }
        res.push((s, e));
    }
    res
}

fn main() {
    let merged = merge(vec![(1, 3), (5, 8), (4, 10), (20, 25)]);
    let parts: Vec<String> = merged.iter().map(|(a, b)| format!("({}, {})", a, b)).collect();
    println!("[{}]", parts.join(", "));
}
