// Day 187: Do any two rectangles overlap (containment counts; edge-touching does not).
// Pairwise interior-overlap test. Time O(n^2), Space O(1). (Sweep line gives O(n log n).)
struct Rect { left: i64, top: i64, w: i64, h: i64 }

fn overlap(a: &Rect, b: &Rect) -> bool {
    let ox = (a.left + a.w).min(b.left + b.w) - a.left.max(b.left);
    let oy = a.top.min(b.top) - (a.top - a.h).max(b.top - b.h);
    ox > 0 && oy > 0
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
        Rect { left: 1, top: 4, w: 3, h: 3 },
        Rect { left: -1, top: 3, w: 2, h: 1 },
        Rect { left: 0, top: 5, w: 4, h: 3 },
    ];
    println!("{}", any_overlap(&rs));
}
