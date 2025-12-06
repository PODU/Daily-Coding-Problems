// Day 703: Probability a knight stays on an 8x8 board after k random moves.
// Approach: DP over board cells; each move spreads prob/8 to valid targets.
// Time O(k * N^2 * 8), Space O(N^2).
fn knight_probability(n: usize, k: usize, r: usize, c: usize) -> f64 {
    let moves = [
        (1i32, 2i32), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1),
    ];
    let mut dp = vec![vec![0.0f64; n]; n];
    dp[r][c] = 1.0;
    for _ in 0..k {
        let mut nd = vec![vec![0.0f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                if dp[i][j] > 0.0 {
                    for &(dr, dc) in &moves {
                        let ni = i as i32 + dr;
                        let nj = j as i32 + dc;
                        if ni >= 0 && ni < n as i32 && nj >= 0 && nj < n as i32 {
                            nd[ni as usize][nj as usize] += dp[i][j] / 8.0;
                        }
                    }
                }
            }
        }
        dp = nd;
    }
    dp.iter().flatten().sum()
}

fn main() {
    println!("{}", knight_probability(8, 2, 0, 0)); // 0.1875
}
