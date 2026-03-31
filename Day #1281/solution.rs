// Day 1281: Area of intersection of two axis-aligned rectangles.
// Overlap on each axis = min(rights)-max(lefts), clamped at 0. Time O(1), Space O(1).
// Each rectangle: top-left (x, y) with width w and height h (y grows upward).
fn intersect_area(x1: i32, y1: i32, w1: i32, h1: i32, x2: i32, y2: i32, w2: i32, h2: i32) -> i64 {
    let ow = (x1 + w1).min(x2 + w2) - x1.max(x2);
    let oh = y1.min(y2) - (y1 - h1).max(y2 - h2);
    if ow <= 0 || oh <= 0 {
        return 0;
    }
    ow as i64 * oh as i64
}

fn main() {
    println!("{}", intersect_area(1, 4, 3, 3, 0, 5, 4, 3)); // 6
}
