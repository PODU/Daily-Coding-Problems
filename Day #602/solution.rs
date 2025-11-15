// Approach: sort segments by p, then count inversions in the q-order via merge sort.
// Two segments cross iff their p-order and q-order disagree => an inversion. Time O(n log n), Space O(n).

fn merge_count(a: &[i64]) -> (Vec<i64>, u64) {
    if a.len() <= 1 {
        return (a.to_vec(), 0);
    }
    let m = a.len() / 2;
    let (left, il) = merge_count(&a[..m]);
    let (right, ir) = merge_count(&a[m..]);
    let mut merged = Vec::with_capacity(a.len());
    let mut inv = il + ir;
    let (mut i, mut j) = (0usize, 0usize);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
            inv += (left.len() - i) as u64;
        }
    }
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);
    (merged, inv)
}

fn count_intersections(p: &[i64], q: &[i64]) -> u64 {
    let mut order: Vec<usize> = (0..p.len()).collect();
    order.sort_by(|&x, &y| p[x].cmp(&p[y]));
    let qs: Vec<i64> = order.iter().map(|&k| q[k]).collect();
    merge_count(&qs).1
}

fn main() {
    let p = vec![1, 2, 3, 4];
    let q = vec![4, 3, 2, 1];
    println!("{}", count_intersections(&p, &q));
}
