// Rotate array right by k via three reversals: reverse all, reverse first k, reverse rest.
// Time: O(n), Space: O(1).

fn rotate(a: &mut Vec<i32>, k: usize) {
    let n = a.len();
    if n == 0 {
        return;
    }
    let k = k % n;
    a.reverse();
    a[..k].reverse();
    a[k..].reverse();
}

fn main() {
    let mut a = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut a, 3);
    let parts: Vec<String> = a.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
