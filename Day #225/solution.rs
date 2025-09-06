// Day 225: Josephus problem - position (1-indexed) of last survivor.
// Approach: general O(N) recurrence J(i)=(J(i-1)+k)%i. Bonus: k==2 closed form O(log N): 2*(n-2^floor(log2 n))+1.
fn josephus(n: u32, k: u32) -> u32 {
    if k == 2 {
        let mut p = 1;
        while p * 2 <= n {
            p *= 2; // highest power of 2 <= n
        }
        return 2 * (n - p) + 1;
    }
    let mut res = 0u32; // 0-indexed
    for i in 2..=n {
        res = (res + k) % i;
    }
    res + 1
}

fn main() {
    println!("{}", josephus(5, 2)); // 3
}
