// Quxes: adjacent different colors merge to third. Smallest remaining count.
// Count r,g,b; distinct<=1 -> total; all same parity -> 2; else 1. Time O(n), Space O(1).
fn smallest_quxes(q: &[char]) -> usize {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for &c in q {
        match c {
            'R' => r += 1,
            'G' => g += 1,
            _ => b += 1,
        }
    }
    let distinct = (r > 0) as usize + (g > 0) as usize + (b > 0) as usize;
    if distinct <= 1 {
        return r + g + b;
    }
    if r % 2 == g % 2 && g % 2 == b % 2 {
        return 2;
    }
    1
}

fn main() {
    let q = vec!['R', 'G', 'B', 'G', 'B'];
    println!("{}", smallest_quxes(&q));
}
