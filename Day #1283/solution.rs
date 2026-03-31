// Day 1283: For each element, count smaller elements to its right.
// Fenwick (BIT) over compressed values, scanning right-to-left. Time O(n log n), Space O(n).
fn count_smaller(a: &[i32]) -> Vec<i32> {
    let n = a.len();
    let mut sorted: Vec<i32> = a.to_vec();
    sorted.sort();
    sorted.dedup();
    let mut tree = vec![0i32; sorted.len() + 1];
    let mut res = vec![0i32; n];
    for i in (0..n).rev() {
        // rank = lower_bound index + 1
        let rank = sorted.partition_point(|&x| x < a[i]) + 1;
        // query prefix sum up to rank-1
        let mut s = 0;
        let mut j = rank - 1;
        while j > 0 {
            s += tree[j];
            j -= j & j.wrapping_neg();
        }
        res[i] = s;
        // update at rank
        let mut k = rank;
        while k < tree.len() {
            tree[k] += 1;
            k += k & k.wrapping_neg();
        }
    }
    res
}

fn main() {
    println!("{:?}", count_smaller(&[3, 4, 9, 6, 1])); // [1, 1, 2, 1, 0]
}
