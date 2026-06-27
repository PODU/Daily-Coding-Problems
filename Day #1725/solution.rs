// Min intervals to remove for non-overlapping (touching allowed).
// Greedy: sort by end, keep intervals whose start >= last kept end. O(n log n) time, O(1) extra space.

fn min_removals(mut intervals: Vec<(i32, i32)>) -> usize {
    intervals.sort_by_key(|iv| iv.1);
    let mut kept = 0usize;
    let mut last_end = i32::MIN;
    for (start, end) in &intervals {
        if *start >= last_end {
            kept += 1;
            last_end = *end;
        }
    }
    intervals.len() - kept
}

fn main() {
    let intervals = vec![(7, 9), (2, 4), (5, 8)];
    println!("{}", min_removals(intervals));
}
