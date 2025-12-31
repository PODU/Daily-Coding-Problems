// Day 826: rsync-style file sync over a low-bandwidth link.
// Receiver signs old file's fixed-size blocks (weak rolling Adler checksum +
// strong FNV-1a hash); sender rolls a window over the new file to find matching
// blocks, emitting COPY/LITERAL tokens; receiver rebuilds.
// Time O(n) average (rolling hash), Space O(n).
use std::collections::HashMap;

const MOD: i64 = 1 << 16;
const L: usize = 4;

fn strong_hash(b: &[u8]) -> u64 {
    let mut h: u64 = 1469598103934665603;
    for &x in b {
        h ^= x as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}

fn weak_block(b: &[u8]) -> (i64, i64, i64) {
    let n = b.len();
    let mut a: i64 = 0;
    let mut s: i64 = 0;
    for k in 0..n {
        a += b[k] as i64;
        s += (n - k) as i64 * b[k] as i64;
    }
    a %= MOD;
    s %= MOD;
    (a, s, a + MOD * s)
}

enum Token {
    Copy(usize),
    Lit(Vec<u8>),
}

fn main() {
    let old_f = b"the quick brown fox jumps".to_vec();
    let new_f = b"the quick red fox leaps over".to_vec();

    // signature
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    while i < old_f.len() {
        let end = (i + L).min(old_f.len());
        blocks.push(old_f[i..end].to_vec());
        i += L;
    }
    let mut table: HashMap<i64, Vec<(u64, usize)>> = HashMap::new();
    for (idx, blk) in blocks.iter().enumerate() {
        if blk.len() == L {
            let (_, _, w) = weak_block(blk);
            table.entry(w).or_default().push((strong_hash(blk), idx));
        }
    }

    // diff
    let mut delta: Vec<Token> = Vec::new();
    let mut lit: Vec<u8> = Vec::new();
    let n = new_f.len();
    let mut i = 0usize;
    let (mut a, mut s, mut cs) = (0i64, 0i64, 0i64);
    let mut have = false;
    while i < n {
        if i + L <= n {
            if !have {
                let w = weak_block(&new_f[i..i + L]);
                a = w.0;
                s = w.1;
                cs = w.2;
                have = true;
            }
            let mut matched = false;
            if let Some(bucket) = table.get(&cs) {
                let win = &new_f[i..i + L];
                let hh = strong_hash(win);
                for &(sh, idx) in bucket {
                    if sh == hh && blocks[idx] == win {
                        if !lit.is_empty() {
                            delta.push(Token::Lit(std::mem::take(&mut lit)));
                        }
                        delta.push(Token::Copy(idx));
                        i += L;
                        have = false;
                        matched = true;
                        break;
                    }
                }
            }
            if matched {
                continue;
            }
            lit.push(new_f[i]);
            if i + L < n {
                let out = new_f[i] as i64;
                let inb = new_f[i + L] as i64;
                a = ((a - out + inb) % MOD + MOD) % MOD;
                s = ((s - L as i64 * out + a) % MOD + MOD) % MOD;
                cs = a + MOD * s;
            }
            i += 1;
        } else {
            lit.push(new_f[i]);
            i += 1;
        }
    }
    if !lit.is_empty() {
        delta.push(Token::Lit(lit));
    }

    // reconstruct
    let mut rebuilt: Vec<u8> = Vec::new();
    let mut copies = 0;
    for t in &delta {
        match t {
            Token::Copy(idx) => {
                rebuilt.extend_from_slice(&blocks[*idx]);
                copies += 1;
            }
            Token::Lit(bytes) => rebuilt.extend_from_slice(bytes),
        }
    }

    println!("{}", String::from_utf8(rebuilt.clone()).unwrap());
    println!(
        "reconstruction OK: {}",
        if rebuilt == new_f { "True" } else { "False" }
    );
    println!("blocks reused (copy tokens): {}", copies);
}
