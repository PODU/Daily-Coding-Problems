// Min broadcast range: sort towers, binary-search nearest per listener, answer = max of mins.
// Time O((N+M) log M), Space O(1).
fn min_range(listeners: &[i64], towers: &[i64]) -> i64 {
    let mut t = towers.to_vec();
    t.sort();
    let mut ans = 0i64;
    for &l in listeners {
        let i = t.partition_point(|&x| x < l);
        let mut best = i64::MAX;
        if i < t.len() {
            best = best.min(t[i] - l);
        }
        if i > 0 {
            best = best.min(l - t[i - 1]);
        }
        ans = ans.max(best);
    }
    ans
}

fn main() {
    println!("{}", min_range(&[1, 5, 11, 20], &[4, 8, 15]));
}
