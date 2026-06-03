// 2xN tiling with dominoes + L-trominoes (LeetCode 790). DP recurrence
// f(n)=2*f(n-1)+f(n-3). O(N) time, O(1) space. Mod 1e9+7 for large N.
const MOD: i64 = 1_000_000_007;

fn num_tilings(n: i32) -> i64 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    let (mut a, mut b, mut c): (i64, i64, i64) = (1, 1, 2); // f(0),f(1),f(2)
    for _ in 3..=n {
        let f = (2 * c + a) % MOD;
        a = b;
        b = c;
        c = f;
    }
    c
}

fn main() {
    println!("N=4 -> {}", num_tilings(4)); // 11
    let table: Vec<String> = (1..=5).map(|n| num_tilings(n).to_string()).collect();
    println!("Table N=1..5: {}", table.join(" ")); // 1 2 5 11 24
}
