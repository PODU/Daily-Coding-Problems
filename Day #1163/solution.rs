// Greedy: sort intervals by end ascending; keep if start >= last kept end; count removals. O(n log n) time, O(1) extra space.
fn erase_overlap_intervals(mut intervals: Vec<(i64, i64)>) -> i32 {
    intervals.sort_by_key(|iv| iv.1);
    let mut removals = 0;
    let mut last_end = i64::MIN;
    for (start, end) in intervals {
        if start >= last_end {
            last_end = end;
        } else {
            removals += 1;
        }
    }
    removals
}

fn main() {
    let intervals = vec![(7, 9), (2, 4), (5, 8)];
    println!("{}", erase_overlap_intervals(intervals));
}
