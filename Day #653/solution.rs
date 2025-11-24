// Brute-force all O(k^2) pairs; rectangles overlap iff their projections strictly overlap on both axes.
// Sweep-line O(k log k) is possible but k^2 is clear. Time O(k^2), space O(k).
#[derive(Clone, Copy)]
struct Rect { x1: i32, y1: i32, x2: i32, y2: i32 }

// top_left (x,y), dims (w,h): x1=x, x2=x+w, y2=y(top), y1=y-h(bottom)
fn make(x: i32, y: i32, w: i32, h: i32) -> Rect {
    Rect { x1: x, y1: y - h, x2: x + w, y2: y }
}

fn overlap(a: &Rect, b: &Rect) -> bool {
    a.x1 < b.x2 && b.x1 < a.x2 && a.y1 < b.y2 && b.y1 < a.y2
}

fn main() {
    let rects = vec![
        make(1, 4, 3, 3),   // R1
        make(-1, 3, 2, 1),  // R2
        make(0, 5, 4, 3),   // R3
    ];
    let mut any = false;
    'outer: for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            if overlap(&rects[i], &rects[j]) { any = true; break 'outer; }
        }
    }
    println!("{}", if any { "true" } else { "false" });
}
