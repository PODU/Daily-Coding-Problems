// Find all anagram start indices of W in S.
// Sliding window + 26-letter freq + match counter. Time O(|S|), Space O(1).
fn find_anagrams(w: &str, s: &str) -> Vec<usize> {
    let sb = s.as_bytes();
    let wb = w.as_bytes();
    let (n, m) = (sb.len(), wb.len());
    let mut res = Vec::new();
    if m == 0 || m > n {
        return res;
    }
    let mut need = [0i32; 26];
    let mut win = [0i32; 26];
    for &c in wb {
        need[(c - b'a') as usize] += 1;
    }
    let mut matches = 0;
    for i in 0..26 {
        if need[i] == 0 {
            matches += 1;
        }
    }
    for i in 0..n {
        let add = (sb[i] - b'a') as usize;
        win[add] += 1;
        if win[add] == need[add] {
            matches += 1;
        } else if win[add] == need[add] + 1 {
            matches -= 1;
        }
        if i >= m {
            let rem = (sb[i - m] - b'a') as usize;
            win[rem] -= 1;
            if win[rem] == need[rem] {
                matches += 1;
            } else if win[rem] == need[rem] - 1 {
                matches -= 1;
            }
        }
        if i + 1 >= m && matches == 26 {
            res.push(i + 1 - m);
        }
    }
    res
}

fn main() {
    let idx = find_anagrams("ab", "abxaba");
    let parts: Vec<String> = idx.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(", "));
}
