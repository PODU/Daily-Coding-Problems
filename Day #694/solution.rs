// Day 694: First missing positive integer in O(n) time, O(1) space.
// Approach: cyclic sort - place each value v in [1,n] at index v-1, then scan.
// Time O(n), Space O(1) (in-place).
fn first_missing_positive(a: &mut Vec<i32>) -> i32 {
    let n = a.len() as i32;
    for i in 0..a.len() {
        while a[i] > 0 && a[i] <= n && a[(a[i] - 1) as usize] != a[i] {
            let j = (a[i] - 1) as usize;
            a.swap(i, j);
        }
    }
    for i in 0..a.len() {
        if a[i] != i as i32 + 1 {
            return i as i32 + 1;
        }
    }
    n + 1
}

fn main() {
    println!("{}", first_missing_positive(&mut vec![3, 4, -1, 1])); // 2
    println!("{}", first_missing_positive(&mut vec![1, 2, 0]));     // 3
}
