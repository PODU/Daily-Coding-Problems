// Day 767: Find all start indices in S that are anagrams of W.
// Sliding window of size |W| with a 26-count array. O(|S|) time, O(1) space.
fn find_anagrams(s: &str, w: &str) -> Vec<usize> {
    let s = s.as_bytes();
    let w = w.as_bytes();
    let (n, m) = (s.len(), w.len());
    let mut res = Vec::new();
    if m > n {
        return res;
    }
    let mut need = [0i32; 26];
    let mut win = [0i32; 26];
    for &c in w {
        need[(c - b'a') as usize] += 1;
    }
    for i in 0..n {
        win[(s[i] - b'a') as usize] += 1;
        if i >= m {
            win[(s[i - m] - b'a') as usize] -= 1;
        }
        if i + 1 >= m && win == need {
            res.push(i - m + 1);
        }
    }
    res
}

fn main() {
    let r = find_anagrams("abxaba", "ab");
    let parts: Vec<String> = r.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(", ")); // 0, 3, 4
}
