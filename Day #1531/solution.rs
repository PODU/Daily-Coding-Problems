// Rearrange string so no two adjacent chars equal.
// Greedy: place chars by descending frequency into even indices then odd indices.
// Time O(n + k log k), Space O(n).
fn reorganize(s: &str) -> Option<String> {
    let bytes: Vec<u8> = s.bytes().collect();
    let n = bytes.len();
    let mut cnt = [0usize; 256];
    for &b in &bytes {
        cnt[b as usize] += 1;
    }
    let maxc = *cnt.iter().max().unwrap();
    if maxc > (n + 1) / 2 {
        return None;
    }
    let mut ps: Vec<(usize, u8)> = (0..256)
        .filter(|&i| cnt[i] > 0)
        .map(|i| (cnt[i], i as u8))
        .collect();
    ps.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let mut res = vec![0u8; n];
    let mut idx = 0;
    for (c, ch) in ps {
        for _ in 0..c {
            res[idx] = ch;
            idx += 2;
            if idx >= n {
                idx = 1;
            }
        }
    }
    Some(String::from_utf8(res).unwrap())
}

fn main() {
    match reorganize("aaabbc") {
        Some(r) => println!("{}", r),
        None => println!("None"),
    }
}
