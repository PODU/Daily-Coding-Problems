// Approximate median: sample a fixed, sublinear subset (size independent of N),
// return the sample's median -> lands in the central half [N/4, 3N/4] w.h.p.
// Sampling+median: O(s log s), sublinear in N. (Rank shown only for the demo.)

static mut STATE: u64 = 0x0123456789ABCDEF; // fixed seed -> deterministic

fn next_rand() -> u64 {
    unsafe {
        STATE = STATE
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        STATE >> 33 // top 31 bits
    }
}

fn main() {
    const N: usize = 1000;
    const SAMPLE_SIZE: usize = 99; // fixed, independent of N (sublinear)

    let mut data: Vec<i32> = (1..=N as i32).collect();
    for i in (1..N).rev() {
        let j = (next_rand() % (i as u64 + 1)) as usize;
        data.swap(i, j);
    }

    let mut sample: Vec<i32> = Vec::with_capacity(SAMPLE_SIZE);
    for _ in 0..SAMPLE_SIZE {
        let idx = (next_rand() % N as u64) as usize;
        sample.push(data[idx]);
    }
    sample.sort();
    let median = sample[SAMPLE_SIZE / 2];

    let rank = data.iter().filter(|&&v| v <= median).count();

    println!("Approximate median: {}", median);
    println!(
        "Rank: {} (acceptable range: {} to {})",
        rank,
        N / 4,
        3 * N / 4
    );
}
