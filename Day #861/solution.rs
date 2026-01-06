// Day 861: Probability a knight stays on an 8x8 board after k random moves.
// Approach: DP over moves; dp[r][c] = prob of being on this cell, spread 1/8 each move.
// Time: O(k * N^2), Space: O(N^2).
const MOVES: [(i32, i32); 8] = [
    (-2, -1), (-2, 1), (-1, -2), (-1, 2),
    (1, -2), (1, 2), (2, -1), (2, 1),
];

fn knight_probability(n: usize, k: usize, sr: usize, sc: usize) -> f64 {
    let mut dp = vec![vec![0.0f64; n]; n];
    dp[sr][sc] = 1.0;
    for _ in 0..k {
        let mut nx = vec![vec![0.0f64; n]; n];
        for r in 0..n {
            for c in 0..n {
                if dp[r][c] > 0.0 {
                    for &(dr, dc) in MOVES.iter() {
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;
                        if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                            nx[nr as usize][nc as usize] += dp[r][c] / 8.0;
                        }
                    }
                }
            }
        }
        dp = nx;
    }
    dp.iter().flatten().sum()
}

fn main() {
    println!("{}", knight_probability(8, 1, 0, 0)); // 0.25
    println!("{}", knight_probability(8, 2, 0, 0));
}
