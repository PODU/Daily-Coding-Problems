// Smallest window to sort so the whole array is sorted.
// Two passes: left->right track max for right bound; right->left track min for left bound.
// Time: O(n), Space: O(1).

fn window_to_sort(a: &[i32]) -> (i32, i32) {
    let n = a.len();
    let mut left: i32 = -1;
    let mut right: i32 = -1;
    let mut max_so_far = a[0];
    for i in 1..n {
        if a[i] < max_so_far {
            right = i as i32;
        } else {
            max_so_far = a[i];
        }
    }
    let mut min_so_far = a[n - 1];
    for i in (0..n - 1).rev() {
        if a[i] > min_so_far {
            left = i as i32;
        } else {
            min_so_far = a[i];
        }
    }
    (left, right)
}

fn main() {
    let a = [3, 7, 5, 6, 9];
    let (left, right) = window_to_sort(&a);
    println!("({}, {})", left, right);
}
