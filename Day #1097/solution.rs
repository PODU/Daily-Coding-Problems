// Day 1097: Smallest string by moving one of first k letters to the end.
// k==1 -> min rotation; k>=2 -> any permutation reachable -> sorted string.
// Time: O(N^2) for k==1, O(N log N) for k>=2. Space: O(N).
fn smallest(s: &str, k: usize) -> String {
    if k >= 2 {
        let mut v: Vec<char> = s.chars().collect();
        v.sort();
        return v.into_iter().collect();
    }
    let mut best = s.to_string();
    let mut cur = s.to_string();
    for _ in 0..s.len() {
        let first = cur.remove(0);
        cur.push(first);
        if cur < best {
            best = cur.clone();
        }
    }
    best
}

fn main() {
    println!("{}", smallest("daily", 1)); // ailyd
}
