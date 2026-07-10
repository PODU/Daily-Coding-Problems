// fib(n): iterative two-variable rolling sum.
// Time O(N), Space O(1).
fn fib(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        let t = a + b;
        a = b;
        b = t;
    }
    a
}

fn main() {
    println!("{}", fib(10));
}
