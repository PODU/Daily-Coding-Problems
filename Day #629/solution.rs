// Split array into k contiguous parts minimizing the max partition sum.
// Binary search on answer in [max, total]; greedy feasibility check. O(n log(sum)).

fn feasible(nums: &[i64], k: i64, cap: i64) -> bool {
    let mut parts = 1;
    let mut cur = 0i64;
    for &x in nums {
        if cur + x > cap {
            parts += 1;
            cur = x;
        } else {
            cur += x;
        }
    }
    parts <= k
}

fn split_array(nums: &[i64], k: i64) -> i64 {
    let mut lo = *nums.iter().max().unwrap();
    let mut hi: i64 = nums.iter().sum();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if feasible(nums, k, mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main() {
    let nums = [5i64, 1, 2, 7, 3, 4];
    println!("{}", split_array(&nums, 3)); // expected 8
}
