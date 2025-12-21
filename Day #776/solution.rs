// Day 776: Josephus problem. General O(N) recurrence j=(j+k)%i.
// For k=2 an O(log N) closed form is also given. Returns 1-indexed survivor.
fn josephus(n: u32, k: u32) -> u32 {
    let mut r = 0;
    for i in 2..=n {
        r = (r + k) % i;
    }
    r + 1
}

fn josephus_k2(n: u32) -> u32 {
    let mut p = 1;
    while p * 2 <= n {
        p *= 2;
    }
    2 * (n - p) + 1
}

fn main() {
    println!("{}", josephus(5, 2)); // 3
    println!("{}", josephus_k2(5)); // 3
}
