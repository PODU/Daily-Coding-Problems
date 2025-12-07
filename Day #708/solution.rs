// Day 708: Fixed point (a[i]==i) in a sorted distinct array via binary search.
// Since values are distinct integers, a[i]-i is monotonic. Time O(log n).
fn fixed_point(a: &[i32]) -> Option<usize> {
    let mut lo = 0i64;
    let mut hi = a.len() as i64 - 1;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let v = a[mid as usize] as i64;
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

fn report(a: &[i32]) {
    match fixed_point(a) {
        Some(i) => println!("{}", i),
        None => println!("False"),
    }
}

fn main() {
    report(&[-6, 0, 2, 40]);
    report(&[1, 5, 7, 8]);
}
