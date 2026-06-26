// Day 1718: Count intersecting segment pairs (p_i on y=0 -> q_i on y=1).
// Two segments cross iff their (p, q) ordering is inverted: sort by p,
// count inversions in q via merge sort. Time: O(n log n), Space: O(n).

fn merge_count(a: &mut [i32], l: usize, r: usize) -> u64 {
    if r - l <= 1 {
        return 0;
    }
    let mid = (l + r) / 2;
    let mut inv = merge_count(a, l, mid) + merge_count(a, mid, r);
    let mut tmp = Vec::with_capacity(r - l);
    let (mut i, mut j) = (l, mid);
    while i < mid && j < r {
        if a[i] <= a[j] {
            tmp.push(a[i]);
            i += 1;
        } else {
            inv += (mid - i) as u64;
            tmp.push(a[j]);
            j += 1;
        }
    }
    while i < mid {
        tmp.push(a[i]);
        i += 1;
    }
    while j < r {
        tmp.push(a[j]);
        j += 1;
    }
    a[l..r].copy_from_slice(&tmp);
    inv
}

fn count_intersections(p: &[i32], q: &[i32]) -> u64 {
    let n = p.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&x, &y| p[x].cmp(&p[y]));
    let mut qs: Vec<i32> = idx.iter().map(|&i| q[i]).collect();
    merge_count(&mut qs, 0, n)
}

fn main() {
    let p = [1, 2, 3, 4];
    let q = [2, 1, 4, 3];
    println!("Intersecting pairs: {}", count_intersections(&p, &q));
}
