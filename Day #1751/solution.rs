// Day 1751: Min steps to visit points in order on an 8-directional grid.
// Sum of Chebyshev distances max(|dx|,|dy|) between consecutive points. O(n) time, O(1) space.

fn min_steps(pts: &[(i32, i32)]) -> i32 {
    let mut total = 0;
    for w in pts.windows(2) {
        let dx = (w[1].0 - w[0].0).abs();
        let dy = (w[1].1 - w[0].1).abs();
        total += dx.max(dy);
    }
    total
}

fn main() {
    let pts = [(0, 0), (1, 1), (1, 2)];
    println!("{}", min_steps(&pts)); // 2
}
