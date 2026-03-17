// Iterative rolling Fibonacci: two variables, O(n) time, O(1) space.
// fib(0)=0, fib(1)=1.
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
    println!("{}", fib(10));
}
