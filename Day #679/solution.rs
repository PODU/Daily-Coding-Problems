// Count inversions via modified merge sort. Time O(N log N), Space O(N).

fn sort_count(a: &[i64]) -> (Vec<i64>, u64) {
    if a.len() <= 1 {
        return (a.to_vec(), 0);
    }
    let mid = a.len() / 2;
    let (left, il) = sort_count(&a[..mid]);
    let (right, ir) = sort_count(&a[mid..]);
    let mut merged = Vec::with_capacity(a.len());
    let (mut i, mut j, mut inv) = (0usize, 0usize, 0u64);
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
    (merged, il + ir + inv)
}

fn count_inversions(arr: &[i64]) -> u64 {
    sort_count(arr).1
}

fn main() {
    println!("{}", count_inversions(&[2, 4, 1, 3, 5]));
    println!("{}", count_inversions(&[5, 4, 3, 2, 1]));
}
