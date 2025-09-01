// Day 197: Rotate array right by k in-place.
// Triple-reversal: reverse whole, reverse first k, reverse rest. O(n) time, O(1) space.
fn reverse(a: &mut [i32], mut i: usize, mut j: usize) {
    while i < j {
        a.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn rotate_right(a: &mut Vec<i32>, k: usize) {
    let n = a.len();
    if n == 0 {
        return;
    }
    let k = k % n;
    reverse(a, 0, n - 1);
    if k > 0 {
        reverse(a, 0, k - 1);
    }
    reverse(a, k, n - 1);
}

fn main() {
    let mut a = vec![1, 2, 3, 4, 5];
    rotate_right(&mut a, 2);
    println!("{:?}", a); // [4, 5, 1, 2, 3]
}
