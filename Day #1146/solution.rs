// Day 1146: Dominoes - two-pass force accumulation.
// L->R pass adds rightward force, R->L pass adds leftward; sign decides. O(n) time, O(n) space.
fn push_dominoes(s: &str) -> String {
    let b = s.as_bytes();
    let n = b.len();
    let mut forces = vec![0i32; n];
    let mut force = 0i32;
    for i in 0..n {
        match b[i] {
            b'R' => force = n as i32,
            b'L' => force = 0,
            _ => force = (force - 1).max(0),
        }
        forces[i] += force;
    }
    force = 0;
    for i in (0..n).rev() {
        match b[i] {
            b'L' => force = n as i32,
            b'R' => force = 0,
            _ => force = (force - 1).max(0),
        }
        forces[i] -= force;
    }
    forces
        .iter()
        .map(|&f| if f > 0 { 'R' } else if f < 0 { 'L' } else { '.' })
        .collect()
}

fn main() {
    println!("{}", push_dominoes(".L.R....L")); // LL.RRRLLL
    println!("{}", push_dominoes("..R...L.L")); // ..RR.LLLL
}
