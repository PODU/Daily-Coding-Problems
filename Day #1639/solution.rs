// Count inversions using modified merge sort (count cross-pairs during merge).
// Time: O(N log N), Space: O(N).

fn merge_count(a: &mut [i32], tmp: &mut [i32], lo: usize, hi: usize) -> u64 {
    if hi - lo <= 1 {
        return 0;
    }
    let mid = lo + (hi - lo) / 2;
    let mut inv = merge_count(a, tmp, lo, mid) + merge_count(a, tmp, mid, hi);
    let (mut i, mut j, mut k) = (lo, mid, lo);
    while i < mid && j < hi {
        if a[i] <= a[j] {
            tmp[k] = a[i];
            i += 1;
        } else {
            tmp[k] = a[j];
            j += 1;
            inv += (mid - i) as u64;
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

fn count_inversions(src: &[i32]) -> u64 {
    let mut a = src.to_vec();
    let mut tmp = vec![0; a.len()];
    let n = a.len();
    merge_count(&mut a, &mut tmp, 0, n)
}

fn main() {
    println!("[2, 4, 1, 3, 5] has {} inversions", count_inversions(&[2, 4, 1, 3, 5]));
    println!("[5, 4, 3, 2, 1] has {} inversions", count_inversions(&[5, 4, 3, 2, 1]));
}
