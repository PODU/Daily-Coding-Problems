// Josephus problem via iterative recurrence j(i)=(j(i-1)+k)%i, answer j(N)+1. O(N) time, O(1) space.

fn josephus(n: usize, k: usize) -> usize {
    let mut res = 0;
    for i in 2..=n {
        res = (res + k) % i;
    }
    res + 1
}

fn main() {
    println!("{}", josephus(5, 2));
}
