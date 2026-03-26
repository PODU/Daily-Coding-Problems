// Day 1270: Find all start indices in S that are anagrams of W.
// Fixed-size sliding window with a 26-length count. O(|S|) time.
fn find_anagrams(w: &str, s: &str) -> Vec<usize> {
    let mut res = Vec::new();
    let (wb, sb) = (w.as_bytes(), s.as_bytes());
    let (m, n) = (wb.len(), sb.len());
    if m > n {
        return res;
    }
    let mut need = [0i32; 26];
    let mut win = [0i32; 26];
    for &c in wb {
        need[(c - b'a') as usize] += 1;
    }
    for i in 0..n {
        win[(sb[i] - b'a') as usize] += 1;
        if i >= m {
            win[(sb[i - m] - b'a') as usize] -= 1;
        }
        if i + 1 >= m && win == need {
            res.push(i + 1 - m);
        }
    }
    res
}

fn main() {
    let res = find_anagrams("ab", "abxaba");
    let parts: Vec<String> = res.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(", "));
}
