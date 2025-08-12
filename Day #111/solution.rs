// Day 111: Anagram substrings via fixed sliding window of char counts. O(|S|).
fn find_anagrams(s: &str, w: &str) -> Vec<usize> {
    let sb = s.as_bytes();
    let wb = w.as_bytes();
    let (n, m) = (sb.len(), wb.len());
    let mut res = Vec::new();
    if m > n {
        return res;
    }
    let mut need = [0i32; 256];
    let mut win = [0i32; 256];
    for &c in wb {
        need[c as usize] += 1;
    }
    for i in 0..n {
        win[sb[i] as usize] += 1;
        if i >= m {
            win[sb[i - m] as usize] -= 1;
        }
        if i + 1 >= m && win == need {
            res.push(i + 1 - m);
        }
    }
    res
}

fn main() {
    let r: Vec<String> = find_anagrams("abxaba", "ab")
        .iter()
        .map(|x| x.to_string())
        .collect();
    println!("{}", r.join(", "));
}
