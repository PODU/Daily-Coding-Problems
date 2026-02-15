// Iterative Fibonacci with two rolling variables. O(n) time, O(1) space.
fn fib(n: u32) -> u64 {
    if n == 0 { return 0; }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n { let c = a + b; a = b; b = c; }
    b
}

fn main() {
    let seq: Vec<String> = (0..=10).map(|i| fib(i).to_string()).collect();
    println!("{}", seq.join(" "));
    println!("fib(10) = {}", fib(10));
}
