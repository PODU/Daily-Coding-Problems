// Knight on board probability via DP over moves. prob(m,r,c)=avg of 8 neighbors.
// Time: O(k*64*8), Space: O(64).
fn knight_probability(k: usize, start_r: usize, start_c: usize) -> f64 {
    let moves = [(-2i32, -1i32), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];
    let mut prob = vec![vec![1.0f64; 8]; 8];
    for _ in 0..k {
        let mut next = vec![vec![0.0f64; 8]; 8];
        for r in 0..8 {
            for c in 0..8 {
                let mut s = 0.0;
                for &(dr, dc) in moves.iter() {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < 8 && nc >= 0 && nc < 8 {
                        s += prob[nr as usize][nc as usize];
                    }
                }
                next[r][c] = s / 8.0;
            }
        }
        prob = next;
    }
    prob[start_r][start_c]
}

fn main() {
    let ans = knight_probability(1, 0, 0);
    // Format trimming trailing zeros: 0.25
    let s = format!("{:.2}", ans);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    println!("{}", s);
}
