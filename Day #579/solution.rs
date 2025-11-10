// Min jumps: smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even. Time O(sqrt(N)), Space O(1).
fn min_jumps(n: i64) -> i64 {
    let n = n.abs();
    let mut k = 0i64;
    let mut s = 0i64;
    while s < n || (s - n) % 2 != 0 {
        k += 1;
        s += k;
    }
    k
}

fn main() {
    println!("Minimum jumps to reach 10: {}", min_jumps(10));
}
