// Day 1301: Fewest jumps from 0 to N where the ith jump moves exactly i (left/right).
// Find smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even (flipping a jump changes sum by 2*val).
fn min_jumps(n: i64) -> i64 {
    let n = n.abs();
    let (mut k, mut sum) = (0i64, 0i64);
    while sum < n || (sum - n) % 2 != 0 {
        k += 1;
        sum += k;
    }
    k
}

fn main() {
    println!("{}", min_jumps(3)); // 2
    println!("{}", min_jumps(2)); // 3
}
