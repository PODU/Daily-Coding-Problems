// Quxes minimization: count R/G/B; two zero counts -> n; all parities equal -> 2; else 1.
// Time O(n), Space O(1).
fn min_quxes(a: &[char]) -> usize {
    let (mut r, mut g, mut b) = (0usize, 0usize, 0usize);
    for &c in a {
        match c {
            'R' => r += 1,
            'G' => g += 1,
            _ => b += 1,
        }
    }
    let n = a.len();
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
    let demo = ['R', 'G', 'B', 'G', 'B'];
    println!("{}", min_quxes(&demo));
}
