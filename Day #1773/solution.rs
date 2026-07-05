// Day 1773: Count ordered positive pairs (a,b) with a+b=M, a^b=N.
// Use a+b=(a^b)+2*(a&b): s=(M-N)/2 must satisfy (s&N)==0; answer=2^popcount(N)
// minus the all-or-nothing assignments when s==0. O(1) time, O(1) space.
fn count_pairs(m: i64, n: i64) -> i64 {
    if m - n < 0 || (m - n) & 1 == 1 {
        return 0;
    }
    let s = (m - n) / 2; // s == a & b
    if s & n != 0 {
        return 0; // carry bits disjoint from xor bits
    }
    if n == 0 {
        return if m > 0 && m % 2 == 0 { 1 } else { 0 }; // only (M/2, M/2)
    }
    let mut ways = 1i64 << n.count_ones();
    if s == 0 {
        ways -= 2; // drop a=0 and b=0 to keep both positive
    }
    ways
}

fn main() {
    println!("{}", count_pairs(6, 4)); // -> 2
}
