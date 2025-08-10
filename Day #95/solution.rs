// Day 95: Next lexicographic permutation in place. Find rightmost ascent, swap
// with next-larger suffix element, reverse suffix. O(n) time, O(1) space.
fn next_permutation(a: &mut [i32]) {
    let n = a.len();
    if n < 2 {
        return;
    }
    let mut i = n - 1;
    while i > 0 && a[i - 1] >= a[i] {
        i -= 1;
    }
    if i > 0 {
        let pivot = i - 1;
        let mut j = n - 1;
        while a[j] <= a[pivot] {
            j -= 1;
        }
        a.swap(pivot, j);
    }
    a[i..].reverse();
}

fn main() {
    for test in [[1, 2, 3], [1, 3, 2], [3, 2, 1]] {
        let mut a = test;
        next_permutation(&mut a);
        println!("{:?}", a);
    }
    // [1, 3, 2] / [2, 1, 3] / [1, 2, 3]
}
