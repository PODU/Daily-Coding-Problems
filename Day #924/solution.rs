// Smallest window to sort: scan left->right tracking max for right bound,
// right->left tracking min for left bound. Time O(n), Space O(1).
fn find_unsorted_window(a: &[i32]) -> (i32, i32) {
    let n = a.len();
    let mut right: i32 = -1;
    let mut run_max = a[0];
    for i in 1..n {
        if a[i] < run_max {
            right = i as i32;
        } else {
            run_max = a[i];
        }
    }
    let mut left: i32 = -1;
    let mut run_min = a[n - 1];
    for i in (0..n - 1).rev() {
        if a[i] > run_min {
            left = i as i32;
        } else {
            run_min = a[i];
        }
    }
    (left, right)
}

fn main() {
    let arr = [3, 7, 5, 6, 9];
    let (l, r) = find_unsorted_window(&arr);
    println!("({}, {})", l, r);
}
