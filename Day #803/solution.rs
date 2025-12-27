// Day 803: Does a secret (6 distinct digits) exist matching all guess scores?
// Brute force all 6-digit distinct-digit codes, verify every guess's score.
// Time O(10^6 * G), Space O(G).

fn digits(mut x: u32) -> [u8; 6] {
    let mut d = [0u8; 6];
    for i in (0..6).rev() {
        d[i] = (x % 10) as u8;
        x /= 10;
    }
    d
}

fn distinct(d: &[u8; 6]) -> bool {
    let mut seen = 0u16;
    for &v in d {
        if seen & (1 << v) != 0 {
            return false;
        }
        seen |= 1 << v;
    }
    true
}

fn score(code: &[u8; 6], guess: &[u8; 6]) -> u8 {
    (0..6).filter(|&i| code[i] == guess[i]).count() as u8
}

fn feasible(guesses: &[(u32, u8)]) -> bool {
    let gs: Vec<([u8; 6], u8)> = guesses.iter().map(|&(g, sc)| (digits(g), sc)).collect();
    for code in 0..=999999u32 {
        let d = digits(code);
        if !distinct(&d) {
            continue;
        }
        if gs.iter().all(|(gd, sc)| score(&d, gd) == *sc) {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", feasible(&[(175286, 2), (293416, 3), (654321, 0)])); // true
    println!("{}", feasible(&[(123456, 4), (345678, 4), (567890, 4)])); // false
}
