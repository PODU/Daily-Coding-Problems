// Day 1106: Count buildings (east->west) with a clear sunset view to the west.
// A building sees west if taller than all to its west; scan from west end, track max.
// Time: O(N) single pass. Space: O(1).
fn sunset_views(h: &[i32]) -> usize {
    let mut count = 0;
    let mut max_so_far = i32::MIN;
    for &x in h.iter().rev() {
        if x > max_so_far {
            count += 1;
            max_so_far = x;
        }
    }
    count
}

fn main() {
    println!("{}", sunset_views(&[3, 7, 8, 3, 6, 1])); // 3
}
