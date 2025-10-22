// rsync-style sync: split destination into fixed blocks indexed by a weak rolling Adler-32 checksum
// + strong FNV-1a hash; slide a rolling window over the source to emit COPY/LITERAL tokens (only
// diffs sent), then reconstruct new = old blocks + literals. Time O(n) (constant block size).
use std::collections::HashMap;

const M: i64 = 65521;
const BLOCK: usize = 4;

fn fnv(d: &[u8], from: usize, len: usize) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for k in from..from + len {
        h ^= d[k] as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

enum Token {
    Copy(usize),
    Lit(Vec<u8>),
}

fn main() {
    let old_s = "the quick brown fox jumps over the lazy dog";
    let new_s = "the quick BROWN fox jumps over the lazy cat";
    let old_b = old_s.as_bytes();
    let new_b = new_s.as_bytes();

    let mut idx: HashMap<u64, Vec<(usize, u64)>> = HashMap::new();
    let nb = old_b.len() / BLOCK;
    for bi in 0..nb {
        let s = bi * BLOCK;
        let (mut a, mut b) = (0i64, 0i64);
        for k in s..s + BLOCK {
            a = (a + old_b[k] as i64) % M;
            b = (b + a) % M;
        }
        let weak = (a as u64) + ((b as u64) << 16);
        idx.entry(weak).or_insert_with(Vec::new).push((bi, fnv(old_b, s, BLOCK)));
    }

    let mut tokens: Vec<Token> = Vec::new();
    let mut lit: Vec<u8> = Vec::new();
    let n = new_b.len();
    let mut pos = 0usize;
    let (mut a, mut b) = (0i64, 0i64);
    let mut have = false;
    while pos < n {
        if pos + BLOCK <= n {
            if !have {
                a = 0;
                b = 0;
                for k in pos..pos + BLOCK {
                    a = (a + new_b[k] as i64) % M;
                    b = (b + a) % M;
                }
                have = true;
            }
            let weak = (a as u64) + ((b as u64) << 16);
            let mut matched: i64 = -1;
            if let Some(cand) = idx.get(&weak) {
                let strong = fnv(new_b, pos, BLOCK);
                for &(bi, st) in cand {
                    if st == strong {
                        matched = bi as i64;
                        break;
                    }
                }
            }
            if matched >= 0 {
                if !lit.is_empty() {
                    tokens.push(Token::Lit(lit.clone()));
                    lit.clear();
                }
                tokens.push(Token::Copy(matched as usize));
                pos += BLOCK;
                have = false;
                continue;
            }
            lit.push(new_b[pos]);
            if pos + BLOCK < n {
                // roll forward one byte
                let out = new_b[pos] as i64;
                let inb = new_b[pos + BLOCK] as i64;
                a = (((a - out + inb) % M) + M) % M;
                b = (((b - (BLOCK as i64) * out + a) % M) + M) % M;
            } else {
                have = false;
            }
            pos += 1;
        } else {
            lit.push(new_b[pos]);
            pos += 1;
        }
    }
    if !lit.is_empty() {
        tokens.push(Token::Lit(lit.clone()));
    }

    let mut out: Vec<u8> = Vec::new();
    let mut mb = 0usize;
    let mut lb = 0usize;
    for t in &tokens {
        match t {
            Token::Copy(bi) => {
                let s = bi * BLOCK;
                out.extend_from_slice(&old_b[s..s + BLOCK]);
                mb += 1;
            }
            Token::Lit(v) => {
                out.extend_from_slice(v);
                lb += v.len();
            }
        }
    }
    let recon = String::from_utf8(out).unwrap();
    println!("Matched blocks: {} ({} bytes), Literal bytes: {}", mb, mb * BLOCK, lb);
    println!("Reconstructed: {}", recon);
    println!("Equals new file: {}", recon == new_s);
}
