// Meeting rooms: sort starts & ends, sweep with two pointers tracking overlap.
// Time O(n log n), Space O(n).
fn min_rooms(intervals: &[(i32, i32)]) -> i32 {
    let n = intervals.len();
    let mut starts: Vec<i32> = intervals.iter().map(|x| x.0).collect();
    let mut ends: Vec<i32> = intervals.iter().map(|x| x.1).collect();
    starts.sort();
    ends.sort();
    let (mut rooms, mut max_rooms, mut j) = (0, 0, 0usize);
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
    println!("{}", min_rooms(&intervals));
}
