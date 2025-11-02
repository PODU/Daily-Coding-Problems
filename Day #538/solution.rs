// De Bruijn sequence via FKM (Lyndon-word/necklace) algorithm: emit Lyndon words whose
// length divides n, in order, giving lexicographically smallest sequence. Time O(k^n).

fn de_bruijn(k: usize, n: usize) -> String {
    let mut a = vec![0usize; k * n + 1];
    let mut seq = String::new();

    fn db(t: usize, p: usize, k: usize, n: usize, a: &mut Vec<usize>, seq: &mut String) {
        if t > n {
            if n % p == 0 {
                for j in 1..=p {
                    seq.push((b'0' + a[j] as u8) as char);
                }
            }
        } else {
            a[t] = a[t - p];
            db(t + 1, p, k, n, a, seq);
            for j in (a[t - p] + 1)..k {
                a[t] = j;
                db(t + 1, t, k, n, a, seq);
            }
        }
    }

    db(1, 1, k, n, &mut a, &mut seq);
    seq
}

fn main() {
    println!("{}", de_bruijn(2, 3)); // 00010111
}
