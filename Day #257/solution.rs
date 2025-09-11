// Day 257: Smallest window that must be sorted to make the whole array sorted.
// Left->right track max to find right bound; right->left track min to find left bound.
// Time: O(n), Space: O(1).

fn sort_window(a: &[i32]) -> (i32, i32) {
    let n = a.len();
    let mut begin: i32 = -1;
    let mut end: i32 = -1;
    let mut mx = i32::MIN;
    for i in 0..n {
        if a[i] < mx {
            end = i as i32;
        } else {
            mx = a[i];
        }
    }
    let mut mn = i32::MAX;
    for i in (0..n).rev() {
        if a[i] > mn {
            begin = i as i32;
        } else {
            mn = a[i];
        }
    }
    (begin, end)
}

fn main() {
    let a = [3, 7, 5, 6, 9];
    let (b, e) = sort_window(&a);
    println!("({}, {})", b, e); // (1, 3)
}
