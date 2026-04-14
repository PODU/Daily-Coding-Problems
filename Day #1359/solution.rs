// Reservoir sampling (size 1): i-th element (1-indexed) replaces pick with prob 1/i. O(1) space.
// Demo uses a portable 64-bit LCG seeded with 1 so output is deterministic across languages -> 7.

fn main() {
    let stream = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    const A: u64 = 6364136223846793005;
    const C: u64 = 1442695040888963407;
    let mut state: u64 = 1; // fixed seed

    let mut pick = 0;
    for i in 1..=stream.len() {
        state = state.wrapping_mul(A).wrapping_add(C); // advance LCG (mod 2^64)
        if state % (i as u64) == 0 {                   // replace with probability 1/i
            pick = stream[i - 1];
        }
    }
    println!("Selected: {}", pick);
}
