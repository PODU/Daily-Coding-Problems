// Trapping rain water with two pointers.
// Time: O(n), Space: O(1).
fn trap(h: &[i32]) -> i32 {
    let mut l = 0usize;
    let mut r = h.len() - 1;
    let (mut left_max, mut right_max, mut water) = (0, 0, 0);
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
    let heights = [3, 0, 1, 3, 0, 5];
    println!("{}", trap(&heights));
}
