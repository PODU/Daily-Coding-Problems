// Closest pair of points via divide-and-conquer on x, strip-merge on y.
// Each point tagged with an id for an unambiguous left/right split. Time: O(n log n). Space: O(n).
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Pt {
    x: i64,
    y: i64,
    id: usize,
}

fn dist2(a: &Pt, b: &Pt) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    dx * dx + dy * dy
}

fn rec(px: &[Pt], py: &[Pt]) -> (Pt, Pt) {
    let n = px.len();
    if n <= 3 {
        let mut best = f64::INFINITY;
        let mut bp = (px[0], px[0]);
        for i in 0..n {
            for j in i + 1..n {
                let d = dist2(&px[i], &px[j]);
                if d < best {
                    best = d;
                    bp = (px[i], px[j]);
                }
            }
        }
        return bp;
    }
    let mid = n / 2;
    let mid_x = px[mid].x;
    let lx = &px[..mid];
    let rx = &px[mid..];
    let left_ids: HashSet<usize> = lx.iter().map(|p| p.id).collect();
    let ly: Vec<Pt> = py.iter().filter(|p| left_ids.contains(&p.id)).cloned().collect();
    let ry: Vec<Pt> = py.iter().filter(|p| !left_ids.contains(&p.id)).cloned().collect();

    let bl = rec(lx, &ly);
    let br = rec(rx, &ry);
    let mut best = if dist2(&bl.0, &bl.1) < dist2(&br.0, &br.1) { bl } else { br };
    let mut d2 = dist2(&best.0, &best.1);

    let strip: Vec<Pt> = py
        .iter()
        .filter(|p| {
            let dx = (p.x - mid_x) as f64;
            dx * dx < d2
        })
        .cloned()
        .collect();
    for i in 0..strip.len() {
        let mut j = i + 1;
        while j < strip.len() {
            let dy = (strip[j].y - strip[i].y) as f64;
            if dy * dy >= d2 {
                break;
            }
            let d = dist2(&strip[i], &strip[j]);
            if d < d2 {
                d2 = d;
                best = (strip[i], strip[j]);
            }
            j += 1;
        }
    }
    best
}

fn closest_pair(points: &[(i64, i64)]) -> ((i64, i64), (i64, i64)) {
    let pts: Vec<Pt> = points.iter().enumerate().map(|(i, &(x, y))| Pt { x, y, id: i }).collect();
    let mut px = pts.clone();
    let mut py = pts.clone();
    px.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
    py.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    let (a, b) = rec(&px, &py);
    let mut pa = (a.x, a.y);
    let mut pb = (b.x, b.y);
    if pa > pb {
        std::mem::swap(&mut pa, &mut pb);
    }
    (pa, pb)
}

fn main() {
    let pts = [(1, 1), (-1, -1), (3, 4), (6, 1), (-1, -6), (-4, -3)];
    let (a, b) = closest_pair(&pts);
    println!("[({}, {}), ({}, {})]", a.0, a.1, b.0, b.1);
    // [(-1, -1), (1, 1)]
}
