// Day 1447: Does a secret code (6 distinct digits) exist consistent with all
// (guess, score) pairs? Brute force all 6-permutations of 0-9. Time O(P*G*6).
fn score_match(code: &[usize; 6], g: &[usize; 6]) -> usize {
    (0..6).filter(|&i| code[i] == g[i]).count()
}

fn dfs(pos: usize, code: &mut [usize; 6], used: &mut [bool; 10],
       guesses: &[([usize; 6], usize)]) -> bool {
    if pos == 6 {
        return guesses.iter().all(|(g, s)| score_match(code, g) == *s);
    }
    for d in 0..10 {
        if used[d] {
            continue;
        }
        if pos == 0 && d == 0 {
            continue; // no leading zero
        }
        used[d] = true;
        code[pos] = d;
        if dfs(pos + 1, code, used, guesses) {
            used[d] = false;
            return true;
        }
        used[d] = false;
    }
    false
}

fn consistent(guesses: &[([usize; 6], usize)]) -> bool {
    let mut code = [0usize; 6];
    let mut used = [false; 10];
    dfs(0, &mut code, &mut used, guesses)
}

fn to_digits(mut n: usize) -> [usize; 6] {
    let mut d = [0usize; 6];
    for i in (0..6).rev() {
        d[i] = n % 10;
        n /= 10;
    }
    d
}

fn main() {
    let e1 = [(to_digits(175286), 2), (to_digits(293416), 3), (to_digits(654321), 0)];
    let e2 = [(to_digits(123456), 4), (to_digits(345678), 4), (to_digits(567890), 4)];
    println!("{}", if consistent(&e1) { "True" } else { "False" }); // True
    println!("{}", if consistent(&e2) { "True" } else { "False" }); // False
}
