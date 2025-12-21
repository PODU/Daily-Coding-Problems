// Day 773: Count inversions via modified merge sort. O(n log n) time, O(n) space.
fn merge_count(a: &mut [i32], l: usize, r: usize, tmp: &mut [i32]) -> u64 {
    if r - l <= 1 {
        return 0;
    }
    let m = (l + r) / 2;
    let mut cnt = merge_count(a, l, m, tmp) + merge_count(a, m, r, tmp);
    let (mut i, mut j, mut k) = (l, m, l);
    while i < m && j < r {
        if a[i] <= a[j] {
            tmp[k] = a[i];
            i += 1;
        } else {
            tmp[k] = a[j];
            j += 1;
            cnt += (m - i) as u64;
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
    a[l..r].copy_from_slice(&tmp[l..r]);
    cnt
}

fn count_inversions(arr: &[i32]) -> u64 {
    let mut a = arr.to_vec();
    let mut tmp = vec![0; a.len()];
    let n = a.len();
    merge_count(&mut a, 0, n, &mut tmp)
}

fn main() {
    println!("{}", count_inversions(&[2, 4, 1, 3, 5])); // 3
    println!("{}", count_inversions(&[5, 4, 3, 2, 1])); // 10
}
