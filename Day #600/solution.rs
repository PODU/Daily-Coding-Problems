// Day 600: Closest pair of points on a plane.
// Approach: divide & conquer with a y-sorted strip check. Time O(n log n), Space O(n).

type Pt = (i64, i64);

fn dist2(a: Pt, b: Pt) -> f64 {
    let dx = (a.0 - b.0) as f64;
    let dy = (a.1 - b.1) as f64;
    dx * dx + dy * dy
}

fn rec(px: &[Pt], lo: usize, hi: usize) -> (f64, Pt, Pt) {
    let n = hi - lo;
    let mut best = f64::INFINITY;
    let (mut ba, mut bb) = ((0, 0), (0, 0));
    if n <= 3 {
        for i in lo..hi {
            for j in (i + 1)..hi {
                let d = dist2(px[i], px[j]);
                if d < best {
                    best = d;
                    ba = px[i];
                    bb = px[j];
                }
            }
        }
        return (best, ba, bb);
    }
    let mid = (lo + hi) / 2;
    let midx = px[mid].0;
    let (bl, la, lb) = rec(px, lo, mid);
    let (br, ra, rb) = rec(px, mid, hi);
    if bl <= br {
        best = bl; ba = la; bb = lb;
    } else {
        best = br; ba = ra; bb = rb;
    }
    let mut dd = best.sqrt();
    let mut strip: Vec<Pt> = Vec::new();
    for i in lo..hi {
        if ((px[i].0 - midx) as f64).abs() < dd {
            strip.push(px[i]);
        }
    }
    strip.sort_by_key(|p| p.1);
    for i in 0..strip.len() {
        let mut j = i + 1;
        while j < strip.len() && ((strip[j].1 - strip[i].1) as f64) < dd {
            let d = dist2(strip[i], strip[j]);
            if d < best {
                best = d;
                ba = strip[i];
                bb = strip[j];
                dd = best.sqrt();
            }
            j += 1;
        }
    }
    (best, ba, bb)
}

fn main() {
    let mut pts: Vec<Pt> = vec![(1, 1), (-1, -1), (3, 4), (6, 1), (-1, -6), (-4, -3)];
    pts.sort();
    let (_, mut a, mut b) = rec(&pts, 0, pts.len());
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    println!("[({}, {}), ({}, {})]", a.0, a.1, b.0, b.1);
}
