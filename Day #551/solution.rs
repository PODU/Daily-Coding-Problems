// Von Neumann fair-coin from biased coin: toss pairs, (0,1)->0, (1,0)->1, else retry.
// Expected O(1) tosses per fair toss; O(1) space.

static mut RNG_STATE: u64 = 88172645463325252;

fn next_uniform() -> f64 { // xorshift64 -> [0,1)
    unsafe {
        RNG_STATE ^= RNG_STATE << 13;
        RNG_STATE ^= RNG_STATE >> 7;
        RNG_STATE ^= RNG_STATE << 17;
        (RNG_STATE >> 11) as f64 * (1.0 / 9007199254740992.0)
    }
}

fn toss_biased() -> i32 { // p(1) = 0.3
    if next_uniform() < 0.3 { 1 } else { 0 }
}

fn fair_toss() -> i32 {
    loop {
        let a = toss_biased();
        let b = toss_biased();
        if a == 0 && b == 1 { return 0; }
        if a == 1 && b == 0 { return 1; }
    }
}

fn main() {
    let mut heads: i64 = 0;
    let mut tails: i64 = 0;
    for _ in 0..100000 {
        if fair_toss() == 1 { heads += 1; } else { tails += 1; }
    }
    println!("heads: {}, tails: {}", heads, tails);
}
