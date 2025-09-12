// Day 260: Reconstruct a permutation of [0..N] consistent with +/- sign array.
// Grow a list: on '+' append max+1, on '-' append min-1; shift by -min into [0..N].
// O(n) time, O(n) space.

fn reconstruct(signs: &[i32]) -> Vec<i32> {
    // signs[0] is a sentinel (None)
    let mut res = vec![0];
    let (mut cur_max, mut cur_min) = (0, 0);
    for &s in &signs[1..] {
        if s > 0 {
            cur_max += 1;
            res.push(cur_max);
        } else {
            cur_min -= 1;
            res.push(cur_min);
        }
    }
    let off = -cur_min;
    res.iter().map(|x| x + off).collect()
}

fn main() {
    let signs = vec![0, 1, 1, -1, 1]; // [None, +, +, -, +]
    let r = reconstruct(&signs);
    let s: Vec<String> = r.iter().map(|x| x.to_string()).collect();
    println!("[{}]", s.join(", "));
}
