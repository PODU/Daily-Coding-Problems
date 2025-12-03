// Custom reduce/fold (left to right). O(n) time, O(1) extra space.

fn reduce<F: Fn(i32, i32) -> i32>(arr: &[i32], fn_: F, init: i32) -> i32 {
    let mut acc = init;
    for &x in arr {
        acc = fn_(acc, x);
    }
    acc
}

fn main() {
    let arr = [1, 2, 3, 4];
    println!("{}", reduce(&arr, |a, b| a + b, 0)); // 10
    println!("{}", reduce(&arr, |a, b| a * b, 1)); // 24 (bonus)
}
