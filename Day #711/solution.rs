// Day 711: Point strictly inside polygon. First reject boundary via on-segment
// test, then ray-casting parity test. Time O(N) per query.
type P = (f64, f64);

fn on_seg(a: P, b: P, p: P) -> bool {
    let cross = (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
    if cross.abs() > 1e-9 {
        return false;
    }
    a.0.min(b.0) - 1e-9 <= p.0 && p.0 <= a.0.max(b.0) + 1e-9 &&
        a.1.min(b.1) - 1e-9 <= p.1 && p.1 <= a.1.max(b.1) + 1e-9
}

fn inside(poly: &[P], p: P) -> bool {
    let n = poly.len();
    for i in 0..n {
        if on_seg(poly[i], poly[(i + 1) % n], p) {
            return false;
        }
    }
    let mut inq = false;
    let mut j = n - 1;
    for i in 0..n {
        if (poly[i].1 > p.1) != (poly[j].1 > p.1) {
            let xint = (poly[j].0 - poly[i].0) * (p.1 - poly[i].1)
                / (poly[j].1 - poly[i].1)
                + poly[i].0;
            if p.0 < xint {
                inq = !inq;
            }
        }
        j = i;
    }
    inq
}

fn b2s(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}

fn main() {
    let sq = vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)];
    println!("{}", b2s(inside(&sq, (2.0, 2.0))));
    println!("{}", b2s(inside(&sq, (4.0, 2.0))));
    println!("{}", b2s(inside(&sq, (5.0, 5.0))));
}
