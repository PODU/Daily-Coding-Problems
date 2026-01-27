// Day 968: Fewest jumps from 0 to N where i-th jump moves +/- i.
// Approach: smallest k with sum 1..k >= |N| and (sum-|N|) even. Time O(sqrt(N)), Space O(1).

fn min_jumps(n: i64) -> i64 {
    let n = n.abs();
    let mut k = 0i64;
    let mut sum = 0i64;
    while sum < n || (sum - n) % 2 != 0 {
        k += 1;
        sum += k;
    }
    k
}

fn main() {
    println!("{}", min_jumps(10)); // 4  (1+2+3+4=10)
}
