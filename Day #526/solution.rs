// k==1: smallest rotation (try all n rotations). k>=2: sorted string (any pair
// of front letters can be reordered). Time O(n^2) for k==1, O(n log n) k>=2.
fn smallest(s: &str, k: usize) -> String {
    if k >= 2 {
        let mut c: Vec<char> = s.chars().collect();
        c.sort();
        return c.into_iter().collect();
    }
    let n = s.len();
    let mut best = s.to_string();
    for i in 1..n {
        let rot = format!("{}{}", &s[i..], &s[..i]);
        if rot < best {
            best = rot;
        }
    }
    best
}

fn main() {
    println!("{}", smallest("daily", 1));
}
