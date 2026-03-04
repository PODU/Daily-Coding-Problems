// Day 1153: Min steps to visit points in order (8-directional moves).
// Sum of Chebyshev distances max(|dx|,|dy|) between consecutive points. O(n) time, O(1) space.
fn min_steps(pts: &[(i32, i32)]) -> i32 {
    pts.windows(2)
        .map(|w| (w[1].0 - w[0].0).abs().max((w[1].1 - w[0].1).abs()))
        .sum()
}

fn main() {
    let pts = [(0, 0), (1, 1), (1, 2)];
    println!("{}", min_steps(&pts)); // 2
}
