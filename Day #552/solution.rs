// Domino + Tromino tiling of 2xN: f(n)=2*f(n-1)+f(n-3), f(0)=1,f(1)=1,f(2)=2.
// O(n) time, O(1) space.

fn tilings(n: i32) -> i64 {
    if n == 0 { return 1; }
    if n == 1 { return 1; }
    if n == 2 { return 2; }
    let (mut a, mut b, mut c): (i64, i64, i64) = (1, 1, 2);
    let mut cur = c;
    for _ in 3..=n {
        cur = 2 * c + a;
        a = b; b = c; c = cur;
    }
    let _ = b;
    cur
}

fn main() {
    let n = 4;
    println!("{}", tilings(n)); // 11
}
