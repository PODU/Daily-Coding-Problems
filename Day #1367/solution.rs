// Cryptarithmetic solver via backtracking over letter->digit assignments.
// Time O(10!/(10-L)!) worst with pruning, Space O(L). L = #distinct letters.
use std::collections::{HashMap, HashSet};

struct Puzzle {
    a: Vec<char>,
    b: Vec<char>,
    c: Vec<char>,
    letters: Vec<char>,
    leading: HashSet<char>,
    assign: HashMap<char, i64>,
    used: [bool; 10],
}

impl Puzzle {
    fn val(&self, w: &[char]) -> i64 {
        w.iter().fold(0i64, |v, ch| v * 10 + self.assign[ch])
    }
    fn bt(&mut self, i: usize) -> bool {
        if i == self.letters.len() {
            return self.val(&self.a) + self.val(&self.b) == self.val(&self.c);
        }
        let ch = self.letters[i];
        for d in 0..10 {
            if self.used[d] || (d == 0 && self.leading.contains(&ch)) {
                continue;
            }
            self.used[d] = true;
            self.assign.insert(ch, d as i64);
            if self.bt(i + 1) {
                return true;
            }
            self.used[d] = false;
        }
        false
    }
}

fn main() {
    let a: Vec<char> = "SEND".chars().collect();
    let b: Vec<char> = "MORE".chars().collect();
    let c: Vec<char> = "MONEY".chars().collect();
    let mut letters = Vec::new();
    let mut seen = HashSet::new();
    let mut leading = HashSet::new();
    for w in [&a, &b, &c] {
        leading.insert(w[0]);
        for &ch in w.iter() {
            if seen.insert(ch) {
                letters.push(ch);
            }
        }
    }
    let mut p = Puzzle { a, b, c, letters: letters.clone(), leading, assign: HashMap::new(), used: [false; 10] };
    if p.bt(0) {
        let parts: Vec<String> = letters.iter().map(|ch| format!("'{}': {}", ch, p.assign[ch])).collect();
        println!("{{{}}}", parts.join(", "));
    } else {
        println!("No solution");
    }
}
