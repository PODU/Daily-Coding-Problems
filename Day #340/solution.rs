// Closest pair of points via divide & conquer. O(n log n) time, O(n) space.
// Sort by x, recurse on halves, merge by checking a y-sorted strip near the split.
use std::collections::HashMap;

type Pt = (i64, i64);

fn dist(a: Pt, b: Pt) -> f64 {
    let dx = (a.0 - b.0) as f64;
    let dy = (a.1 - b.1) as f64;
    (dx * dx + dy * dy).sqrt()
}

fn closest(px: &[Pt], py: &[Pt]) -> (f64, Pt, Pt) {
    let n = px.len();
    if n <= 3 {
        let mut best = f64::MAX;
        let mut ba = px[0];
        let mut bb = px[0];
        for i in 0..n {
            for j in (i + 1)..n {
                let d = dist(px[i], px[j]);
                if d < best {
                    best = d;
                    ba = px[i];
                    bb = px[j];
                }
            }
        }
        return (best, ba, bb);
    }
    let mid = n / 2;
    let midx = px[mid].0;
    let lx = &px[..mid];
    let rx = &px[mid..];
    let mut left_count: HashMap<Pt, i32> = HashMap::new();
    for &p in lx {
        *left_count.entry(p).or_insert(0) += 1;
    }
    let mut ly: Vec<Pt> = Vec::new();
    let mut ry: Vec<Pt> = Vec::new();
    for &p in py {
        let c = left_count.entry(p).or_insert(0);
        if *c > 0 {
            ly.push(p);
            *c -= 1;
        } else {
            ry.push(p);
        }
    }
    let (dl, la, lb) = closest(lx, &ly);
    let (dr, ra, rb) = closest(rx, &ry);
    let (mut d, mut ba, mut bb) = if dl < dr { (dl, la, lb) } else { (dr, ra, rb) };
    let strip: Vec<Pt> = py.iter().cloned().filter(|p| ((p.0 - midx) as f64).abs() < d).collect();
    for i in 0..strip.len() {
        let mut j = i + 1;
        while j < strip.len() && ((strip[j].1 - strip[i].1) as f64) < d {
            let dd = dist(strip[i], strip[j]);
            if dd < d {
                d = dd;
                ba = strip[i];
                bb = strip[j];
            }
            j += 1;
        }
    }
    (d, ba, bb)
}

fn main() {
    let pts: Vec<Pt> = vec![(1, 1), (-1, -1), (3, 4), (6, 1), (-1, -6), (-4, -3)];
    let mut px = pts.clone();
    let mut py = pts.clone();
    px.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    py.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    let (_, mut a, mut b) = closest(&px, &py);
    if a.0 > b.0 || (a.0 == b.0 && a.1 > b.1) {
        std::mem::swap(&mut a, &mut b);
    }
    println!("[({}, {}), ({}, {})]", a.0, a.1, b.0, b.1);
}
