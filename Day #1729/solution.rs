// Day 1729: Count buildings with a sunset (west) view.
// Single right-to-left pass tracking max height seen to the west; a building is
// visible iff strictly taller than all to its west. Time: O(n). Space: O(1).

fn count_sunset_views(heights: &[i32]) -> usize {
    let mut count = 0;
    let mut max_west = 0;
    // Scan from the west end (rightmost index) toward the east.
    for &h in heights.iter().rev() {
        if h > max_west {
            count += 1;
            max_west = h;
        }
    }
    count
}

fn main() {
    let heights = [3, 7, 8, 3, 6, 1]; // east -> west
    println!("{}", count_sunset_views(&heights)); // 1, 6, 8 visible => 3
}
