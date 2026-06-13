// Rotate singly linked list right by k. Use index rotation on a Vec. O(n) time, O(n) space (demo).
fn rotate_right(xs: &[i32], k: usize) -> Vec<i32> {
    let n = xs.len();
    if n == 0 { return vec![]; }
    let steps = n - (k % n);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(xs[(steps + i) % n]);
    }
    out
}

fn show(xs: &[i32]) {
    let parts: Vec<String> = xs.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" -> "));
}

fn main() {
    show(&rotate_right(&[7, 7, 3, 5], 2));
    show(&rotate_right(&[1, 2, 3, 4, 5], 3));
}
