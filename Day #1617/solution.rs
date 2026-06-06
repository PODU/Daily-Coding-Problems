// Falling dominoes: two-pass force accumulation (R adds +, L adds -, decay between).
// Sign of net force decides L/R/.; equal force stays '.'. Time O(n), Space O(n).
fn push_dominoes(d: &str) -> String {
    let b = d.as_bytes();
    let n = b.len();
    let mut force = vec![0i32; n];
    let mut f = 0i32;
    for i in 0..n {
        match b[i] {
            b'R' => f = n as i32,
            b'L' => f = 0,
            _ => f = (f - 1).max(0),
        }
        force[i] += f;
    }
    f = 0;
    for i in (0..n).rev() {
        match b[i] {
            b'L' => f = n as i32,
            b'R' => f = 0,
            _ => f = (f - 1).max(0),
        }
        force[i] -= f;
    }
    force
        .iter()
        .map(|&x| if x > 0 { 'R' } else if x < 0 { 'L' } else { '.' })
        .collect()
}

fn main() {
    println!("{}", push_dominoes(".L.R....L"));
    println!("{}", push_dominoes("..R...L.L"));
}
