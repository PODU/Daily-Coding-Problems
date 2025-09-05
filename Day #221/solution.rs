// Day 221: nth "sevenish" number (sum of distinct powers of 7).
// Approach: bits of n select which powers of 7 to include (bijection with binary). Time O(log n), Space O(1).
fn sevenish(mut n: u64) -> u64 {
    let mut result = 0u64;
    let mut power = 1u64; // 7^0
    while n > 0 {
        if n & 1 == 1 {
            result += power;
        }
        power *= 7;
        n >>= 1;
    }
    result
}

fn main() {
    let s: Vec<String> = (1..=5).map(|i| sevenish(i).to_string()).collect();
    println!("{}", s.join(" ")); // 1 7 8 49 50
    println!("{}", sevenish(4)); // 49
}
