// Day 1331: Does any pair of rectangles overlap (full containment counts; edge-touching does not)?
// Convert top_left+dimensions to [xmin,xmax,ymin,ymax]; pairwise strict-interval overlap test. O(n^2).

#[derive(Clone, Copy)]
struct Rect { xmin: i64, xmax: i64, ymin: i64, ymax: i64 }

fn make(tlx: i64, tly: i64, w: i64, h: i64) -> Rect {
    Rect { xmin: tlx, xmax: tlx + w, ymin: tly - h, ymax: tly }
}

fn overlap(a: &Rect, b: &Rect) -> bool {
    a.xmin < b.xmax && b.xmin < a.xmax && a.ymin < b.ymax && b.ymin < a.ymax
}

fn main() {
    let rs = [
        make(1, 4, 3, 3),
        make(-1, 3, 2, 1),
        make(0, 5, 4, 3),
    ];
    let mut any = false;
    'outer: for i in 0..rs.len() {
        for j in i + 1..rs.len() {
            if overlap(&rs[i], &rs[j]) {
                any = true;
                break 'outer;
            }
        }
    }
    println!("{}", any); // true
}
