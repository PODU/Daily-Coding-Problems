// Dominoes final state via two-pass force summation. Time: O(n), Space: O(n).
fn dominoes(s: &str) -> String {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut forces = vec![0i32; n];
    // Left to right: R force propagates rightward
    let mut f = 0i32;
    for i in 0..n {
        match s[i] {
            'R' => f = n as i32,
            'L' => f = 0,
            _   => f = (f - 1).max(0),
        }
        forces[i] += f;
    }
    // Right to left: L force propagates leftward (subtract)
    f = 0;
    for i in (0..n).rev() {
        match s[i] {
            'L' => f = n as i32,
            'R' => f = 0,
            _   => f = (f - 1).max(0),
        }
        forces[i] -= f;
    }
    forces.iter().map(|&v| if v>0 {'R'} else if v<0 {'L'} else {'.'}).collect()
}

fn main() {
    println!("{}", dominoes(".L.R....L"));
    println!("{}", dominoes("..R...L.L"));
}
