// Day 451: nth Fibonacci number in O(1) space.
// Iterative rolling pair. Time O(n), Space O(1).
fn fib(n: u32) -> u64 {
    if n < 2 {
        return n as u64;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

fn main() {
    println!("{}", fib(10)); // 55
}
