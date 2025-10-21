// Mastermind: brute force all 6-permutations of digits 0-9 (10P6=151200),
// keep one consistent with every guess score. Time: O(10P6 * G), Space: O(1).
fn score(secret: &[u8; 6], guess: &[u8; 6]) -> i32 {
    let mut s = 0;
    for i in 0..6 {
        if secret[i] == guess[i] {
            s += 1;
        }
    }
    s
}

fn search(secret: &mut [u8; 6], pos: usize, used: &mut [bool; 10], guesses: &[([u8; 6], i32)]) -> bool {
    if pos == 6 {
        return guesses.iter().all(|(g, sc)| score(secret, g) == *sc);
    }
    for d in 0u8..10 {
        if used[d as usize] {
            continue;
        }
        used[d as usize] = true;
        secret[pos] = d;
        if search(secret, pos + 1, used, guesses) {
            return true;
        }
        used[d as usize] = false;
    }
    false
}

fn to_code(s: &str) -> [u8; 6] {
    let mut arr = [0u8; 6];
    for (i, c) in s.bytes().enumerate() {
        arr[i] = c - b'0';
    }
    arr
}

fn consistent(guesses: &[(&str, i32)]) -> bool {
    let parsed: Vec<([u8; 6], i32)> = guesses.iter().map(|(g, s)| (to_code(g), *s)).collect();
    let mut secret = [0u8; 6];
    let mut used = [false; 10];
    search(&mut secret, 0, &mut used, &parsed)
}

fn main() {
    let ex1 = [("175286", 2), ("293416", 3), ("654321", 0)];
    let ex2 = [("123456", 4), ("345678", 4), ("567890", 4)];
    println!("{}", if consistent(&ex1) { "True" } else { "False" });
    println!("{}", if consistent(&ex2) { "True" } else { "False" });
}
