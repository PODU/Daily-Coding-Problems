// Day 1453: Apply a permutation P (P[i] = destination of element i) to an array
// in place by following cycles. Time O(n), Space O(1) extra (P is consumed).
fn apply_permutation<T>(arr: &mut [T], p: &mut [usize]) {
    for i in 0..arr.len() {
        while p[i] != i {
            let pi = p[i];
            arr.swap(i, pi);
            p.swap(i, pi);
        }
    }
}

fn main() {
    let mut arr = vec!["a", "b", "c"];
    let mut p = vec![2usize, 1, 0];
    apply_permutation(&mut arr, &mut p);
    let quoted: Vec<String> = arr.iter().map(|x| format!("\"{}\"", x)).collect();
    println!("[{}]", quoted.join(", ")); // ["c", "b", "a"]
}
