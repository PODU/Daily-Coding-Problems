// Day 271: Search sorted list without *, /, or bit-shift -> Fibonacci search.
// Uses only addition/subtraction/comparison. Time O(log N), Space O(1).
fn fib_search(arr: &[i32], x: i32) -> i32 {
    let n = arr.len() as i32;
    let (mut fib_mm2, mut fib_mm1) = (0i32, 1i32);
    let mut fib_m = fib_mm2 + fib_mm1;
    while fib_m < n {
        fib_mm2 = fib_mm1;
        fib_mm1 = fib_m;
        fib_m = fib_mm2 + fib_mm1;
    }
    let mut offset = -1i32;
    while fib_m > 1 {
        let i = std::cmp::min(offset + fib_mm2, n - 1);
        let v = arr[i as usize];
        if v < x {
            fib_m = fib_mm1;
            fib_mm1 = fib_mm2;
            fib_mm2 = fib_m - fib_mm1;
            offset = i;
        } else if v > x {
            fib_m = fib_mm2;
            fib_mm1 = fib_mm1 - fib_mm2;
            fib_mm2 = fib_m - fib_mm1;
        } else {
            return i;
        }
    }
    if fib_mm1 != 0 && offset + 1 < n && arr[(offset + 1) as usize] == x {
        return offset + 1;
    }
    -1
}

fn main() {
    let arr = [1, 3, 4, 7, 9, 11, 15];
    println!("7 -> index {}", fib_search(&arr, 7)); // 3
    println!("8 -> index {}", fib_search(&arr, 8)); // -1
}
