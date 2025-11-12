// Greedy rearrange: at each step pick highest remaining count != prev char, tie by smallest char.
// Feasible iff maxCount <= (n+1)/2. Time O(n*26), Space O(26).

fn rearrange(s: &str) -> Option<String> {
    let mut cnt = [0i32; 26];
    for c in s.bytes() {
        cnt[(c - b'a') as usize] += 1;
    }
    let n = s.len();
    let mut res = String::with_capacity(n);
    let mut prev: i32 = -1;
    for _ in 0..n {
        let mut best: i32 = -1;
        for i in 0..26 {
            if i as i32 == prev || cnt[i] <= 0 {
                continue;
            }
            if best == -1 || cnt[i] > cnt[best as usize] {
                best = i as i32;
            }
        }
        if best == -1 {
            return None;
        }
        res.push((b'a' + best as u8) as char);
        cnt[best as usize] -= 1;
        prev = best;
    }
    Some(res)
}

fn main() {
    match rearrange("aaabbc") {
        Some(a) => println!("{}", a),
        None => println!("None"),
    }
    match rearrange("aaab") {
        Some(b) => println!("{}", b),
        None => println!("None"),
    }
}
