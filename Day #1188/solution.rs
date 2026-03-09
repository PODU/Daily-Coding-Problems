// Count inversions via merge sort: count cross-pairs while merging. Time O(N log N), Space O(N).

fn sort_count(a: &[i64]) -> (Vec<i64>, u64) {
    let n = a.len();
    if n <= 1 {
        return (a.to_vec(), 0);
    }
    let mid = n / 2;
    let (left, l_inv) = sort_count(&a[..mid]);
    let (right, r_inv) = sort_count(&a[mid..]);
    let mut merged = Vec::with_capacity(n);
    let (mut i, mut j) = (0usize, 0usize);
    let mut inv = l_inv + r_inv;
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

fn main() {
    let arr: Vec<i64> = vec![2, 4, 1, 3, 5];
    let (_, inv) = sort_count(&arr);
    println!("{}", inv);
}
