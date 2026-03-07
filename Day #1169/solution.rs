// K nearest points: max-heap (BinaryHeap) of size k by squared Euclidean
// distance, then sort the k results by (dist, original index) deterministically.
// Time: O(n log k), Space: O(k).
use std::collections::BinaryHeap;

fn main() {
    let points: Vec<(i64, i64)> = vec![(0, 0), (5, 4), (3, 1)];
    let (cx, cy) = (1i64, 2i64);
    let k = 2usize;

    let dist2 = |p: &(i64, i64)| -> i64 {
        let dx = p.0 - cx;
        let dy = p.1 - cy;
        dx * dx + dy * dy
    };

    // max-heap of (dist, index) keeping k smallest
    let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    for (i, p) in points.iter().enumerate() {
        heap.push((dist2(p), i));
        if heap.len() > k {
            heap.pop();
        }
    }

    let mut idx: Vec<usize> = heap.into_iter().map(|(_, i)| i).collect();
    idx.sort_by(|&a, &b| {
        let da = dist2(&points[a]);
        let db = dist2(&points[b]);
        da.cmp(&db).then(a.cmp(&b))
    });

    let parts: Vec<String> = idx
        .iter()
        .map(|&i| format!("({}, {})", points[i].0, points[i].1))
        .collect();
    println!("[{}]", parts.join(", "));
}
