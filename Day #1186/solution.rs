// Count intersecting segment pairs: sort segments by p, then count inversions in q.
// Two segments cross iff their p-order and q-order disagree. O(n log n) time, O(n) space.

fn merge_count(a: Vec<i64>) -> (Vec<i64>, i64) {
    if a.len() <= 1 {
        return (a, 0);
    }
    let m = a.len() / 2;
    let (left, cl) = merge_count(a[..m].to_vec());
    let (right, cr) = merge_count(a[m..].to_vec());
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0usize, 0usize);
    let mut c = cl + cr;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
            c += (left.len() - i) as i64;
        }
    }
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);
    (merged, c)
}

fn count_intersections(p: &[i64], q: &[i64]) -> i64 {
    let mut order: Vec<usize> = (0..p.len()).collect();
    order.sort_by(|&x, &y| p[x].cmp(&p[y]));
    let qq: Vec<i64> = order.iter().map(|&k| q[k]).collect();
    merge_count(qq).1
}

fn main() {
    let p = [1, 2, 3];
    let q = [3, 1, 2];
    println!("{}", count_intersections(&p, &q)); // 2
}
