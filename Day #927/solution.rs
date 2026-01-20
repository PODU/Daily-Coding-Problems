// Rotate list right by k using index math on a Vec (O(1) extra besides output).
// Time O(n), Space O(1) auxiliary.
fn rotate_right(vals: &[i32], k: usize) -> Vec<i32> {
    let l = vals.len();
    if l == 0 {
        return Vec::new();
    }
    let shift = k % l;
    if shift == 0 {
        return vals.to_vec();
    }
    let split = l - shift;
    let mut out = Vec::with_capacity(l);
    out.extend_from_slice(&vals[split..]);
    out.extend_from_slice(&vals[..split]);
    out
}

fn to_str(vals: &[i32]) -> String {
    vals.iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" -> ")
}

fn main() {
    let vals = [1, 2, 3, 4, 5];
    println!("{}", to_str(&rotate_right(&vals, 3)));
}
