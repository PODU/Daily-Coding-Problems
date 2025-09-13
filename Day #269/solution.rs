// Day 269: Push dominoes simulation via force/two-pointer scan.
// Left-to-right add +force from R, right-to-left add -force from L, sign decides. Time O(n), Space O(n).

fn push_dominoes(s: &str) -> String {
    let b = s.as_bytes();
    let n = b.len();
    let mut force = vec![0i64; n];
    let mut f = 0i64;
    for i in 0..n {                       // rightward push
        match b[i] {
            b'R' => f = n as i64,
            b'L' => f = 0,
            _ => f = (f - 1).max(0),
        }
        force[i] += f;
    }
    f = 0;
    for i in (0..n).rev() {               // leftward push
        match b[i] {
            b'L' => f = n as i64,
            b'R' => f = 0,
            _ => f = (f - 1).max(0),
        }
        force[i] -= f;
    }
    force
        .iter()
        .map(|&v| if v > 0 { 'R' } else if v < 0 { 'L' } else { '.' })
        .collect()
}

fn main() {
    println!("{}", push_dominoes(".L.R....L"));
    println!("{}", push_dominoes("..R...L.L"));
}
