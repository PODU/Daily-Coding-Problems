// Day 77: Merge overlapping intervals. Sort by start, then sweep merging.
// Time O(n log n), Space O(n).
fn merge_intervals(mut iv: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    iv.sort();
    let mut res: Vec<(i32, i32)> = Vec::new();
    for (start, end) in iv {
        if let Some(last) = res.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        res.push((start, end));
    }
    res
}

fn main() {
    let inp = vec![(1, 3), (5, 8), (4, 10), (20, 25)];
    let out = merge_intervals(inp);
    let parts: Vec<String> = out.iter().map(|(a, b)| format!("({}, {})", a, b)).collect();
    println!("[{}]", parts.join(", ")); // [(1, 3), (4, 10), (20, 25)]
}
