// Sort, then fix arr[i] and two-pointer scan for the remaining pair summing to k.
// Time O(n^2), Space O(1) extra.
fn three_sum(mut arr: Vec<i32>, k: i32) -> bool {
    arr.sort();
    let n = arr.len();
    for i in 0..n.saturating_sub(2) {
        let (mut lo, mut hi) = (i + 1, n - 1);
        while lo < hi {
            let s = arr[i] + arr[lo] + arr[hi];
            if s == k {
                return true;
            }
            if s < k {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
    }
    false
}

fn main() {
    let arr = vec![20, 303, 3, 4, 25];
    println!("{}", if three_sum(arr, 49) { "true" } else { "false" });
}
