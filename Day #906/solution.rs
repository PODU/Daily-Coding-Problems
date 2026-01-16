// Closest pair of points via divide & conquer: sort by x, recurse, merge with strip check by y.
// O(n log n) time, O(n) space.
type Pt = (i64, i64);

fn dist(a: Pt, b: Pt) -> f64 {
    let dx = (a.0 - b.0) as f64;
    let dy = (a.1 - b.1) as f64;
    (dx * dx + dy * dy).sqrt()
}

struct Best {
    d: f64,
    a: Pt,
    b: Pt,
}

fn consider(best: &mut Best, a: Pt, b: Pt) {
    let d = dist(a, b);
    if d < best.d {
        best.d = d;
        best.a = a;
        best.b = b;
    }
}

// px sorted by x in [lo,hi); returns vec sorted by y
fn rec(px: &[Pt], lo: usize, hi: usize, best: &mut Best) -> Vec<Pt> {
    let n = hi - lo;
    if n <= 3 {
        let mut pts: Vec<Pt> = px[lo..hi].to_vec();
        for i in 0..pts.len() {
            for j in (i + 1)..pts.len() {
                consider(best, pts[i], pts[j]);
            }
        }
        pts.sort_by_key(|p| p.1);
        return pts;
    }
    let mid = lo + n / 2;
    let midx = px[mid].0;
    let left = rec(px, lo, mid, best);
    let right = rec(px, mid, hi, best);
    let mut merged: Vec<Pt> = Vec::with_capacity(n);
    let (mut i, mut j) = (0usize, 0usize);
    while i < left.len() && j < right.len() {
        if left[i].1 <= right[j].1 {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);
    let strip: Vec<Pt> = merged
        .iter()
        .cloned()
        .filter(|p| ((p.0 - midx).abs() as f64) < best.d)
        .collect();
    for a in 0..strip.len() {
        let mut b = a + 1;
        while b < strip.len() && ((strip[b].1 - strip[a].1) as f64) < best.d {
            consider(best, strip[a], strip[b]);
            b += 1;
        }
    }
    merged
}

fn main() {
    let mut pts: Vec<Pt> = vec![(1, 1), (-1, -1), (3, 4), (6, 1), (-1, -6), (-4, -3)];
    pts.sort();
    let mut best = Best {
        d: f64::MAX,
        a: (0, 0),
        b: (0, 0),
    };
    rec(&pts, 0, pts.len(), &mut best);
    let (mut a, mut b) = (best.a, best.b);
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    println!("[({}, {}), ({}, {})]", a.0, a.1, b.0, b.1);
}
