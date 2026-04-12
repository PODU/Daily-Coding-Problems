// Find smallest window to sort: right bound = last i where a[i] < running max; left bound = first j where a[j] > running min from right.
// Time: O(n), Space: O(1).
fn find_unsorted(a: &[i32]) -> (i32, i32) {
    let n = a.len();
    let mut end: i32 = -1;
    let mut mx = i32::MIN;
    for i in 0..n {
        if a[i] < mx {
            end = i as i32;
        } else {
            mx = a[i];
        }
    }
    let mut start: i32 = -1;
    let mut mn = i32::MAX;
    for i in (0..n).rev() {
        if a[i] > mn {
            start = i as i32;
        } else {
            mn = a[i];
        }
    }
    (start, end)
}

fn main() {
    let a = [3, 7, 5, 6, 9];
    let (start, end) = find_unsorted(&a);
    println!("({}, {})", start, end);
}
