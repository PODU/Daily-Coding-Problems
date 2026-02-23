// Day 1119 - Split array into k contiguous partitions minimizing the max sum
// Binary search the answer; greedily count partitions. Time: O(n log(sum)).

fn partitions_needed(n: &[i64], limit: i64) -> i64 {
    let mut count = 1;
    let mut cur = 0i64;
    for &x in n {
        if cur + x > limit {
            count += 1;
            cur = x;
        } else {
            cur += x;
        }
    }
    count
}

fn split_min_max(n: &[i64], k: i64) -> i64 {
    let mut lo = *n.iter().max().unwrap();
    let mut hi: i64 = n.iter().sum();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if partitions_needed(n, mid) <= k {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main() {
    let n = vec![5i64, 1, 2, 7, 3, 4];
    println!("{}", split_min_max(&n, 3)); // 8
}
