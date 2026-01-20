// Iterate integers, sum digits, count until the n-th whose digit sum is 10.
// Time O(answer * digits), Space O(1).
fn digit_sum(mut x: u64) -> u64 {
    let mut s = 0;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    s
}

fn nth_perfect(n: u64) -> u64 {
    let mut count = 0;
    let mut num = 0u64;
    while count < n {
        num += 1;
        if digit_sum(num) == 10 {
            count += 1;
        }
    }
    num
}

fn main() {
    println!("{}", nth_perfect(1));
}
