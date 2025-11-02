// Egg drop (min worst-case trials): find smallest m such that with N eggs we can
// cover k floors. f(m, N) = sum_{i=1..N} C(m,i); smallest m with f(m,N) >= k.
// Time: O(m * N) where m is the answer; Space: O(1).

fn egg_drop(n: i64, k: i64) -> i32 {
    let mut m: i32 = 0;
    let mut covered: i64 = 0;
    while covered < k {
        m += 1;
        let mut sum: i64 = 0;
        let mut term: i64 = 1; // term = C(m, i)
        for i in 1..=n {
            term = term * (m as i64 - i + 1) / i; // C(m,i)
            sum += term;
            if sum >= k {
                break;
            }
        }
        covered = sum;
    }
    m
}

fn main() {
    println!("{}", egg_drop(1, 5)); // expected 5
}
