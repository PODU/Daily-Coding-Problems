// Day 1120 - Minimum steps to cover points in order (8-directional moves)
// Cost between two points is Chebyshev distance max(|dx|,|dy|); sum them.
// Time: O(n), Space: O(1).

fn min_steps(points: &[(i32, i32)]) -> i32 {
    let mut total = 0;
    for w in points.windows(2) {
        let dx = (w[1].0 - w[0].0).abs();
        let dy = (w[1].1 - w[0].1).abs();
        total += dx.max(dy);
    }
    total
}

fn main() {
    let points = vec![(0, 0), (1, 1), (1, 2)];
    println!("{}", min_steps(&points)); // 2
}
