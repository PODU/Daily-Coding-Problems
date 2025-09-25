// Count ordered (a,b), a+b=M, a^b=N. c=(M-N)/2; valid if c&N==0.
// Count=2^popcount(N), minus 2 if M==N. Time O(1), Space O(1).
fn count_pairs(m: i64, n: i64) -> i64 {
    let diff = m - n;
    if diff < 0 || diff & 1 == 1 {
        return 0;
    }
    let c = diff / 2;
    if c & n != 0 {
        return 0;
    }
    let mut count: i64 = 1 << n.count_ones();
    if m == n {
        count -= 2;
    }
    if count < 0 { 0 } else { count }
}

fn main() {
    println!("{}", count_pairs(10, 4));
}
