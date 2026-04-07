// Move one of first k chars to the end, unlimited times.
// k==1: only rotations reachable -> smallest rotation. k>=2: any order -> sort.
// Time O(n^2) for k==1 (rotation scan), O(n log n) for k>=2.

fn smallest_string(s: &str, k: usize) -> String {
    if k >= 2 {
        let mut c: Vec<char> = s.chars().collect();
        c.sort_unstable();
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
    println!("{}", smallest_string("daily", 1)); // ailyd
}
