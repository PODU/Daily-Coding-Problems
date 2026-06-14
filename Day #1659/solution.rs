// Smallest unsorted window. Scan L->R tracking max for end, R->L tracking min for start. O(n) time, O(1) space.
fn unsorted_window(a: &[i32]) -> (i32, i32) {
    let n = a.len();
    let (mut end, mut start) = (-1i32, -1i32);
    let (mut mx, mut mn) = (i32::MIN, i32::MAX);
    for i in 0..n {
        mx = mx.max(a[i]);
        if a[i] < mx { end = i as i32; }
    }
    for i in (0..n).rev() {
        mn = mn.min(a[i]);
        if a[i] > mn { start = i as i32; }
    }
    (start, end)
}

fn main() {
    let (s, e) = unsorted_window(&[3, 7, 5, 6, 9]);
    println!("({}, {})", s, e);
}
