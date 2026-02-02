// Count intersecting segment pairs: sort by p, count inversions in q via merge sort.
// Time O(n log n), Space O(n).

fn merge_count(a: &mut Vec<i64>, tmp: &mut Vec<i64>, l: usize, r: usize) -> i64 {
    if r - l <= 1 {
        return 0;
    }
    let m = (l + r) / 2;
    let mut cnt = merge_count(a, tmp, l, m) + merge_count(a, tmp, m, r);
    let (mut i, mut j, mut k) = (l, m, l);
    while i < m && j < r {
        if a[i] <= a[j] {
            tmp[k] = a[i];
            i += 1;
        } else {
            tmp[k] = a[j];
            j += 1;
            cnt += (m - i) as i64;
        }
        k += 1;
    }
    while i < m {
        tmp[k] = a[i];
        i += 1;
        k += 1;
    }
    while j < r {
        tmp[k] = a[j];
        j += 1;
        k += 1;
    }
    for t in l..r {
        a[t] = tmp[t];
    }
    cnt
}

fn count_intersections(p: &[i64], q: &[i64]) -> i64 {
    let n = p.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&x, &y| p[x].cmp(&p[y]));
    let mut qs: Vec<i64> = idx.iter().map(|&i| q[i]).collect();
    let mut tmp = vec![0i64; n];
    merge_count(&mut qs, &mut tmp, 0, n)
}

fn main() {
    let p = vec![1, 2, 3, 4];
    let q = vec![4, 3, 2, 1];
    println!("{}", count_intersections(&p, &q)); // 6
}
