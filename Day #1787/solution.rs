// Josephus problem: iterative recurrence J(n)=(J(n-1)+k)%n, plus k=2 closed form.
// Time O(N) (O(log N) for k=2 closed form), Space O(1).
fn josephus(n: u64, k: u64) -> u64 {
    let mut r = 0u64;
    for i in 2..=n {
        r = (r + k) % i;
    }
    r + 1
}

fn josephus2(n: u64) -> u64 { // k==2 closed form
    let mut p = 1u64;
    while p * 2 <= n {
        p *= 2;
    }
    2 * (n - p) + 1
}

fn main() {
    let (n, k) = (5u64, 2u64);
    let mut ans = josephus(n, k);
    if k == 2 {
        ans = josephus2(n);
    }
    println!("{}", ans);
}
