// De Bruijn sequence B(k_alphabet, n) via the FKM/Lyndon-word recursion.
// Concatenating Lyndon words whose length divides n. Time: O(k^n). Space: O(k^n).
fn de_bruijn(c: &[char], n: usize) -> String {
    let k = c.len();
    let mut a = vec![0usize; k * n];
    let mut seq: Vec<usize> = Vec::new();

    fn db(t: usize, p: usize, n: usize, k: usize, a: &mut Vec<usize>, seq: &mut Vec<usize>) {
        if t > n {
            if n % p == 0 {
                for j in 1..=p {
                    seq.push(a[j]);
                }
            }
        } else {
            a[t] = a[t - p];
            db(t + 1, p, n, k, a, seq);
            for j in (a[t - p] + 1)..k {
                a[t] = j;
                db(t + 1, t, n, k, a, seq);
            }
        }
    }

    db(1, 1, n, k, &mut a, &mut seq);
    seq.iter().map(|&i| c[i]).collect()
}

fn main() {
    let c = ['0', '1'];
    println!("{}", de_bruijn(&c, 3)); // 00010111
}
