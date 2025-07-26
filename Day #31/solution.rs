// Edit Distance via DP. Time O(m*n), Space O(min(m,n)) using two rolling rows.
fn edit_distance(a: &str, b: &str) -> usize {
    let (a, b) = if a.len() < b.len() { (b, a) } else { (a, b) };
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (m, n) = (a.len(), b.len());
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut cur = vec![0usize; n + 1];
    for i in 1..=m {
        cur[0] = i;
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                cur[j] = prev[j - 1];
            } else {
                cur[j] = 1 + prev[j - 1].min(prev[j]).min(cur[j - 1]);
            }
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[n]
}

fn main() {
    println!("{}", edit_distance("kitten", "sitting"));
}
