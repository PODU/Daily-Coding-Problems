// Day 796: Point strictly inside a polygon.
// Ray-casting (even-odd rule) + on-boundary check. Time O(N), Space O(1).

type Pt = (f64, f64);

fn on_segment(a: Pt, b: Pt, p: Pt) -> bool {
    let cross = (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
    if cross.abs() > 1e-9 {
        return false;
    }
    a.0.min(b.0) - 1e-9 <= p.0 && p.0 <= a.0.max(b.0) + 1e-9
        && a.1.min(b.1) - 1e-9 <= p.1 && p.1 <= a.1.max(b.1) + 1e-9
}

fn inside_polygon(poly: &[Pt], p: Pt) -> bool {
    let n = poly.len();
    for i in 0..n {
        if on_segment(poly[i], poly[(i + 1) % n], p) {
            return false;
        }
    }
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        if (poly[i].1 > p.1) != (poly[j].1 > p.1)
            && p.0 < (poly[j].0 - poly[i].0) * (p.1 - poly[i].1)
                / (poly[j].1 - poly[i].1) + poly[i].0
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn main() {
    let square = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)];
    println!("{}", inside_polygon(&square, (2.0, 2.0))); // true
    println!("{}", inside_polygon(&square, (4.0, 2.0))); // false (boundary)
    println!("{}", inside_polygon(&square, (5.0, 5.0))); // false
}
