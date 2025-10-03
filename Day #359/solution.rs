// Recover digits from scrambled letters: use unique marker letters (z,w,u,x,g) then derive the rest. O(N) time, O(1) space.
fn recover(s: &str) -> String {
    let mut c = [0i32; 26];
    for ch in s.chars() {
        if ch.is_ascii_lowercase() {
            c[(ch as u8 - b'a') as usize] += 1;
        }
    }
    let g = |ch: char| c[(ch as u8 - b'a') as usize];
    let mut d = [0i32; 10];
    d[0] = g('z');
    d[2] = g('w');
    d[4] = g('u');
    d[6] = g('x');
    d[8] = g('g');
    d[3] = g('h') - d[8];
    d[5] = g('f') - d[4];
    d[7] = g('s') - d[6];
    d[1] = g('o') - d[0] - d[2] - d[4];
    d[9] = g('i') - d[5] - d[6] - d[8];
    let mut res = String::new();
    for i in 0..10 {
        for _ in 0..d[i] {
            res.push(std::char::from_digit(i as u32, 10).unwrap());
        }
    }
    res
}

fn main() {
    println!("{}", recover("niesevehrtfeev"));
}
