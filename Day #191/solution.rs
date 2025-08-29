// Day 191: Min intervals to remove = n - max non-overlapping set (touching allowed).
// Greedy by earliest end. Time O(n log n), Space O(1).
fn min_removals(mut iv: Vec<(i64, i64)>) -> usize {
    iv.sort_by_key(|p| p.1);
    let mut kept = 0usize;
    let mut end = i64::MIN;
    for (s, e) in &iv {
        if *s >= end {
            kept += 1;
            end = *e;
        }
    }
    iv.len() - kept
}

fn main() {
    println!("{}", min_removals(vec![(7, 9), (2, 4), (5, 8)]));
}
