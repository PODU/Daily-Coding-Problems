// n-th perfect number (digits sum to 10): count up from 1 checking digit sums. Time O(answer * digits), Space O(1).
fn digit_sum(mut x: u64) -> u32 {
    let mut s = 0;
    while x > 0 {
        s += (x % 10) as u32;
        x /= 10;
    }
    s
}

fn nth_perfect(n: u32) -> u64 {
    let mut count = 0;
    let mut num: u64 = 0;
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
    println!("{}", nth_perfect(2));
}
