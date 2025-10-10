// Min meeting rooms: sort starts and ends, two-pointer sweep, half-open [start,end).
// Time O(n log n), Space O(n).

fn min_rooms(intervals: &[(i32, i32)]) -> i32 {
    let n = intervals.len();
    let mut starts: Vec<i32> = intervals.iter().map(|p| p.0).collect();
    let mut ends: Vec<i32> = intervals.iter().map(|p| p.1).collect();
    starts.sort();
    ends.sort();
    let (mut rooms, mut max_rooms, mut j) = (0i32, 0i32, 0usize);
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
    let intervals = [(30, 75), (0, 50), (60, 150)];
    println!("{}", min_rooms(&intervals));
}
