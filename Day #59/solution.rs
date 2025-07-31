// Day 59: File sync over low-bandwidth link, rsync-style.
// Receiver sends per-block (weak rolling + strong) checksums of its OLD file;
// sender scans NEW file with a rolling checksum, emitting COPY(block) tokens for
// matches and LITERAL bytes otherwise -> only differing bytes cross the wire.
// Time: O(n) expected, bandwidth ~ size of the changes.
use std::collections::HashMap;

const M: i64 = 1 << 16;
const B: usize = 4;

fn fnv1a(d: &[u8], s: usize, e: usize) -> u64 {
    let mut h: u64 = 1469598103934665603;
    for i in s..e {
        h ^= d[i] as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}

fn weak_full(d: &[u8], s: usize, e: usize) -> (i64, i64) {
    let (mut a, mut b) = (0i64, 0i64);
    for i in s..e {
        a = (a + d[i] as i64) % M;
        b = (b + (e - i) as i64 * d[i] as i64) % M;
    }
    (a, b)
}

enum Token {
    Copy(usize),
    Lit(Vec<u8>),
}

// Receiver side: weak -> (strong -> index).
fn signatures(old: &[u8]) -> HashMap<i64, HashMap<u64, usize>> {
    let mut sigs: HashMap<i64, HashMap<u64, usize>> = HashMap::new();
    let n = old.len() / B;
    for i in 0..n {
        let (a, b) = weak_full(old, i * B, i * B + B);
        let weak = (b << 16) | a;
        sigs.entry(weak).or_default().insert(fnv1a(old, i * B, i * B + B), i);
    }
    sigs
}

fn diff(nw: &[u8], sigs: &HashMap<i64, HashMap<u64, usize>>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut literal: Vec<u8> = Vec::new();
    let (mut pos, n) = (0usize, nw.len());
    let (mut a, mut b) = (0i64, 0i64);
    let mut have_window = false;
    while pos + B <= n {
        if !have_window {
            let (na, nb) = weak_full(nw, pos, pos + B);
            a = na;
            b = nb;
            have_window = true;
        }
        let weak = (b << 16) | a;
        let idx = sigs.get(&weak).and_then(|m| m.get(&fnv1a(nw, pos, pos + B)).copied());
        if let Some(i) = idx {
            if !literal.is_empty() {
                tokens.push(Token::Lit(std::mem::take(&mut literal)));
            }
            tokens.push(Token::Copy(i));
            pos += B;
            have_window = false;
        } else {
            let first = nw[pos] as i64;
            literal.push(nw[pos]);
            a = ((a - first + nw[pos + B] as i64) % M + M) % M;
            b = ((b - B as i64 * first + a) % M + M) % M;
            pos += 1;
        }
    }
    literal.extend_from_slice(&nw[pos..]);
    if !literal.is_empty() {
        tokens.push(Token::Lit(literal));
    }
    tokens
}

fn apply_delta(old: &[u8], tokens: &[Token]) -> Vec<u8> {
    let mut out = Vec::new();
    for t in tokens {
        match t {
            Token::Copy(i) => out.extend_from_slice(&old[i * B..i * B + B]),
            Token::Lit(bytes) => out.extend_from_slice(bytes),
        }
    }
    out
}

fn main() {
    let old = b"the quick brown fox jumps over the lazy dog";
    let nw = b"the quick brown cat jumps over the lazy dog";
    let sigs = signatures(old);
    let tokens = diff(nw, &sigs);
    let rebuilt = apply_delta(old, &tokens);
    let mut literal_bytes = 0;
    let mut copied = 0;
    for t in &tokens {
        match t {
            Token::Copy(_) => copied += 1,
            Token::Lit(b) => literal_bytes += b.len(),
        }
    }
    println!("synced: {}", rebuilt == nw);
    println!("literal bytes sent: {} of {}", literal_bytes, nw.len());
    println!("blocks reused: {}", copied);
}
