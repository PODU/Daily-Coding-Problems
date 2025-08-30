// Day 194: Segments p_i->q_i cross iff order of p and q disagree. Count = inversions of q
// after sorting pairs by p. Merge-sort inversion count. Time O(n log n), Space O(n).
fn merge_count(v: &mut Vec<i64>) -> i64 {
    let n = v.len();
    if n <= 1 {
        return 0;
    }
    let m = n / 2;
    let mut left: Vec<i64> = v[..m].to_vec();
    let mut right: Vec<i64> = v[m..].to_vec();
    let mut cnt = merge_count(&mut left) + merge_count(&mut right);
    let (mut i, mut j, mut k) = (0usize, 0usize, 0usize);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            v[k] = left[i];
            i += 1;
        } else {
            v[k] = right[j];
            j += 1;
            cnt += (left.len() - i) as i64;
        }
        k += 1;
    }
    while i < left.len() {
        v[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        v[k] = right[j];
        j += 1;
        k += 1;
    }
    cnt
}

fn count_crossings(p: &[i64], q: &[i64]) -> i64 {
    let n = p.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| p[a].cmp(&p[b]));
    let mut qs: Vec<i64> = idx.iter().map(|&i| q[i]).collect();
    merge_count(&mut qs)
}

fn main() {
    println!("{}", count_crossings(&[1, 2, 3, 4], &[4, 3, 2, 1]));
}
