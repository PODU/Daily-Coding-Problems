// Day 1772: Min jumps to reach N. Grow k until triangular sum >= |N| and (sum-|N|) even.
// Flipping any jump j changes parity of (sum-N) by 2j, so even surplus is reachable. O(sqrt(N)).

fn min_jumps(n: i64) -> i64 {
    let s = n.abs();
    let mut sum: i64 = 0;
    let mut k: i64 = 0;
    while sum < s || (sum - s) % 2 != 0 {
        k += 1;
        sum += k;
    }
    k
}

fn main() {
    println!("{}", min_jumps(10));
}
