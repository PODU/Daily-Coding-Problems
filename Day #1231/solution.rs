// Monte Carlo pi estimate with a shared 64-bit LCG. Time O(n), Space O(1).
const A: u64 = 6364136223846793005;
const C: u64 = 1442695040888963407;

fn estimate_pi(samples: u64, seed: u64) -> f64 {
    let mut x = seed;
    let mut inside: u64 = 0;
    let den = (1u64 << 53) as f64;
    for _ in 0..samples {
        x = A.wrapping_mul(x).wrapping_add(C);
        let px = (x >> 11) as f64 / den;
        x = A.wrapping_mul(x).wrapping_add(C);
        let py = (x >> 11) as f64 / den;
        if px * px + py * py <= 1.0 {
            inside += 1;
        }
    }
    4.0 * inside as f64 / samples as f64
}

fn main() {
    println!("{:.3}", estimate_pi(2_000_000, 42));
}
