// Point in polygon: ray-casting (even-odd rule). Boundary points are detected separately
// and return false. Time: O(N), Space: O(1).
fn on_segment(px: f64, py: f64, ax: f64, ay: f64, bx: f64, by: f64) -> bool {
    let cross = (bx - ax) * (py - ay) - (by - ay) * (px - ax);
    if cross.abs() > 1e-9 {
        return false;
    }
    ax.min(bx) - 1e-9 <= px && px <= ax.max(bx) + 1e-9 &&
        ay.min(by) - 1e-9 <= py && py <= ay.max(by) + 1e-9
}

fn inside(poly: &[(f64, f64)], px: f64, py: f64) -> bool {
    let n = poly.len();
    let mut res = false;
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = poly[i];
        let (xj, yj) = poly[j];
        if on_segment(px, py, xi, yi, xj, yj) {
            return false; // boundary
        }
        if (yi > py) != (yj > py) && px < (xj - xi) * (py - yi) / (yj - yi) + xi {
            res = !res;
        }
        j = i;
    }
    res
}

fn main() {
    let poly = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)];
    println!("{}", inside(&poly, 2.0, 2.0)); // true
    println!("{}", inside(&poly, 4.0, 2.0)); // false (boundary)
    println!("{}", inside(&poly, 5.0, 5.0)); // false (outside)
}
