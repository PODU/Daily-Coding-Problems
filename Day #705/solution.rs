// Day 705: Trapping rain water.
// Approach: two pointers tracking running left/right maxima; the smaller side is
// bounded so we can resolve it. Time O(N), Space O(1).
fn trap(h: &[i32]) -> i32 {
    let (mut l, mut r) = (0usize, h.len().saturating_sub(1));
    let (mut lmax, mut rmax, mut water) = (0, 0, 0);
    while l < r {
        if h[l] < h[r] {
            lmax = lmax.max(h[l]);
            water += lmax - h[l];
            l += 1;
        } else {
            rmax = rmax.max(h[r]);
            water += rmax - h[r];
            r -= 1;
        }
    }
    water
}

fn main() {
    println!("{}", trap(&[2, 1, 2]));          // 1
    println!("{}", trap(&[3, 0, 1, 3, 0, 5])); // 8
}
