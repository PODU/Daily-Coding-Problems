// Rectangle intersection area via overlap of x and y ranges. Time O(1), Space O(1).
fn intersection_area(tlx1: i32, tly1: i32, w1: i32, h1: i32,
                     tlx2: i32, tly2: i32, w2: i32, h2: i32) -> i32 {
    let (left1, right1, top1, bottom1) = (tlx1, tlx1 + w1, tly1, tly1 - h1);
    let (left2, right2, top2, bottom2) = (tlx2, tlx2 + w2, tly2, tly2 - h2);
    let overlap_w = right1.min(right2) - left1.max(left2);
    let overlap_h = top1.min(top2) - bottom1.max(bottom2);
    overlap_w.max(0) * overlap_h.max(0)
}

fn main() {
    println!("{}", intersection_area(1, 4, 3, 3, 0, 5, 4, 3));
}
