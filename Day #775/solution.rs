// Day 775: Minimum meeting rooms = max overlapping intervals.
// Sort starts and ends, two-pointer sweep. O(n log n) time, O(n) space.
fn min_rooms(intervals: &[(i32, i32)]) -> i32 {
    let n = intervals.len();
    let mut starts: Vec<i32> = intervals.iter().map(|x| x.0).collect();
    let mut ends: Vec<i32> = intervals.iter().map(|x| x.1).collect();
    starts.sort();
    ends.sort();
    let (mut rooms, mut best, mut i, mut j) = (0, 0, 0, 0);
    while i < n {
        if starts[i] < ends[j] {
            rooms += 1;
            i += 1;
            best = best.max(rooms);
        } else {
            rooms -= 1;
            j += 1;
        }
    }
    best
}

fn main() {
    println!("{}", min_rooms(&[(30, 75), (0, 50), (60, 150)])); // 2
}
