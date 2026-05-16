// First missing positive: cyclic sort (index-as-hash), place each x in [1,n] at index x-1.
// Time O(n), Space O(1) extra (in-place).
fn first_missing_positive(arr: &[i32]) -> i32 {
    let mut a = arr.to_vec();
    let n = a.len();
    for i in 0..n {
        while a[i] >= 1 && a[i] <= n as i32 && a[(a[i] - 1) as usize] != a[i] {
            let j = (a[i] - 1) as usize;
            a.swap(i, j);
        }
    }
    for i in 0..n {
        if a[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }
    (n + 1) as i32
}

fn main() {
    println!("{}", first_missing_positive(&[3, 4, -1, 1])); // 2
    println!("{}", first_missing_positive(&[1, 2, 0]));     // 3
}
