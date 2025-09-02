// Day 206: Apply permutation P to array (result[P[i]] = a[i]).
// In-place via cycle following on the permutation. Time: O(n), Space: O(1).
fn apply_permutation<T>(a: &mut [T], perm: &[usize]) {
    let mut p = perm.to_vec();
    for i in 0..a.len() {
        while p[i] != i {
            let j = p[i];
            a.swap(i, j);
            p[i] = p[j];
            p[j] = j;
        }
    }
}

fn main() {
    let mut a = vec!["a", "b", "c"];
    apply_permutation(&mut a, &[2, 1, 0]);
    println!("{:?}", a); // ["c", "b", "a"]
}
