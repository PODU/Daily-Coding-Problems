// Trapping rain water via two pointers tracking leftMax/rightMax. Time O(N), Space O(1).
fn trap(h: &[i32]) -> i32 {
    let (mut l, mut r) = (0usize, h.len() - 1);
    let (mut lm, mut rm, mut water) = (0, 0, 0);
    while l < r {
        if h[l] < h[r] {
            lm = lm.max(h[l]);
            water += lm - h[l];
            l += 1;
        } else {
            rm = rm.max(h[r]);
            water += rm - h[r];
            r -= 1;
        }
    }
    water
}

fn main() {
    println!("[2, 1, 2] -> {}", trap(&[2, 1, 2]));
    println!("[3, 0, 1, 3, 0, 5] -> {}", trap(&[3, 0, 1, 3, 0, 5]));
}
