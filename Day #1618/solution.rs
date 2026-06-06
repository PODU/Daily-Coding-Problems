// Sevenish: write n in binary; each set bit k contributes 7^k. Time O(log n), Space O(1).
fn sevenish(mut n: u64) -> u64 {
    let mut total = 0u64;
    let mut pow7 = 1u64;
    while n > 0 {
        if n & 1 == 1 {
            total += pow7;
        }
        pow7 *= 7;
        n >>= 1;
    }
    total
}

fn main() {
    let parts: Vec<String> = (1..=5).map(|n| sevenish(n).to_string()).collect();
    println!("{}", parts.join(" "));
}
