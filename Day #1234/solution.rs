// Min Quxes remaining. One color -> N; all counts same parity -> 2; else 1.
// Time O(n), Space O(1).
fn min_quxes(q: &str) -> usize {
    if q.is_empty() {
        return 0;
    }
    let (mut r, mut g, mut b) = (0usize, 0usize, 0usize);
    for c in q.chars() {
        match c {
            'R' => r += 1,
            'G' => g += 1,
            _ => b += 1,
        }
    }
    let distinct = (r > 0) as usize + (g > 0) as usize + (b > 0) as usize;
    if distinct == 1 {
        return q.chars().count();
    }
    if r % 2 == g % 2 && g % 2 == b % 2 {
        2
    } else {
        1
    }
}

fn main() {
    println!("{}", min_quxes("RGBGB"));
}
