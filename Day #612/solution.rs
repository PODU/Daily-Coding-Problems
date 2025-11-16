// Day 612: Min intervals to remove so the rest are non-overlapping (touching allowed).
// Approach: sort by end, greedily keep max non-overlapping; answer = total - kept. Time O(n log n).

fn min_removals(intervals: &[(i32, i32)]) -> usize {
    let mut v: Vec<(i32, i32)> = intervals.to_vec();
    v.sort_by_key(|iv| iv.1);
    let mut kept = 0usize;
    let mut end = i32::MIN;
    for &(s, e) in &v {
        if s >= end {
            kept += 1;
            end = e;
        }
    }
    v.len() - kept
}

fn main() {
    let intervals = [(7, 9), (2, 4), (5, 8)];
    println!("{}", min_removals(&intervals)); // 1
}
