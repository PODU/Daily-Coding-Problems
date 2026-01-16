// Greedy: sort intervals by end; keep interval if start >= last kept end.
// Answer = total - kept. Time O(n log n), Space O(n).
fn erase_overlap_intervals(mut intervals: Vec<(i32, i32)>) -> i32 {
    intervals.sort_by_key(|&(_, e)| e);
    let mut kept = 0;
    let mut last_end = i32::MIN;
    for &(s, e) in &intervals {
        if s >= last_end {
            kept += 1;
            last_end = e;
        }
    }
    intervals.len() as i32 - kept
}

fn main() {
    let intervals = vec![(7, 9), (2, 4), (5, 8)];
    println!("{}", erase_overlap_intervals(intervals));
}
