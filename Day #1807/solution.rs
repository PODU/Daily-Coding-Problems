// Split array into k contiguous partitions minimizing the max partition sum.
// Binary search on the answer + greedy feasibility. Time: O(n log(sum)). Space: O(1).
fn feasible(a: &[i64], k: usize, cap: i64) -> bool {
    let mut cur = 0i64;
    let mut parts = 1usize;
    for &x in a {
        if x > cap {
            return false;
        }
        if cur + x > cap {
            parts += 1;
            cur = x;
        } else {
            cur += x;
        }
    }
    parts <= k
}

fn split_array(a: &[i64], k: usize) -> i64 {
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
    let n = [5i64, 1, 2, 7, 3, 4];
    println!("{}", split_array(&n, 3)); // 8
}
