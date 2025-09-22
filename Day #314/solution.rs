// Day 314: Min broadcast range = max over listeners of distance to nearest tower.
// Sort towers, binary search each listener. O((N+M) log M).
fn min_range(listeners: &[i32], towers: &[i32]) -> i32 {
    let mut t = towers.to_vec();
    t.sort();
    let mut ans = 0;
    for &l in listeners {
        let idx = t.partition_point(|&x| x < l);
        let mut best = i32::MAX;
        if idx < t.len() { best = best.min(t[idx] - l); }
        if idx > 0 { best = best.min(l - t[idx - 1]); }
        ans = ans.max(best);
    }
    ans
}

fn main() {
    println!("{}", min_range(&[1, 5, 11, 20], &[4, 8, 15])); // 5
}
