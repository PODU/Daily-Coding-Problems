// k nearest points: stable sort by squared Euclidean distance, take first k.
// Time O(N log N), Space O(N).
fn main() {
    let pts: Vec<(i64, i64)> = vec![(0, 0), (5, 4), (3, 1)];
    let (cx, cy, k) = (1i64, 2i64, 2usize);
    let dist = |p: &(i64, i64)| (p.0 - cx) * (p.0 - cx) + (p.1 - cy) * (p.1 - cy);
    let mut idx: Vec<usize> = (0..pts.len()).collect();
    idx.sort_by_key(|&i| dist(&pts[i])); // sort_by_key is stable
    let parts: Vec<String> = idx
        .iter()
        .take(k)
        .map(|&i| format!("({}, {})", pts[i].0, pts[i].1))
        .collect();
    println!("[{}]", parts.join(", "));
}
