// Day 100: 8-directional steps between two points = Chebyshev distance
// max(|dx|,|dy|). Sum over consecutive points. O(n) time.
fn min_steps(pts: &[(i32, i32)]) -> i32 {
    pts.windows(2)
        .map(|w| (w[1].0 - w[0].0).abs().max((w[1].1 - w[0].1).abs()))
        .sum()
}

fn main() {
    println!("{}", min_steps(&[(0, 0), (1, 1), (1, 2)])); // 2
}
