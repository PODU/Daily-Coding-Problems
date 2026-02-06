// Day 1030: Quxes minimum remaining. Count colors; parity-based O(N) formula.
// If two counts are 0 -> n; else if all parities equal -> 2; else -> 1. Time O(N), Space O(1).
fn min_quxes(q: &[char]) -> usize {
    let mut r = 0usize;
    let mut g = 0usize;
    let mut b = 0usize;
    for &c in q {
        match c {
            'R' => r += 1,
            'G' => g += 1,
            _ => b += 1,
        }
    }
    let n = r + g + b;
    let zeros = (r == 0) as usize + (g == 0) as usize + (b == 0) as usize;
    if zeros >= 2 {
        return n;
    }
    if r % 2 == g % 2 && g % 2 == b % 2 {
        return 2;
    }
    1
}

fn main() {
    let q = ['R', 'G', 'B', 'G', 'B'];
    println!("{}", min_quxes(&q));
}
