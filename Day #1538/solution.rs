// Minimum rooms for overlapping intervals: sort starts & ends, sweep with two pointers.
// Time O(n log n), Space O(n).
fn min_rooms(intervals: &[(i32, i32)]) -> i32 {
    let n = intervals.len();
    let mut starts: Vec<i32> = intervals.iter().map(|x| x.0).collect();
    let mut ends: Vec<i32> = intervals.iter().map(|x| x.1).collect();
    starts.sort();
    ends.sort();
    let (mut rooms, mut max, mut j) = (0, 0, 0);
    for i in 0..n {
        while j < n && ends[j] <= starts[i] {
            rooms -= 1;
            j += 1;
        }
        rooms += 1;
        if rooms > max {
            max = rooms;
        }
    }
    max
}

fn main() {
    println!("{}", min_rooms(&[(30, 75), (0, 50), (60, 150)]));
}
