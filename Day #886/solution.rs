// Edit distance (Levenshtein) via DP. Time O(n*m), Space O(min(n,m)).
fn edit_distance(a: &str, b: &str) -> usize {
    let (a, b) = if a.len() > b.len() { (b, a) } else { (a, b) };
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (n, m) = (a.len(), b.len());
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut cur = vec![0usize; n + 1];
    for j in 1..=m {
        cur[0] = j;
        for i in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            cur[i] = (prev[i] + 1).min(cur[i - 1] + 1).min(prev[i - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[n]
}

fn main() {
    println!("{}", edit_distance("kitten", "sitting"));
}
