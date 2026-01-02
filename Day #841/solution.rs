// Day 841: count positive integer pairs (a,b) with a+b=M and a^b=N.
// Math: a+b = (a^b) + 2*(a&b) => a&b=(M-N)/2; answer = 2^popcount(N) minus zero-cases. O(1).
fn count_pairs(m: i64, n: i64) -> i64 {
    let d = m - n;
    if d < 0 || d & 1 != 0 {
        return 0;
    }
    let c = d / 2; // c = a & b
    if c & n != 0 {
        return 0;
    }
    let mut res: i64 = 1 << n.count_ones();
    if c == 0 {
        res -= if n != 0 { 2 } else { 1 };
    }
    if res < 0 {
        0
    } else {
        res
    }
}

fn main() {
    println!("{}", count_pairs(4, 2)); // 2
}
