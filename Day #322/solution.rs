// Min jumps on number line: smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even.
// Time: O(sqrt N), Space: O(1).
fn min_jumps(n: i64) -> i64 {
    let target = n.abs();
    let mut k: i64 = 0;
    let mut s: i64 = 0;
    while s < target || (s - target) % 2 != 0 {
        k += 1;
        s += k;
    }
    k
}

fn main() {
    let n: i64 = 10;
    println!("Minimum jumps to reach {}: {}", n, min_jumps(n));
}
