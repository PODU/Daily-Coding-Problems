// Power set via bitmask iteration over 2^n subsets, then sorted by (size, elements).
// Time: O(n*2^n), Space: O(n*2^n) to hold all subsets.

fn power_set(s: &[i32]) -> Vec<Vec<i32>> {
    let n = s.len();
    let mut subsets: Vec<Vec<i32>> = Vec::new();
    for mask in 0..(1 << n) {
        let mut cur = Vec::new();
        for i in 0..n {
            if mask & (1 << i) != 0 {
                cur.push(s[i]);
            }
        }
        subsets.push(cur);
    }
    subsets.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    subsets
}

fn main() {
    let s = vec![1, 2, 3];
    let subsets = power_set(&s);
    let parts: Vec<String> = subsets
        .iter()
        .map(|sub| {
            let elems: Vec<String> = sub.iter().map(|v| v.to_string()).collect();
            format!("{{{}}}", elems.join(", "))
        })
        .collect();
    println!("{{{}}}", parts.join(", "));
}
