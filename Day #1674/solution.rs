// Day 1674: Low-bandwidth file sync (rsync algorithm).
// Receiver sends per-block weak(rolling)+strong checksums; sender emits COPY/LITERAL
// tokens using a rolling hash to find matches. Time O(n) amortized, transfers only diffs.
use std::collections::HashMap;

const M: i64 = 1 << 16;

fn weak_init(d: &[u8], i: usize, l: usize) -> (i64, i64) {
    let (mut a, mut b) = (0i64, 0i64);
    for k in i..i + l {
        a = (a + d[k] as i64) % M;
        b = (b + (l - (k - i)) as i64 * d[k] as i64) % M;
    }
    (a, b)
}
fn weak_roll(a: i64, b: i64, d: &[u8], i: usize, l: usize) -> (i64, i64) {
    let (out, inb) = (d[i] as i64, d[i + l] as i64);
    let a = ((a - out + inb) % M + M) % M;
    let b = ((b - l as i64 * out + a) % M + M) % M;
    (a, b)
}
fn strong(s: &[u8], i: usize, l: usize) -> u64 {
    let mut h: u64 = 1469598103934665603;
    for k in i..i + l {
        h ^= s[k] as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}

enum Tok {
    Copy(usize),
    Lit(Vec<u8>),
}

fn diff(old: &[u8], neu: &[u8], l: usize) -> Vec<Tok> {
    let mut table: HashMap<i64, HashMap<u64, usize>> = HashMap::new();
    let nblocks = old.len() / l;
    for idx in 0..nblocks {
        let (a, b) = weak_init(old, idx * l, l);
        let w = (b << 16) | a;
        table.entry(w).or_default().insert(strong(old, idx * l, l), idx);
    }
    let mut tokens: Vec<Tok> = Vec::new();
    let mut lit: Vec<u8> = Vec::new();
    let (mut i, n) = (0usize, neu.len());
    let (mut a, mut b) = (0i64, 0i64);
    let mut have = false;
    while i < n {
        if i + l <= n {
            if !have {
                let p = weak_init(neu, i, l);
                a = p.0;
                b = p.1;
                have = true;
            }
            let w = (b << 16) | a;
            let mut matched = false;
            if let Some(bucket) = table.get(&w) {
                if let Some(&idx) = bucket.get(&strong(neu, i, l)) {
                    if !lit.is_empty() {
                        tokens.push(Tok::Lit(std::mem::take(&mut lit)));
                    }
                    tokens.push(Tok::Copy(idx));
                    i += l;
                    have = false;
                    matched = true;
                }
            }
            if matched {
                continue;
            }
            lit.push(neu[i]);
            if i + l <= n - 1 {
                let p = weak_roll(a, b, neu, i, l);
                a = p.0;
                b = p.1;
            } else {
                have = false;
            }
            i += 1;
        } else {
            lit.push(neu[i]);
            i += 1;
        }
    }
    if !lit.is_empty() {
        tokens.push(Tok::Lit(lit));
    }
    tokens
}
fn patch(old: &[u8], tokens: &[Tok], l: usize) -> Vec<u8> {
    let mut out = Vec::new();
    for t in tokens {
        match t {
            Tok::Copy(idx) => out.extend_from_slice(&old[idx * l..idx * l + l]),
            Tok::Lit(v) => out.extend_from_slice(v),
        }
    }
    out
}

fn main() {
    let l = 5;
    let old = b"the quick brown fox jumps over";
    let neu = b"the quick brown cat jumps over";
    let tokens = diff(old, neu, l);
    let rebuilt = patch(old, &tokens, l);
    let lit_bytes: usize = tokens
        .iter()
        .map(|t| if let Tok::Lit(v) = t { v.len() } else { 0 })
        .sum();
    println!("{}", rebuilt == neu); // true
    println!("{} of {}", lit_bytes, neu.len());
}
