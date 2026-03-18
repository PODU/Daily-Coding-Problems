// Merge overlapping intervals: sort by start, sweep merging when start <= last end.
// Time: O(n log n), Space: O(n).
fn merge(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    intervals.sort();
    let mut res: Vec<(i32, i32)> = Vec::new();
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
    let data = vec![(1, 3), (5, 8), (4, 10), (20, 25)];
    let out = merge(data);
    let parts: Vec<String> = out.iter().map(|(s, e)| format!("({}, {})", s, e)).collect();
    println!("[{}]", parts.join(", "));
}
