// Apply permutation where result[P[i]] = array[i] (scatter). In-place cycle-following via swaps:
// O(n) time, O(1) extra space. Also a simple O(n)-space scatter is provided.

fn apply_in_place(a: &mut Vec<String>, p_in: &[usize]) {
    let mut p = p_in.to_vec(); // local copy so caller's permutation is untouched
    for i in 0..a.len() {
        while p[i] != i {
            let j = p[i];
            a.swap(i, j);
            p.swap(i, j);
        }
    }
}

#[allow(dead_code)]
fn apply_simple(a: &[String], p: &[usize]) -> Vec<String> {
    let mut res = vec![String::new(); a.len()];
    for i in 0..a.len() {
        res[p[i]] = a[i].clone();
    }
    res
}

fn main() {
    let mut a: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
    let p = vec![2usize, 1, 0]; // result[P[i]] = a[i]
    apply_in_place(&mut a, &p);
    println!("[{}]", a.join(", "));
}
