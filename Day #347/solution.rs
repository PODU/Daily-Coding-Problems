// Day 347: Lexicographically smallest string by moving one of first k letters to the end.
// k==1 -> best rotation; k>=2 -> any permutation reachable, so sorted. Time O(N^2)/O(N log N).
fn smallest(s: &str, k: usize) -> String {
    if k == 1 {
        let mut best = s.to_string();
        for i in 1..s.len() {
            let rot = format!("{}{}", &s[i..], &s[..i]);
            if rot < best { best = rot; }
        }
        return best;
    }
    let mut c: Vec<char> = s.chars().collect();
    c.sort();
    c.into_iter().collect()
}

fn main() {
    println!("{}", smallest("daily", 1));
}
