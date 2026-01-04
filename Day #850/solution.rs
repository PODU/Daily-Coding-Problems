// Day 850: De Bruijn sequence via the FKM (Lyndon-word) algorithm.
// Lexicographically smallest cyclic sequence containing every length-n string once. O(k^n).
fn de_bruijn(k: usize, n: usize, alphabet: &str) -> String {
    let alpha: Vec<char> = alphabet.chars().collect();
    let mut a = vec![0usize; k * n];
    let mut seq: Vec<usize> = Vec::new();

    fn db(t: usize, p: usize, k: usize, n: usize, a: &mut Vec<usize>, seq: &mut Vec<usize>) {
        if t > n {
            if n % p == 0 {
                for j in 1..=p {
                    seq.push(a[j]);
                }
            }
        } else {
            a[t] = a[t - p];
            db(t + 1, p, k, n, a, seq);
            let start = a[t - p] + 1;
            for j in start..k {
                a[t] = j;
                db(t + 1, t, k, n, a, seq);
            }
        }
    }

    db(1, 1, k, n, &mut a, &mut seq);
    seq.iter().map(|&i| alpha[i]).collect()
}

fn main() {
    // C = {0,1}, length 3 => alphabet size 2, n = 3
    println!("{}", de_bruijn(2, 3, "01")); // 00010111
}
