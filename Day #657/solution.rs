// Power set via bitmasks; sort subsets by size then numeric order. Time O(n*2^n), Space O(2^n).
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
    subsets.sort_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)));
    let parts: Vec<String> = subsets
        .iter()
        .map(|sub| {
            let elems: Vec<String> = sub.iter().map(|x| x.to_string()).collect();
            format!("{{{}}}", elems.join(", "))
        })
        .collect();
    println!("{{{}}}", parts.join(", "));
}
