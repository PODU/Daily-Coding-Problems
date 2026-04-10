// Day 1334: Levenshtein edit distance between two strings.
// Classic DP with rolling row. O(n*m) time, O(min(n,m)) space.

fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (n, m) = (a.len(), b.len());
    let mut prev: Vec<usize> = (0..=m).collect();
    let mut cur = vec![0usize; m + 1];
    for i in 1..=n {
        cur[0] = i;
        for j in 1..=m {
            if a[i - 1] == b[j - 1] {
                cur[j] = prev[j - 1];
            } else {
                cur[j] = 1 + prev[j - 1].min(prev[j]).min(cur[j - 1]);
            }
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[m]
}

fn main() {
    println!("{}", edit_distance("kitten", "sitting")); // 3
}
