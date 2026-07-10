// Min broadcast range: sort towers, binary-search nearest tower per listener, take max.
// Time O((N+M) log M), Space O(1) extra.
fn main() {
    let listeners = [1i32, 5, 11, 20];
    let mut towers = vec![4i32, 8, 15];
    towers.sort();
    let mut ans = 0;
    for &l in &listeners {
        let i = towers.partition_point(|&t| t < l);
        let mut best = i32::MAX;
        if i < towers.len() {
            best = best.min(towers[i] - l);
        }
        if i > 0 {
            best = best.min(l - towers[i - 1]);
        }
        ans = ans.max(best);
    }
    println!("{}", ans);
}
