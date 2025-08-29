// Day 185: Area of intersection of two axis-aligned rectangles (top-left + width/height, y up).
// Overlap = max(0, dx) * max(0, dy). Time O(1), Space O(1).
struct Rect { left: i64, top: i64, w: i64, h: i64 }

fn intersection_area(a: &Rect, b: &Rect) -> i64 {
    let ox = (a.left + a.w).min(b.left + b.w) - a.left.max(b.left);
    let oy = a.top.min(b.top) - (a.top - a.h).max(b.top - b.h);
    if ox <= 0 || oy <= 0 {
        return 0;
    }
    ox * oy
}

fn main() {
    let a = Rect { left: 1, top: 4, w: 3, h: 3 };
    let b = Rect { left: 0, top: 5, w: 4, h: 3 };
    println!("{}", intersection_area(&a, &b));
}
