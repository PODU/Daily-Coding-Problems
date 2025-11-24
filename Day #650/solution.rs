// Per-row binary search: count elements < A[i1][j1] (lower_bound) plus elements >= A[i2][j2] (M - lower_bound).
// Upper bound taken inclusive (>=) to match reference example. Time O(N log M), space O(1).
fn lower_bound(row: &[i32], key: i32) -> usize {
    let (mut lo, mut hi) = (0usize, row.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        if row[mid] < key { lo = mid + 1; } else { hi = mid; }
    }
    lo
}

fn main() {
    let a: Vec<Vec<i32>> = vec![
        vec![1, 3, 7, 10, 15, 20],
        vec![2, 6, 9, 14, 22, 25],
        vec![3, 8, 10, 15, 25, 30],
        vec![10, 11, 12, 23, 30, 35],
        vec![20, 25, 30, 35, 40, 45],
    ];
    let (i1, j1, i2, j2) = (1usize, 1usize, 3usize, 3usize);
    let pivot1 = a[i1][j1]; // 6
    let pivot2 = a[i2][j2]; // 23
    let m = a[0].len();
    let mut count_smaller = 0i64;
    let mut count_upper = 0i64;
    for row in &a {
        count_smaller += lower_bound(row, pivot1) as i64;        // strictly less than pivot1
        count_upper += (m - lower_bound(row, pivot2)) as i64;    // >= pivot2 (inclusive)
    }
    println!("{}", count_smaller + count_upper);
}
