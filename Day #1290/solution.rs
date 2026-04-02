// Day 1290: Strict point-in-polygon test (ray casting), boundary counts as outside.
// Check edges for on-boundary, then parity of rightward ray crossings. Time O(n), Space O(1).
type Pt = (f64, f64);

fn on_segment(a: Pt, b: Pt, p: Pt) -> bool {
    let cross = (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
    if cross.abs() > 1e-9 {
        return false;
    }
    p.0 >= a.0.min(b.0) - 1e-9 && p.0 <= a.0.max(b.0) + 1e-9 &&
        p.1 >= a.1.min(b.1) - 1e-9 && p.1 <= a.1.max(b.1) + 1e-9
}

fn inside(poly: &[Pt], p: Pt) -> bool {
    let n = poly.len();
    for i in 0..n {
        if on_segment(poly[i], poly[(i + 1) % n], p) {
            return false; // boundary
        }
    }
    let mut res = false;
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = poly[i];
        let (xj, yj) = poly[j];
        if (yi > p.1) != (yj > p.1) {
            let xint = (xj - xi) * (p.1 - yi) / (yj - yi) + xi;
            if p.0 < xint {
                res = !res;
            }
        }
        j = i;
    }
    res
}

fn main() {
    let square = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)];
    println!("{}", inside(&square, (2.0, 2.0))); // true
    println!("{}", inside(&square, (5.0, 5.0))); // false
    println!("{}", inside(&square, (4.0, 2.0))); // false (boundary)
}
