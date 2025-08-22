// K nearest points: max-heap (BinaryHeap) of size k keyed by squared distance. Time O(n log k), Space O(k).
use std::collections::BinaryHeap;

fn k_nearest(pts: &[(i64, i64)], c: (i64, i64), k: usize) -> Vec<(i64, i64)> {
    let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new(); // max-heap on (dist2, index)
    for (i, &(x, y)) in pts.iter().enumerate() {
        let dx = x - c.0;
        let dy = y - c.1;
        heap.push((dx * dx + dy * dy, i));
        if heap.len() > k {
            heap.pop();
        }
    }
    let mut idx: Vec<usize> = heap.into_iter().map(|(_, i)| i).collect();
    idx.sort(); // keep original order for stable output
    idx.into_iter().map(|i| pts[i]).collect()
}

fn main() {
    let pts = vec![(0i64, 0i64), (5, 4), (3, 1)];
    let c = (1i64, 2i64);
    let k = 2;
    let res = k_nearest(&pts, c, k);
    let parts: Vec<String> = res.iter().map(|(x, y)| format!("({}, {})", x, y)).collect();
    println!("[{}]", parts.join(", "));
}
