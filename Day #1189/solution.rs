// 3-sum existence: sort, fix each i, two-pointer scan remaining pair. Time O(N^2), Space O(1).

fn three_sum(arr: &[i64], k: i64) -> bool {
    let mut a = arr.to_vec();
    a.sort();
    let n = a.len();
    if n < 3 {
        return false;
    }
    for i in 0..n - 2 {
        let (mut lo, mut hi) = (i + 1, n - 1);
        while lo < hi {
            let s = a[i] + a[lo] + a[hi];
            if s == k {
                return true;
            } else if s < k {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
    }
    false
}

fn main() {
    let arr: Vec<i64> = vec![20, 303, 3, 4, 25];
    println!("{}", three_sum(&arr, 49));
}
