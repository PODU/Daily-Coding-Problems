// Rectangle intersection area: O(1) time, O(1) space.
// top_left is top y, height extends downward. x_overlap*y_overlap clamped at 0.
struct Rect { left: f64, top: f64, width: f64, height: f64 }

fn intersect_area(a: &Rect, b: &Rect) -> f64 {
    let (a_right, b_right) = (a.left + a.width, b.left + b.width);
    let (a_bottom, b_bottom) = (a.top - a.height, b.top - b.height);
    let xo = (a_right.min(b_right) - a.left.max(b.left)).max(0.0);
    let yo = (a.top.min(b.top) - a_bottom.max(b_bottom)).max(0.0);
    xo * yo
}

fn main() {
    let r1 = Rect { left: 1.0, top: 4.0, width: 3.0, height: 3.0 };
    let r2 = Rect { left: 0.0, top: 5.0, width: 4.0, height: 3.0 };
    println!("{}", intersect_area(&r1, &r2));
}
