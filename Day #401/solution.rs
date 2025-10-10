// Apply permutation P: result[P[i]] = array[i]. O(n) time, O(n) space.
// (In-place alternative: follow cycles swapping elements.)
fn apply_permutation(array: &[&str], p: &[usize]) -> Vec<String> {
    let mut result = vec![String::new(); array.len()];
    for (i, &val) in array.iter().enumerate() {
        result[p[i]] = val.to_string();
    }
    result
}

fn main() {
    let array = ["a", "b", "c"];
    let p = [2usize, 1, 0];
    let res = apply_permutation(&array, &p);
    let quoted: Vec<String> = res.iter().map(|x| format!("\"{}\"", x)).collect();
    println!("[{}]", quoted.join(", "));
}
