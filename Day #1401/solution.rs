// rsync-style delta sync: index fixed blocks of the old file by a rolling
// (Adler-like) weak hash + exact-block strong check; slide a rolling window over
// the new file emitting COPY(block) or literal bytes. Time O(N) avg, Space O(old/B).
use std::collections::HashMap;

const B: usize = 4;

fn weak(s: &[u8], off: usize, len: usize) -> (i32, i32) {
    let (mut a, mut b) = (0i32, 0i32);
    for i in 0..len {
        let c = s[off + i] as i32;
        a = (a + c) & 0xFFFF;
        b = (b + (len as i32 - i as i32) * c) & 0xFFFF;
    }
    (a, b)
}

enum Tok {
    Copy(usize),
    Lit(u8),
}

fn make_delta(old: &[u8], nw: &[u8]) -> Vec<Tok> {
    let mut table: HashMap<i32, Vec<(usize, &[u8])>> = HashMap::new();
    let mut idx = 0;
    while idx + B <= old.len() {
        let (a, b) = weak(old, idx, B);
        table.entry((b << 16) | a).or_default().push((idx, &old[idx..idx + B]));
        idx += B;
    }
    let mut delta = Vec::new();
    let n = nw.len();
    let (mut i, mut a, mut b) = (0usize, 0i32, 0i32);
    if n >= B {
        let p = weak(nw, 0, B);
        a = p.0;
        b = p.1;
    }
    while i < n {
        if i + B <= n {
            let h = (b << 16) | a;
            let mut matched: Option<usize> = None;
            if let Some(cand) = table.get(&h) {
                let win = &nw[i..i + B];
                for &(bi, blk) in cand {
                    if blk == win {
                        matched = Some(bi);
                        break;
                    }
                }
            }
            if let Some(bi) = matched {
                delta.push(Tok::Copy(bi));
                i += B;
                if i + B <= n {
                    let p = weak(nw, i, B);
                    a = p.0;
                    b = p.1;
                }
                continue;
            }
        }
        delta.push(Tok::Lit(nw[i]));
        if i + B < n {
            let out = nw[i] as i32;
            let inb = nw[i + B] as i32;
            a = (a - out + inb) & 0xFFFF;
            b = (b - B as i32 * out + a) & 0xFFFF;
        }
        i += 1;
    }
    delta
}

fn rebuild(old: &[u8], delta: &[Tok]) -> Vec<u8> {
    let mut out = Vec::new();
    for t in delta {
        match t {
            Tok::Copy(idx) => out.extend_from_slice(&old[*idx..*idx + B]),
            Tok::Lit(c) => out.push(*c),
        }
    }
    out
}

fn main() {
    let old = b"the quick brown fox jumps over the lazy dog";
    let nw = b"the quick brown cat jumps over the lazy dog";
    let delta = make_delta(old, nw);
    let mut copies = 0;
    let mut lits = 0;
    for t in &delta {
        match t {
            Tok::Copy(_) => copies += 1,
            Tok::Lit(_) => lits += 1,
        }
    }
    let rb = rebuild(old, &delta);
    println!("Reconstructed: {}", String::from_utf8_lossy(&rb));
    println!("Match: {}", rb == nw.to_vec());
    println!("copied blocks: {} literal bytes: {}", copies, lits);
}
