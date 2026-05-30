// Day 1581: Area of intersection of two axis-aligned rectangles.
// top_left=(x,y), dims=(w,h); x-range [x,x+w], y-range [y-h,y]. Overlap = clamped widths.
// Time: O(1); Space: O(1).

fn intersection_area(x1: i64, y1: i64, w1: i64, h1: i64, x2: i64, y2: i64, w2: i64, h2: i64) -> i64 {
    let ow = (x1 + w1).min(x2 + w2) - x1.max(x2);
    let oh = y1.min(y2) - (y1 - h1).max(y2 - h2);
    if ow <= 0 || oh <= 0 {
        0
    } else {
        ow * oh
    }
}

fn main() {
    println!("{}", intersection_area(1, 4, 3, 3, 0, 5, 4, 3)); // 6
}
