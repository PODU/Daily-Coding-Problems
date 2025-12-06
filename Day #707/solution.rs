// Day 707: Min broadcast range. Sort towers; for each listener binary-search the
// nearest tower, answer is max of those min distances. Time O((N+M)logM).
fn min_range(listeners: &[i32], towers: &[i32]) -> i32 {
    let mut t = towers.to_vec();
    t.sort();
    let mut ans = 0;
    for &x in listeners {
        let i = t.partition_point(|&v| v < x);
        let mut best = i32::MAX;
        if i < t.len() {
            best = best.min(t[i] - x);
        }
        if i > 0 {
            best = best.min(x - t[i - 1]);
        }
        ans = ans.max(best);
    }
    ans
}

fn main() {
    println!("{}", min_range(&[1, 5, 11, 20], &[4, 8, 15]));
}
