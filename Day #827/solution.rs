// Day 827: Min broadcast range.
// Sort towers; for each listener binary-search nearest tower, take min distance;
// answer = max over listeners. Time O((N+M) log M), Space O(1).
fn min_broadcast_range(listeners: &[i64], towers: &[i64]) -> i64 {
    let mut t = towers.to_vec();
    t.sort();
    let mut ans = 0i64;
    for &l in listeners {
        let mut best = i64::MAX;
        let i = t.partition_point(|&x| x < l);
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
    println!("{}", min_broadcast_range(&[1, 5, 11, 20], &[4, 8, 15]));
}
