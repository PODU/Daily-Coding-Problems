// Day 845: rotate a list left by k in place using the 3-reversal trick.
// reverse(0,k-1), reverse(k,n-1), reverse(0,n-1). O(n) time, O(1) extra space.
fn reverse(a: &mut [i32], mut i: usize, mut j: usize) {
    while i < j {
        a.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn rotate_left(a: &mut Vec<i32>, k: usize) {
    let n = a.len();
    if n == 0 {
        return;
    }
    let k = k % n;
    if k > 0 {
        reverse(a, 0, k - 1);
    }
    reverse(a, k, n - 1);
    reverse(a, 0, n - 1);
}

fn main() {
    let mut a = vec![1, 2, 3, 4, 5, 6];
    rotate_left(&mut a, 2);
    println!("{:?}", a); // [3, 4, 5, 6, 1, 2]
}
