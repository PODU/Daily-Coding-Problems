// Day 420: n-th positive integer whose digits sum to exactly 10.
// Iterate integers, count those with digit sum 10. Time: O(answer), Space: O(1).
fn digit_sum(mut x: u64) -> u64 {
    let mut s = 0;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    s
}

fn nth_perfect(n: u64) -> u64 {
    let mut count = 0u64;
    let mut x = 0u64;
    while count < n {
        x += 1;
        if digit_sum(x) == 10 {
            count += 1;
        }
    }
    x
}

fn main() {
    println!("{}", nth_perfect(1)); // 19
    println!("{}", nth_perfect(2)); // 28
}
