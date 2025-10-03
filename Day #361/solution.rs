// Day 361: Mastermind feasibility.
// Brute-force all 6-position codes with distinct digits; accept if some code
// matches every guess's score. Time O(P*G*6), P=151200, Space O(1).

fn score_of(code: &[u8; 6], guess: &[u8]) -> i32 {
    let mut s = 0;
    for i in 0..6 {
        if code[i] == guess[i] {
            s += 1;
        }
    }
    s
}

fn rec(code: &mut [u8; 6], pos: usize, used: u32, guesses: &[(&str, i32)]) -> bool {
    if pos == 6 {
        return guesses
            .iter()
            .all(|(g, s)| score_of(code, g.as_bytes()) == *s);
    }
    for d in 0..10u8 {
        if used & (1 << d) == 0 {
            code[pos] = b'0' + d;
            if rec(code, pos + 1, used | (1 << d), guesses) {
                return true;
            }
        }
    }
    false
}

fn feasible(guesses: &[(&str, i32)]) -> bool {
    rec(&mut [b'0'; 6], 0, 0, guesses)
}

fn main() {
    let g1 = [("175286", 2), ("293416", 3), ("654321", 0)];
    let g2 = [("123456", 4), ("345678", 4), ("567890", 4)];
    println!("{}", if feasible(&g1) { "True" } else { "False" });
    println!("{}", if feasible(&g2) { "True" } else { "False" });
}
