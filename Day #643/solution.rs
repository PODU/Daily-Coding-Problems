// Day 643: Longest common subsequence of three strings.
// Approach: 3D DP over prefix lengths of a, b, c.
// Time: O(|a|*|b|*|c|), Space: O(|b|*|c|) (two rolling layers).
fn lcs3(a: &str, b: &str, c: &str) -> usize {
    let a: Vec<u8> = a.bytes().collect();
    let b: Vec<u8> = b.bytes().collect();
    let c: Vec<u8> = c.bytes().collect();
    let (bn, cn) = (b.len(), c.len());
    let mut prev = vec![vec![0usize; cn + 1]; bn + 1];
    for i in 1..=a.len() {
        let mut cur = vec![vec![0usize; cn + 1]; bn + 1];
        for j in 1..=bn {
            for k in 1..=cn {
                if a[i - 1] == b[j - 1] && b[j - 1] == c[k - 1] {
                    cur[j][k] = prev[j - 1][k - 1] + 1;
                } else {
                    cur[j][k] = prev[j][k].max(cur[j - 1][k]).max(cur[j][k - 1]);
                }
            }
        }
        prev = cur;
    }
    prev[bn][cn]
}

fn main() {
    println!("{}", lcs3("epidemiologist", "refrigeration",
        "supercalifragilisticexpialodocious")); // 5
}
