// Day 1775: Mastermind consistency. Brute-force every 6-digit code with distinct
// digits (P(10,6)=151200); a code is valid if it reproduces every guess's score.
// O(P(10,6) * G) time, O(1) extra space.
fn mk(mut num: u32, score: u32) -> ([u32; 6], u32) {
    let mut d = [0u32; 6];
    for i in (0..6).rev() {
        d[i] = num % 10;
        num /= 10;
    }
    (d, score)
}

fn rec(pos: usize, code: &mut [u32; 6], used: &mut [bool; 10], gs: &[([u32; 6], u32)]) -> bool {
    if pos == 6 {
        for (digits, score) in gs {
            let m = (0..6).filter(|&i| code[i] == digits[i]).count() as u32;
            if m != *score {
                return false;
            }
        }
        return true;
    }
    for d in 0..10u32 {
        if used[d as usize] {
            continue;
        }
        used[d as usize] = true;
        code[pos] = d;
        if rec(pos + 1, code, used, gs) {
            used[d as usize] = false;
            return true;
        }
        used[d as usize] = false;
    }
    false
}

fn consistent(gs: &[([u32; 6], u32)]) -> bool {
    let mut code = [0u32; 6];
    let mut used = [false; 10];
    rec(0, &mut code, &mut used, gs)
}

fn main() {
    let a = [mk(175286, 2), mk(293416, 3), mk(654321, 0)];
    let b = [mk(123456, 4), mk(345678, 4), mk(567890, 4)];
    println!("{}", if consistent(&a) { "True" } else { "False" });
    println!("{}", if consistent(&b) { "True" } else { "False" });
}
