// Day 801: nth sevenish number = sum of unique powers of 7.
// Bits of n select which powers of 7 to add (base-7 analog of binary).
// Time O(log n), Space O(1).

fn sevenish(mut n: u64) -> u64 {
    let mut result = 0u64;
    let mut power = 1u64;
    while n != 0 {
        if n & 1 == 1 {
            result += power;
        }
        power *= 7;
        n >>= 1;
    }
    result
}

fn main() {
    let parts: Vec<String> = (1..=5).map(|i| sevenish(i).to_string()).collect();
    println!("{}", parts.join(" ")); // 1 7 8 49 50
}
