// First missing positive: place each value v in slot v-1 via swaps, then scan.
// Time: O(n), Space: O(1).
fn first_missing_positive(a: &mut [i32]) -> i32 {
    let n = a.len();
    for i in 0..n {
        while a[i] > 0 && (a[i] as usize) <= n && a[a[i] as usize - 1] != a[i] {
            let j = a[i] as usize - 1;
            a.swap(i, j);
        }
    }
    for i in 0..n {
        if a[i] != (i as i32) + 1 {
            return (i as i32) + 1;
        }
    }
    n as i32 + 1
}

fn main() {
    let mut a = [3, 4, -1, 1];
    println!("{}", first_missing_positive(&mut a));
}
