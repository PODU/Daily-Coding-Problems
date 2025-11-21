// Day 637: Merge overlapping intervals.
// Approach: sort by start, sweep merging while next.start <= cur.end.
// Time: O(n log n), Space: O(n).
fn merge(mut iv: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    iv.sort();
    let mut res: Vec<(i32, i32)> = Vec::new();
    for (s, e) in iv {
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
    let iv = vec![(1, 3), (5, 8), (4, 10), (20, 25)];
    let r = merge(iv);
    let parts: Vec<String> = r.iter().map(|(s, e)| format!("({}, {})", s, e)).collect();
    println!("[{}]", parts.join(", "));
}
