// Day 673: K nearest points to a center. Sort by squared distance.
// Time O(n log n), Space O(n). No sqrt needed for comparison.
fn k_nearest(pts: &[(i64, i64)], c: (i64, i64), k: usize) -> Vec<(i64, i64)> {
    let d2 = |p: &(i64, i64)| {
        let (dx, dy) = (p.0 - c.0, p.1 - c.1);
        dx * dx + dy * dy
    };
    let mut idx: Vec<usize> = (0..pts.len()).collect();
    idx.sort_by(|&a, &b| d2(&pts[a]).cmp(&d2(&pts[b])).then(a.cmp(&b)));
    idx.into_iter().take(k).map(|i| pts[i]).collect()
}

fn main() {
    let pts = vec![(0, 0), (5, 4), (3, 1)];
    let r = k_nearest(&pts, (1, 2), 2);
    let s: Vec<String> = r.iter().map(|p| format!("({}, {})", p.0, p.1)).collect();
    println!("[{}]", s.join(", ")); // [(0, 0), (3, 1)]
}
