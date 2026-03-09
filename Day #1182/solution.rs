// Day 1182: Split N into k contiguous partitions minimizing the maximum sum.
// Binary search the answer in [max element, total]; greedy feasibility check.
// Time O(N log(sum)), Space O(1).

fn feasible(a: &[i64], k: i64, cap: i64) -> bool {
    let mut cur = 0i64;
    let mut parts = 1i64;
    for &x in a {
        if cur + x > cap {
            parts += 1;
            cur = x;
            if parts > k {
                return false;
            }
        } else {
            cur += x;
        }
    }
    true
}

fn split_array(a: &[i64], k: i64) -> i64 {
    let mut lo = *a.iter().max().unwrap();
    let mut hi: i64 = a.iter().sum();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if feasible(a, k, mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main() {
    println!("{}", split_array(&[5, 1, 2, 7, 3, 4], 3)); // 8
}
