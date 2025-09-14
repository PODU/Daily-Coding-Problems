// Day 273: Fixed point (arr[i]==i) in sorted distinct array via binary search.
// Time O(log N), Space O(1). Returns Some(index) or None (False).
fn fixed_point(a: &[i64]) -> Option<usize> {
    let mut lo: i64 = 0;
    let mut hi: i64 = a.len() as i64 - 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let v = a[mid as usize];
        if v == mid {
            return Some(mid as usize);
        } else if v < mid {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    None
}

fn show(a: &[i64]) {
    match fixed_point(a) {
        Some(i) => println!("{}", i),
        None => println!("False"),
    }
}

fn main() {
    show(&[-6, 0, 2, 40]); // 2
    show(&[1, 5, 7, 8]);   // False
}
