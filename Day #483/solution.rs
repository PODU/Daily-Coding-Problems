// Day 483: Josephus problem.
// Iterative recurrence f(i)=(f(i-1)+k)%i in O(n) time, O(1) space.
// Special O(log n) closed form when k=2.
fn josephus(n: u32, k: u32) -> u32 {
    let mut result = 0u32; // 0-indexed survivor among 1 person
    for i in 2..=n {
        result = (result + k) % i;
    }
    result + 1 // 1-indexed
}

// O(log n) special case for k == 2.
#[allow(dead_code)]
fn josephus_k2(n: u32) -> u32 {
    let mut p = 1u32;
    while p * 2 <= n {
        p *= 2;
    }
    2 * (n - p) + 1
}

fn main() {
    let (n, k) = (5u32, 2u32);
    println!("{}", josephus(n, k)); // 3
}
