// Sevenish: nth value = sum of 7^i for each set bit i of n. O(log n) per query.
fn sevenish(mut n: u64) -> u64 {
    let (mut sum, mut pow7) = (0u64, 1u64);
    while n > 0 {
        if n & 1 == 1 {
            sum += pow7;
        }
        pow7 *= 7;
        n >>= 1;
    }
    sum
}

fn main() {
    // First few sevenish numbers: 1, 7, 8, 49, ...
    let parts: Vec<String> = (1..=4).map(|n| sevenish(n).to_string()).collect();
    println!("{}", parts.join(", "));
}
