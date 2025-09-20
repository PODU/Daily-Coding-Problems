// Day 304: Probability knight stays on board after k random moves. DP over board.
// Time O(k*N^2*8), space O(N^2).
fn knight_prob(n: usize, k: usize, sr: usize, sc: usize) -> f64 {
    let mut dp = vec![vec![0.0f64; n]; n];
    dp[sr][sc] = 1.0;
    let moves = [(1i32, 2i32), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)];
    for _ in 0..k {
        let mut nd = vec![vec![0.0f64; n]; n];
        for r in 0..n {
            for c in 0..n {
                if dp[r][c] > 0.0 {
                    for (dr, dc) in moves.iter() {
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;
                        if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                            nd[nr as usize][nc as usize] += dp[r][c] / 8.0;
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
    println!("{}", knight_prob(8, 1, 0, 0)); // 0.25
}
