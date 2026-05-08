// Minimum meeting rooms for overlapping intervals.
// Sort starts and ends, two-pointer sweep. Time O(n log n), Space O(n).

fn min_rooms(intervals: &[(i32, i32)]) -> i32 {
    let n = intervals.len();
    let mut starts: Vec<i32> = intervals.iter().map(|iv| iv.0).collect();
    let mut ends: Vec<i32> = intervals.iter().map(|iv| iv.1).collect();
    starts.sort();
    ends.sort();
    let mut rooms = 0;
    let mut max_rooms = 0;
    let mut j = 0;
    for i in 0..n {
        while j < n && ends[j] <= starts[i] {
            rooms -= 1;
            j += 1;
        }
        rooms += 1;
        if rooms > max_rooms {
            max_rooms = rooms;
        }
    }
    max_rooms
}

fn main() {
    let intervals = vec![(30, 75), (0, 50), (60, 150)];
    println!("{}", min_rooms(&intervals)); // expected: 2
}
