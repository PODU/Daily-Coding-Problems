// Nearest k points: max-heap of size k by squared distance. Time O(n log k), Space O(k).
use std::collections::BinaryHeap;

fn main() {
    let points = vec![(0i64, 0i64), (5, 4), (3, 1)];
    let central = (1i64, 2i64);
    let k = 2usize;

    // BinaryHeap is a max-heap; key by squared distance.
    let mut heap: BinaryHeap<(i64, i64, i64)> = BinaryHeap::new();
    for &(x, y) in &points {
        let dx = x - central.0;
        let dy = y - central.1;
        heap.push((dx * dx + dy * dy, x, y));
        if heap.len() > k {
            heap.pop();
        }
    }
    let mut res: Vec<(i64, i64)> = heap.iter().map(|&(_, x, y)| (x, y)).collect();
    res.sort();

    let parts: Vec<String> = res.iter().map(|&(x, y)| format!("({}, {})", x, y)).collect();
    println!("[{}]", parts.join(", "));
}
