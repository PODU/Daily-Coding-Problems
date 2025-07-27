// Power set via bitmask 0..2^n-1, then order subsets by (size, element order).
// O(2^n * n) time, O(2^n * n) space.
fn main() {
    let s = vec![1, 2, 3];
    let n = s.len();
    let mut subsets: Vec<Vec<i32>> = Vec::new();
    for mask in 0..(1 << n) {
        let mut sub = Vec::new();
        for i in 0..n {
            if mask & (1 << i) != 0 {
                sub.push(s[i]);
            }
        }
        subsets.push(sub);
    }
    subsets.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    let parts: Vec<String> = subsets
        .iter()
        .map(|sub| {
            let elems: Vec<String> = sub.iter().map(|x| x.to_string()).collect();
            format!("{{{}}}", elems.join(", "))
        })
        .collect();
    println!("{{{}}}", parts.join(", "));
}
