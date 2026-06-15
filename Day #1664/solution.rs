// Apply permutation: result[P[i]] = array[i]. O(n) time, O(n) space.
fn main() {
    let arr = ["a", "b", "c"];
    let p = [2usize, 1, 0];
    let mut res = vec![""; arr.len()];
    for i in 0..arr.len() {
        res[p[i]] = arr[i];
    }
    println!("[{}]", res.join(", "));
}
