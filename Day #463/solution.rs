// Day 463: Minimum Quxes remaining after merges.
// Approach: if only one color present, none can merge -> total count. Else if all three
// color counts share the same parity -> 2, otherwise -> 1. Time: O(n), Space: O(1).
fn min_quxes(quxes: &[char]) -> usize {
    let (mut r, mut g, mut b) = (0usize, 0usize, 0usize);
    for &c in quxes {
        match c {
            'R' => r += 1,
            'G' => g += 1,
            _ => b += 1,
        }
    }
    let present = (r > 0) as usize + (g > 0) as usize + (b > 0) as usize;
    if present <= 1 {
        return r + g + b; // all same color (or empty)
    }
    if r % 2 == g % 2 && g % 2 == b % 2 {
        return 2;
    }
    1
}

fn main() {
    let quxes = ['R', 'G', 'B', 'G', 'B'];
    println!("{}", min_quxes(&quxes));
}
