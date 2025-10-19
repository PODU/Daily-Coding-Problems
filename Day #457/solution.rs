// Day 457: All start indices in S that are anagrams of W.
// Fixed-size sliding window of byte counts. Time O(|S|), Space O(1).
fn anagram_indices(w: &str, s: &str) -> Vec<usize> {
    let (wb, sb) = (w.as_bytes(), s.as_bytes());
    let (m, n) = (wb.len(), sb.len());
    let mut res = Vec::new();
    if m == 0 || m > n {
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
        if i + 1 >= m && need == win {
            res.push(i + 1 - m);
        }
    }
    res
}

fn main() {
    let r = anagram_indices("ab", "abxaba");
    let parts: Vec<String> = r.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(", ")); // 0, 3, 4
}
