// Day 1329: Closest pair of points via divide & conquer. O(n log n) time.
// Sort by x, recurse on halves, then check the middle strip ordered by y (each point vs next ~7).

type P = (i64, i64);

fn dist(a: P, b: P) -> f64 {
    let dx = (a.0 - b.0) as f64;
    let dy = (a.1 - b.1) as f64;
    (dx * dx + dy * dy).sqrt()
}

fn rec(px: &[P], py: &[P]) -> (f64, P, P) {
    let n = px.len();
    if n <= 3 {
        let mut best = (f64::INFINITY, (0, 0), (0, 0));
        for i in 0..n {
            for j in i + 1..n {
                let d = dist(px[i], px[j]);
                if d < best.0 {
                    best = (d, px[i], px[j]);
                }
            }
        }
        return best;
    }
    let mid = n / 2;
    let pivot = px[mid];
    let mid_x = pivot.0;
    let (lx, rx) = px.split_at(mid);
    let mut ly = Vec::new();
    let mut ry = Vec::new();
    for &p in py {
        if p < pivot {
            ly.push(p);
        } else {
            ry.push(p);
        }
    }
    let mut best = rec(lx, &ly);
    let r = rec(rx, &ry);
    if r.0 < best.0 {
        best = r;
    }
    let strip: Vec<P> = py.iter().cloned().filter(|p| ((p.0 - mid_x) as f64).abs() < best.0).collect();
    for i in 0..strip.len() {
        let mut j = i + 1;
        while j < strip.len() && ((strip[j].1 - strip[i].1) as f64) < best.0 {
            let d = dist(strip[i], strip[j]);
            if d < best.0 {
                best = (d, strip[i], strip[j]);
            }
            j += 1;
        }
    }
    best
}

fn main() {
    let pts: Vec<P> = vec![(1, 1), (-1, -1), (3, 4), (6, 1), (-1, -6), (-4, -3)];
    let mut px = pts.clone();
    let mut py = pts.clone();
    px.sort();
    py.sort_by_key(|p| p.1);
    let (_, mut a, mut b) = rec(&px, &py);
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    println!("[({}, {}), ({}, {})]", a.0, a.1, b.0, b.1); // [(-1, -1), (1, 1)]
}
