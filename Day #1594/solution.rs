// Approach: Pairwise O(n^2) overlap check on axis-aligned rectangles (strict, positive-area).
// Time O(n^2), Space O(1).

#[derive(Clone, Copy)]
struct Rect { x1: f64, y1: f64, x2: f64, y2: f64 }

// top_left (x,y), dims (w,h): x in [x,x+w], y in [y-h,y]
fn make(x: f64, y: f64, w: f64, h: f64) -> Rect { Rect { x1: x, y1: y - h, x2: x + w, y2: y } }

fn overlap(a: &Rect, b: &Rect) -> bool {
    a.x1 < b.x2 && b.x1 < a.x2 && a.y1 < b.y2 && b.y1 < a.y2
}

fn any_overlap(rs: &[Rect]) -> bool {
    for i in 0..rs.len() {
        for j in (i + 1)..rs.len() {
            if overlap(&rs[i], &rs[j]) { return true; }
        }
    }
    false
}

fn main() {
    let rs = [make(1.0, 4.0, 3.0, 3.0), make(-1.0, 3.0, 2.0, 1.0), make(0.0, 5.0, 4.0, 3.0)];
    println!("{}", if any_overlap(&rs) { "true" } else { "false" });
}
