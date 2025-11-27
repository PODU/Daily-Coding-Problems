// Day 661: Search sorted array without mul/div/bit-shift.
// Fibonacci search uses only +/- to pick probe points. Time O(log N), Space O(1).
fn fib_search(a: &[i32], x: i32) -> i32 {
    let n = a.len() as i32;
    let (mut f2, mut f1) = (0i32, 1i32);
    let mut f = f1 + f2;
    while f < n {
        f2 = f1;
        f1 = f;
        f = f1 + f2;
    }
    let mut offset = -1i32;
    while f > 1 {
        let i = std::cmp::min(offset + f2, n - 1);
        let v = a[i as usize];
        if v < x {
            f = f1;
            f1 = f2;
            f2 = f - f1;
            offset = i;
        } else if v > x {
            f = f2;
            f1 = f1 - f2;
            f2 = f - f1;
        } else {
            return i;
        }
    }
    if f1 != 0 && offset + 1 < n && a[(offset + 1) as usize] == x {
        return offset + 1;
    }
    -1
}

fn main() {
    let a = vec![-1, 0, 3, 5, 9, 12];
    println!("Index of 5: {}", fib_search(&a, 5)); // 3
    println!("Index of 7: {}", fib_search(&a, 7)); // -1 (absent)
}
