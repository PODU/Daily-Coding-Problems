// Day 1680: Strict point-in-polygon. Reject boundary via on-segment test, else
// ray-casting parity. Time O(N), Space O(1).
fn on_seg(x1: f64, y1: f64, x2: f64, y2: f64, px: f64, py: f64) -> bool {
    let cross = (x2 - x1) * (py - y1) - (y2 - y1) * (px - x1);
    if cross.abs() > 1e-9 {
        return false;
    }
    px >= x1.min(x2) - 1e-9 && px <= x1.max(x2) + 1e-9 &&
        py >= y1.min(y2) - 1e-9 && py <= y1.max(y2) + 1e-9
}

fn inside(poly: &[(f64, f64)], px: f64, py: f64) -> bool {
    let n = poly.len();
    for i in 0..n {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % n];
        if on_seg(x1, y1, x2, y2, px, py) {
            return false;
        }
    }
    let mut res = false;
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = poly[i];
        let (xj, yj) = poly[j];
        if (yi > py) != (yj > py) && px < (xj - xi) * (py - yi) / (yj - yi) + xi {
            res = !res;
        }
        j = i;
    }
    res
}

fn main() {
    let sq = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)];
    println!("{}", inside(&sq, 2.0, 2.0)); // true
    println!("{}", inside(&sq, 4.0, 2.0)); // false
    println!("{}", inside(&sq, 5.0, 5.0)); // false
}
