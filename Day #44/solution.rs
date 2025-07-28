// Count Inversions via modified merge sort: count cross-pairs while merging.
// Time O(n log n), Space O(n).

fn merge_count(a: &mut [i32], lo: usize, hi: usize, tmp: &mut [i32]) -> i64 {
    if hi - lo <= 1 {
        return 0;
    }
    let mid = (lo + hi) / 2;
    let mut inv = merge_count(a, lo, mid, tmp) + merge_count(a, mid, hi, tmp);
    let (mut i, mut j, mut k) = (lo, mid, lo);
    while i < mid && j < hi {
        if a[i] <= a[j] {
            tmp[k] = a[i];
            i += 1;
        } else {
            tmp[k] = a[j];
            j += 1;
            inv += (mid - i) as i64;
        }
        k += 1;
    }
    while i < mid {
        tmp[k] = a[i];
        i += 1;
        k += 1;
    }
    while j < hi {
        tmp[k] = a[j];
        j += 1;
        k += 1;
    }
    a[lo..hi].copy_from_slice(&tmp[lo..hi]);
    inv
}

fn count_inversions(src: &[i32]) -> i64 {
    let mut a = src.to_vec();
    let n = a.len();
    let mut tmp = vec![0; n];
    merge_count(&mut a, 0, n, &mut tmp)
}

fn main() {
    println!("{}", count_inversions(&[2, 4, 1, 3, 5]));
    println!("{}", count_inversions(&[5, 4, 3, 2, 1]));
}
