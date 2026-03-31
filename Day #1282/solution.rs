// Day 1282: n-th positive integer whose digits sum to 10.
// Such numbers are ~ every 9th integer; iterate counting matches. Time O(answer/9), Space O(1).
fn digit_sum(mut x: u64) -> u32 {
    let mut s = 0;
    while x > 0 {
        s += (x % 10) as u32;
        x /= 10;
    }
    s
}

fn nth_perfect(n: u32) -> u64 {
    let mut x = 0u64;
    let mut count = 0u32;
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
