// De Bruijn B(k,n) via FKM (Lyndon word) algorithm -> lexicographically smallest. O(k^n).
fn db(t: usize, p: usize, k: usize, n: usize, a: &mut Vec<usize>, seq: &mut String) {
    if t > n {
        if n % p == 0 {
            for i in 1..=p {
                seq.push((b'0' + a[i] as u8) as char);
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

fn de_bruijn(k: usize, n: usize) -> String {
    let mut a = vec![0usize; k * n + 1];
    let mut seq = String::new();
    db(1, 1, k, n, &mut a, &mut seq);
    seq
}

fn main() {
    // C = {0,1} -> alphabet size 2, substring length 3
    println!("{}", de_bruijn(2, 3));
}
