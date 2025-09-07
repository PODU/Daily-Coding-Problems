// Egg drop: find min trials t such that maxFloors(t, eggs) >= k, where
// f(t,e) = f(t-1,e-1) + f(t-1,e) + 1 (floors distinguishable). Time: O(eggs * answer), Space: O(eggs).
fn egg_drop(eggs: usize, k: i64) -> i32 {
    let mut f = vec![0i64; eggs + 1];
    let mut t = 0;
    while f[eggs] < k {
        t += 1;
        for e in (1..=eggs).rev() {
            f[e] = f[e] + f[e - 1] + 1;
        }
    }
    t
}

fn main() {
    println!("{}", egg_drop(1, 5)); // 5
}
