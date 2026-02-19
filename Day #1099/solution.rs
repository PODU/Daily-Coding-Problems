// Day 1099: Rotate array right by k in-place using the reversal algorithm.
// Reverse whole, reverse first k, reverse rest. Time: O(N). Space: O(1).
fn rotate(a: &mut [i32], k: usize) {
    let n = a.len();
    if n == 0 {
        return;
    }
    let k = k % n;
    a.reverse();
    a[..k].reverse();
    a[k..].reverse();
}

fn main() {
    let mut a = [1, 2, 3, 4, 5, 6, 7];
    rotate(&mut a, 3);
    println!("{:?}", a); // [5, 6, 7, 1, 2, 3, 4]
}
