// Fixed point (arr[i]==i) in sorted distinct array via binary search.
// Time O(log n), Space O(1).

// Returns Some(i) where arr[i]==i, or None if none.
fn fixed_point(arr: &[i64]) -> Option<usize> {
    let mut lo: i64 = 0;
    let mut hi: i64 = arr.len() as i64 - 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let v = arr[mid as usize];
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

fn run(arr: &[i64]) {
    match fixed_point(arr) {
        Some(i) => println!("{}", i),
        None => println!("False"),
    }
}

fn main() {
    run(&[-6, 0, 2, 40]);
    run(&[1, 5, 7, 8]);
}
