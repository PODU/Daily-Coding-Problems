// Knight-on-board probability after k random moves: DP over board states.
// dp[r][c] = prob of being on (r,c); each move spreads 1/8 to each target.
// Time: O(k*64*8). Space: O(64).
const MOVES: [(i32, i32); 8] = [
    (-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1),
];

fn knight_probability(n: i32, k: i32, r0: i32, c0: i32) -> f64 {
    let ns = n as usize;
    let mut dp = vec![vec![0.0f64; ns]; ns];
    dp[r0 as usize][c0 as usize] = 1.0;
    for _ in 0..k {
        let mut nxt = vec![vec![0.0f64; ns]; ns];
        for r in 0..n {
            for c in 0..n {
                let p = dp[r as usize][c as usize];
                if p == 0.0 {
                    continue;
                }
                for &(dr, dc) in MOVES.iter() {
                    let nr = r + dr;
                    let nc = c + dc;
                    if nr >= 0 && nr < n && nc >= 0 && nc < n {
                        nxt[nr as usize][nc as usize] += p / 8.0;
                    }
                }
            }
        }
        dp = nxt;
    }
    dp.iter().flatten().sum()
}

fn main() {
    // corner (0,0), k=1 -> 2/8 = 0.25
    println!("{}", knight_probability(8, 1, 0, 0)); // 0.25
}
