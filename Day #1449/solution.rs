// Day 1449: Trapping Rain Water. Two-pointer sweep tracking running left/right
// maxima. Time O(n), Space O(1).
fn trap(h: &[i32]) -> i64 {
    if h.is_empty() {
        return 0;
    }
    let (mut l, mut r) = (0usize, h.len() - 1);
    let (mut left_max, mut right_max) = (0i32, 0i32);
    let mut water = 0i64;
    while l < r {
        if h[l] < h[r] {
            left_max = left_max.max(h[l]);
            water += (left_max - h[l]) as i64;
            l += 1;
        } else {
            right_max = right_max.max(h[r]);
            water += (right_max - h[r]) as i64;
            r -= 1;
        }
    }
    water
}

fn main() {
    println!("{}", trap(&[2, 1, 2]));          // 1
    println!("{}", trap(&[3, 0, 1, 3, 0, 5])); // 8
}
