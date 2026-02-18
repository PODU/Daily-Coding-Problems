// Detect if any pair of axis-aligned rectangles overlap (containment counts). Time O(n^2), Space O(n).
#[derive(Clone, Copy)]
struct Rect {
    minx: f64,
    maxx: f64,
    miny: f64,
    maxy: f64,
}

fn from_top_left(x: f64, y: f64, w: f64, h: f64) -> Rect {
    // top_left corner; width grows right, height grows down
    Rect { minx: x, maxx: x + w, miny: y - h, maxy: y }
}

fn overlap(a: &Rect, b: &Rect) -> bool {
    a.minx < b.maxx && b.minx < a.maxx && a.miny < b.maxy && b.miny < a.maxy
}

fn any_overlap(rs: &[Rect]) -> bool {
    for i in 0..rs.len() {
        for j in (i + 1)..rs.len() {
            if overlap(&rs[i], &rs[j]) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let rs = vec![
        from_top_left(1.0, 4.0, 3.0, 3.0),
        from_top_left(-1.0, 3.0, 2.0, 1.0),
        from_top_left(0.0, 5.0, 4.0, 3.0),
    ];
    println!("{}", any_overlap(&rs));
}
