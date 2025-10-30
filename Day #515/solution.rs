// Partition list around pivot k by splitting into two ordered lists then joining. O(n) time.
// Implemented over a Vec for a self-contained, ownership-friendly demo.
fn partition(vals: &[i64], k: i64) -> Vec<i64> {
    let mut less = Vec::new();
    let mut ge = Vec::new();
    for &v in vals {
        if v < k {
            less.push(v);
        } else {
            ge.push(v);
        }
    }
    less.extend(ge);
    less
}

fn main() {
    let out = partition(&[5, 1, 8, 0, 3], 3);
    let s: Vec<String> = out.iter().map(|x| x.to_string()).collect();
    println!("{}", s.join(" -> "));
}
