// Greedy activity selection: sort by end time, pick job if start >= last end (intervals [start,end)).
// Time O(n log n), Space O(n).
fn select_jobs(jobs: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut sorted = jobs.to_vec();
    sorted.sort_by_key(|j| j.1);
    let mut chosen = Vec::new();
    let mut last_end = i64::MIN;
    for &(start, end) in &sorted {
        if start >= last_end {
            chosen.push((start, end));
            last_end = end;
        }
    }
    chosen
}

fn main() {
    let jobs = [
        (0, 6), (1, 4), (3, 5), (3, 8), (4, 7), (5, 9), (6, 10), (8, 11),
    ];
    let chosen = select_jobs(&jobs);
    let parts: Vec<String> = chosen.iter().map(|&(s, e)| format!("({}, {})", s, e)).collect();
    println!("[{}]", parts.join(", "));
}
