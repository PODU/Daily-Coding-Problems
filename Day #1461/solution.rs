// Egg drop: DP on trials. f(t,e)=f(t-1,e-1)+f(t-1,e)+1 = max floors with t trials, e eggs.
// Smallest t with f(t,N)>=k. Time O(N*answer), Space O(N).

// Minimum worst-case trials for N eggs and k floors.
fn egg_drop(n: usize, k: u64) -> u64 {
    if k == 0 {
        return 0;
    }
    let mut f = vec![0u64; n + 1];
    let mut t: u64 = 0;
    while f[n] < k {
        t += 1;
        for e in (1..=n).rev() {
            f[e] = f[e] + f[e - 1] + 1;
        }
    }
    t
}

fn main() {
    println!("{}", egg_drop(1, 5));
}
