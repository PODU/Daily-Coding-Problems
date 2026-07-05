// Trapping Rain Water: two-pointer sweep tracking left/right running maxima.
// Time: O(N), Space: O(1).
fn trap(h: &[i64]) -> i64 {
    let (mut l, mut r) = (0usize, h.len() - 1);
    let (mut left_max, mut right_max) = (0i64, 0i64);
    let mut water = 0i64;
    while l < r {
        if h[l] < h[r] {
            left_max = left_max.max(h[l]);
            water += left_max - h[l];
            l += 1;
        } else {
            right_max = right_max.max(h[r]);
            water += right_max - h[r];
            r -= 1;
        }
    }
    water
}

fn main() {
    println!("{}", trap(&[2, 1, 2]));
    println!("{}", trap(&[3, 0, 1, 3, 0, 5]));
}
